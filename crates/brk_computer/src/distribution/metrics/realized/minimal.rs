use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, Bitcoin, Cents, Height, Indexes, Sats, StoredF32,
    Version,
};
use vecdb::{
    AnyStoredVec, AnyVec, Exit, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    blocks,
    distribution::state::{CohortState, CostBasisOps, RealizedOps},
    internal::{
        FiatPerBlock, FiatPerBlockWithSum24h, Identity, LazyPerBlock,
        PerBlockWithSum24h, PriceWithRatioPerBlock,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct RealizedSoprMinimal<M: StorageMode = Rw> {
    pub value_created: PerBlockWithSum24h<Cents, M>,
    pub value_destroyed: PerBlockWithSum24h<Cents, M>,
}

/// Minimal realized metrics: cap (fiat), profit/loss (fiat + 24h sum),
/// price, mvrv, sopr (value_created/destroyed with 24h sums).
#[derive(Traversable)]
pub struct RealizedMinimal<M: StorageMode = Rw> {
    pub cap: FiatPerBlock<Cents, M>,
    pub profit: FiatPerBlockWithSum24h<Cents, M>,
    pub loss: FiatPerBlockWithSum24h<Cents, M>,
    pub price: PriceWithRatioPerBlock<M>,
    pub mvrv: LazyPerBlock<StoredF32>,

    pub sopr: RealizedSoprMinimal<M>,
}

impl RealizedMinimal {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        let cap: FiatPerBlock<Cents> = cfg.import("realized_cap", Version::ZERO)?;

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
            .min(self.profit.raw.cents.height.len())
            .min(self.loss.raw.cents.height.len())
            .min(self.sopr.value_created.raw.height.len())
            .min(self.sopr.value_destroyed.raw.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &CohortState<impl RealizedOps, impl CostBasisOps>) -> Result<()> {
        self.cap.cents.height.truncate_push(height, state.realized.cap())?;
        self.profit.raw.cents.height.truncate_push(height, state.realized.profit())?;
        self.loss.raw.cents.height.truncate_push(height, state.realized.loss())?;
        self.sopr
            .value_created
            .raw
            .height
            .truncate_push(height, state.realized.value_created())?;
        self.sopr
            .value_destroyed
            .raw
            .height
            .truncate_push(height, state.realized.value_destroyed())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.cap.cents.height as &mut dyn AnyStoredVec,
            &mut self.profit.raw.cents.height,
            &mut self.loss.raw.cents.height,
            &mut self.sopr.value_created.raw.height,
            &mut self.sopr.value_destroyed.raw.height,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; cap.cents.height);
        sum_others!(self, starting_indexes, others, exit; profit.raw.cents.height);
        sum_others!(self, starting_indexes, others, exit; loss.raw.cents.height);
        sum_others!(self, starting_indexes, others, exit; sopr.value_created.raw.height);
        sum_others!(self, starting_indexes, others, exit; sopr.value_destroyed.raw.height);
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.profit.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback._24h,
            &self.profit.raw.cents.height,
            exit,
        )?;
        self.loss.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback._24h,
            &self.loss.raw.cents.height,
            exit,
        )?;
        self.sopr.value_created.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback._24h,
            &self.sopr.value_created.raw.height,
            exit,
        )?;
        self.sopr.value_destroyed.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback._24h,
            &self.sopr.value_destroyed.raw.height,
            exit,
        )?;
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
