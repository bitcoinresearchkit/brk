use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, Bitcoin, Cents, Dollars, Height, Indexes, Sats, StoredF32, Version,
};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    distribution::state::RealizedOps,
    internal::{
        CentsUnsignedToDollars, ComputedFromHeight, ComputedFromHeightCumulative,
        ComputedFromHeightRatio, Identity, LazyFromHeight, Price, RollingWindow24h,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct RealizedMinimal<M: StorageMode = Rw> {
    pub realized_cap_cents: ComputedFromHeight<Cents, M>,
    pub realized_profit: ComputedFromHeightCumulative<Cents, M>,
    pub realized_loss: ComputedFromHeightCumulative<Cents, M>,
    pub realized_cap: LazyFromHeight<Dollars, Cents>,
    pub realized_price: Price<ComputedFromHeight<Cents, M>>,
    pub realized_price_ratio: ComputedFromHeightRatio<M>,
    pub mvrv: LazyFromHeight<StoredF32>,

    pub realized_profit_sum: RollingWindow24h<Cents, M>,
    pub realized_loss_sum: RollingWindow24h<Cents, M>,
}

impl RealizedMinimal {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let realized_cap_cents: ComputedFromHeight<Cents> =
            cfg.import("realized_cap_cents", Version::ZERO)?;
        let realized_cap = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("realized_cap"),
            cfg.version,
            realized_cap_cents.height.read_only_boxed_clone(),
            &realized_cap_cents,
        );

        let realized_profit = cfg.import("realized_profit", Version::ZERO)?;
        let realized_loss = cfg.import("realized_loss", Version::ZERO)?;

        let realized_price = cfg.import("realized_price", Version::ONE)?;
        let realized_price_ratio: ComputedFromHeightRatio =
            cfg.import("realized_price", Version::ONE)?;
        let mvrv = LazyFromHeight::from_lazy::<Identity<StoredF32>, BasisPoints32>(
            &cfg.name("mvrv"),
            cfg.version,
            &realized_price_ratio.ratio,
        );

        let realized_profit_sum = cfg.import("realized_profit", Version::ONE)?;
        let realized_loss_sum = cfg.import("realized_loss", Version::ONE)?;

        Ok(Self {
            realized_cap_cents,
            realized_profit,
            realized_loss,
            realized_cap,
            realized_price,
            realized_price_ratio,
            mvrv,
            realized_profit_sum,
            realized_loss_sum,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.realized_cap_cents
            .height
            .len()
            .min(self.realized_profit.height.len())
            .min(self.realized_loss.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        state: &impl RealizedOps,
    ) -> Result<()> {
        self.realized_cap_cents
            .height
            .truncate_push(height, state.cap())?;
        self.realized_profit
            .height
            .truncate_push(height, state.profit())?;
        self.realized_loss
            .height
            .truncate_push(height, state.loss())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.realized_cap_cents.height as &mut dyn AnyStoredVec,
            &mut self.realized_profit.height,
            &mut self.realized_loss.height,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; realized_cap_cents.height);
        sum_others!(self, starting_indexes, others, exit; realized_profit.height);
        sum_others!(self, starting_indexes, others, exit; realized_loss.height);
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.realized_profit
            .compute_rest(starting_indexes.height, exit)?;
        self.realized_loss
            .compute_rest(starting_indexes.height, exit)?;
        self.realized_profit_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.realized_profit.height,
            exit,
        )?;
        self.realized_loss_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.realized_loss.height,
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
        self.realized_price.cents.height.compute_transform2(
            starting_indexes.height,
            &self.realized_cap_cents.height,
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

        self.realized_price_ratio.compute_ratio(
            starting_indexes,
            &prices.price.cents.height,
            &self.realized_price.cents.height,
            exit,
        )?;

        Ok(())
    }
}
