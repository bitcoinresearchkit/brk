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
        PerBlockCumulativeWithSums, FiatPerBlockCumulativeWithSums,
        FiatPerBlockWithDeltas, Identity, LazyPerBlock, PriceWithRatioPerBlock,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct RealizedSoprMinimal<M: StorageMode = Rw> {
    pub value_created: PerBlockCumulativeWithSums<Cents, Cents, M>,
    pub value_destroyed: PerBlockCumulativeWithSums<Cents, Cents, M>,
}

#[derive(Traversable)]
pub struct RealizedMinimal<M: StorageMode = Rw> {
    pub cap: FiatPerBlockWithDeltas<Cents, CentsSigned, BasisPointsSigned32, M>,
    pub profit: FiatPerBlockCumulativeWithSums<Cents, M>,
    pub loss: FiatPerBlockCumulativeWithSums<Cents, M>,
    pub price: PriceWithRatioPerBlock<M>,
    pub mvrv: LazyPerBlock<StoredF32>,

    pub sopr: RealizedSoprMinimal<M>,
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
            sopr: RealizedSoprMinimal {
                value_created: cfg.import("value_created", v1)?,
                value_destroyed: cfg.import("value_destroyed", v1)?,
            },
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.cap
            .cents
            .height
            .len()
            .min(self.profit.base.cents.height.len())
            .min(self.loss.base.cents.height.len())
            .min(self.sopr.value_created.base.height.len())
            .min(self.sopr.value_destroyed.base.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &CohortState<impl RealizedOps, impl CostBasisOps>) -> Result<()> {
        self.cap.cents.height.truncate_push(height, state.realized.cap())?;
        self.profit.base.cents.height.truncate_push(height, state.realized.profit())?;
        self.loss.base.cents.height.truncate_push(height, state.realized.loss())?;
        self.sopr
            .value_created
            .base
            .height
            .truncate_push(height, state.realized.value_created())?;
        self.sopr
            .value_destroyed
            .base
            .height
            .truncate_push(height, state.realized.value_destroyed())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.cap.cents.height as &mut dyn AnyStoredVec,
            &mut self.profit.base.cents.height,
            &mut self.loss.base.cents.height,
            &mut self.sopr.value_created.base.height,
            &mut self.sopr.value_destroyed.base.height,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; cap.cents.height);
        sum_others!(self, starting_indexes, others, exit; profit.base.cents.height);
        sum_others!(self, starting_indexes, others, exit; loss.base.cents.height);
        sum_others!(self, starting_indexes, others, exit; sopr.value_created.base.height);
        sum_others!(self, starting_indexes, others, exit; sopr.value_destroyed.base.height);
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.profit.compute_rest(starting_indexes.height, exit)?;
        self.loss.compute_rest(starting_indexes.height, exit)?;
        self.sopr
            .value_created
            .compute_rest(starting_indexes.height, exit)?;
        self.sopr
            .value_destroyed
            .compute_rest(starting_indexes.height, exit)?;
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
