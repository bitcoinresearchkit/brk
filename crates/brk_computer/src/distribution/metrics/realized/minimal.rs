use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, BasisPointsSigned32, Bitcoin, Cents, CentsSigned, Height, Indexes, Sats, StoredF32,
    Version,
};
use vecdb::{
    AnyStoredVec, AnyVec, Exit, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    distribution::state::{CohortState, CostBasisOps, RealizedOps},
    internal::{
        FiatPerBlockCumulativeWithSums,
        FiatPerBlockWithDeltas, Identity, LazyPerBlock, PriceWithRatioPerBlock,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct RealizedMinimal<M: StorageMode = Rw> {
    pub cap: FiatPerBlockWithDeltas<Cents, CentsSigned, BasisPointsSigned32, M>,
    pub profit: FiatPerBlockCumulativeWithSums<Cents, M>,
    pub loss: FiatPerBlockCumulativeWithSums<Cents, M>,
    pub price: PriceWithRatioPerBlock<M>,
    pub mvrv: LazyPerBlock<StoredF32>,
}

impl RealizedMinimal {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        let cap = FiatPerBlockWithDeltas::forced_import(
            cfg.db,
            &cfg.name("realized_cap"),
            cfg.version,
            v1,
            cfg.indexes,
            cfg.cached_starts,
        )?;

        let price: PriceWithRatioPerBlock = cfg.import("realized_price", v1)?;
        let mvrv = LazyPerBlock::from_lazy::<Identity<StoredF32>, BasisPoints32>(
            &cfg.name("mvrv"),
            cfg.version,
            &price.ratio,
        );

        Ok(Self {
            cap,
            profit: cfg.import("realized_profit", v1)?,
            loss: cfg.import("realized_loss", v1)?,
            price,
            mvrv,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.cap
            .cents
            .height
            .len()
            .min(self.profit.block.cents.len())
            .min(self.loss.block.cents.len())
    }

    #[inline(always)]
    pub(crate) fn push_state(&mut self, state: &CohortState<impl RealizedOps, impl CostBasisOps>) {
        self.cap.cents.height.push(state.realized.cap());
        self.profit.block.cents.push(state.realized.profit());
        self.loss.block.cents.push(state.realized.loss());
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.cap.cents.height as &mut dyn AnyStoredVec,
            &mut self.profit.block.cents,
            &mut self.loss.block.cents,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; cap.cents.height);
        sum_others!(self, starting_indexes, others, exit; profit.block.cents);
        sum_others!(self, starting_indexes, others, exit; loss.block.cents);
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.profit.compute_rest(starting_indexes.height, exit)?;
        self.loss.compute_rest(starting_indexes.height, exit)?;
        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        exit: &Exit,
    ) -> Result<()> {
        let cap = &self.cap.cents.height;
        self.price.compute_all(prices, starting_indexes, exit, |v| {
            Ok(v.compute_transform2(
                starting_indexes.height,
                cap,
                height_to_supply,
                |(i, cap_cents, supply, ..)| {
                    let cap = cap_cents.as_u128();
                    let supply_sats = Sats::from(supply).as_u128();
                    if supply_sats == 0 {
                        (i, Cents::ZERO)
                    } else {
                        (i, Cents::from(cap * Sats::ONE_BTC_U128 / supply_sats))
                    }
                },
                exit,
            )?)
        })?;

        Ok(())
    }
}
