use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, BasisPointsSigned32, Bitcoin, Cents, Height, Indexes, Sats, StoredF32,
    Version,
};
use vecdb::{
    AnyStoredVec, AnyVec, Exit, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    blocks,
    distribution::state::{CohortState, CostBasisOps, RealizedOps},
    internal::{
        ComputedPerBlock, FiatPerBlock, FiatPerBlockWithSum24h, Identity, LazyPerBlock,
        PerBlockWithSum24h, Price, RatioPerBlock,
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
/// price, mvrv, nupl, sopr (value_created/destroyed with 24h sums).
#[derive(Traversable)]
pub struct RealizedMinimal<M: StorageMode = Rw> {
    pub cap: FiatPerBlock<Cents, M>,
    pub profit: FiatPerBlockWithSum24h<Cents, M>,
    pub loss: FiatPerBlockWithSum24h<Cents, M>,
    pub price: Price<ComputedPerBlock<Cents, M>>,
    pub price_ratio: RatioPerBlock<BasisPoints32, M>,
    pub mvrv: LazyPerBlock<StoredF32>,
    pub nupl: RatioPerBlock<BasisPointsSigned32, M>,

    pub sopr: RealizedSoprMinimal<M>,
}

impl RealizedMinimal {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        let cap: FiatPerBlock<Cents> = cfg.import("realized_cap", Version::ZERO)?;

        let realized_price = cfg.import("realized_price", v1)?;
        let realized_price_ratio: RatioPerBlock = cfg.import("realized_price", v1)?;
        let mvrv = LazyPerBlock::from_lazy::<Identity<StoredF32>, BasisPoints32>(
            &cfg.name("mvrv"),
            cfg.version,
            &realized_price_ratio.ratio,
        );

        let nupl = cfg.import("nupl", v1)?;

        Ok(Self {
            cap,
            profit: cfg.import("realized_profit", v1)?,
            loss: cfg.import("realized_loss", v1)?,
            price: realized_price,
            price_ratio: realized_price_ratio,
            mvrv,
            nupl,
            sopr: RealizedSoprMinimal {
                value_created: cfg.import("value_created", v1)?,
                value_destroyed: cfg.import("value_destroyed", v1)?,
            },
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
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
            &blocks.lookback.height_24h_ago,
            &self.profit.raw.cents.height,
            exit,
        )?;
        self.loss.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.loss.raw.cents.height,
            exit,
        )?;
        self.sopr.value_created.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.sopr.value_created.raw.height,
            exit,
        )?;
        self.sopr.value_destroyed.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
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
        self.price.cents.height.compute_transform2(
            starting_indexes.height,
            &self.cap.cents.height,
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
        )?;

        self.price_ratio.compute_ratio(
            starting_indexes,
            &prices.price.cents.height,
            &self.price.cents.height,
            exit,
        )?;

        self.nupl.bps.height.compute_transform2(
            starting_indexes.height,
            &prices.price.cents.height,
            &self.price.cents.height,
            |(i, price, realized_price, ..)| {
                let p = price.as_u128();
                if p == 0 {
                    (i, BasisPointsSigned32::ZERO)
                } else {
                    let rp = realized_price.as_u128();
                    let nupl_bps = ((p as i128 - rp as i128) * 10000) / p as i128;
                    (i, BasisPointsSigned32::from(nupl_bps as i32))
                }
            },
            exit,
        )?;

        Ok(())
    }
}
