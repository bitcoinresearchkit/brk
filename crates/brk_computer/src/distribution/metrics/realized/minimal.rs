use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, Bitcoin, Cents, Dollars, Height, Indexes, Sats, StoredF32, Version,
};
use vecdb::{
    AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    blocks,
    distribution::state::RealizedOps,
    internal::{
        CentsUnsignedToDollars, ComputedPerBlock, ComputedPerBlockCumulative, Identity,
        LazyPerBlock, Price, RatioPerBlock, RollingWindow24h,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct RealizedMinimal<M: StorageMode = Rw> {
    pub cap_cents: ComputedPerBlock<Cents, M>,
    pub profit: ComputedPerBlockCumulative<Cents, M>,
    pub loss: ComputedPerBlockCumulative<Cents, M>,
    pub cap: LazyPerBlock<Dollars, Cents>,
    pub price: Price<ComputedPerBlock<Cents, M>>,
    pub price_ratio: RatioPerBlock<M>,
    pub mvrv: LazyPerBlock<StoredF32>,

    pub profit_sum: RollingWindow24h<Cents, M>,
    pub loss_sum: RollingWindow24h<Cents, M>,
}

impl RealizedMinimal {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let realized_cap_cents: ComputedPerBlock<Cents> =
            cfg.import("realized_cap_cents", Version::ZERO)?;
        let realized_cap = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("realized_cap"),
            cfg.version,
            realized_cap_cents.height.read_only_boxed_clone(),
            &realized_cap_cents,
        );

        let realized_profit = cfg.import("realized_profit", Version::ZERO)?;
        let realized_loss = cfg.import("realized_loss", Version::ZERO)?;

        let realized_price = cfg.import("realized_price", Version::ONE)?;
        let realized_price_ratio: RatioPerBlock = cfg.import("realized_price", Version::ONE)?;
        let mvrv = LazyPerBlock::from_lazy::<Identity<StoredF32>, BasisPoints32>(
            &cfg.name("mvrv"),
            cfg.version,
            &realized_price_ratio.ratio,
        );

        let realized_profit_sum = cfg.import("realized_profit", Version::ONE)?;
        let realized_loss_sum = cfg.import("realized_loss", Version::ONE)?;

        Ok(Self {
            cap_cents: realized_cap_cents,
            profit: realized_profit,
            loss: realized_loss,
            cap: realized_cap,
            price: realized_price,
            price_ratio: realized_price_ratio,
            mvrv,
            profit_sum: realized_profit_sum,
            loss_sum: realized_loss_sum,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.cap_cents
            .height
            .len()
            .min(self.profit.height.len())
            .min(self.loss.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &impl RealizedOps) -> Result<()> {
        self.cap_cents.height.truncate_push(height, state.cap())?;
        self.profit.height.truncate_push(height, state.profit())?;
        self.loss.height.truncate_push(height, state.loss())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.cap_cents.height as &mut dyn AnyStoredVec,
            &mut self.profit.height,
            &mut self.loss.height,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; cap_cents.height);
        sum_others!(self, starting_indexes, others, exit; profit.height);
        sum_others!(self, starting_indexes, others, exit; loss.height);
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.profit.compute_rest(starting_indexes.height, exit)?;
        self.loss.compute_rest(starting_indexes.height, exit)?;
        self.profit_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.profit.height,
            exit,
        )?;
        self.loss_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.loss.height,
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
            &self.cap_cents.height,
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

        Ok(())
    }
}
