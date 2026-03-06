use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, BasisPointsSigned32, Bitcoin, Cents, CentsSigned, Dollars, Height, Indexes,
    Sats, StoredF32, StoredF64, Version,
};
use vecdb::{
    AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    blocks,
    distribution::state::RealizedOps,
    internal::{
        CentsUnsignedToDollars, ComputedFromHeight, ComputedFromHeightCumulative,
        ComputedFromHeightRatio, FiatFromHeight, Identity, LazyFromHeight,
        NegCentsUnsignedToDollars, PercentFromHeight, Price, RatioCents64, RatioCentsBp32,
        RatioCentsSignedCentsBps32, RollingEmas1w1m, RollingWindows,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct RealizedCore<M: StorageMode = Rw> {
    pub realized_cap_cents: ComputedFromHeight<Cents, M>,
    pub realized_profit: ComputedFromHeightCumulative<Cents, M>,
    pub realized_loss: ComputedFromHeightCumulative<Cents, M>,

    pub realized_cap: LazyFromHeight<Dollars, Cents>,
    pub realized_price: Price<ComputedFromHeight<Cents, M>>,
    pub realized_price_ratio: ComputedFromHeightRatio<M>,
    pub realized_cap_change_1m: ComputedFromHeight<CentsSigned, M>,

    pub mvrv: LazyFromHeight<StoredF32>,

    pub neg_realized_loss: LazyFromHeight<Dollars, Cents>,
    pub net_realized_pnl: ComputedFromHeightCumulative<CentsSigned, M>,
    pub net_realized_pnl_ema_1w: ComputedFromHeight<CentsSigned, M>,
    pub gross_pnl: FiatFromHeight<Cents, M>,

    pub realized_profit_ema_1w: ComputedFromHeight<Cents, M>,
    pub realized_loss_ema_1w: ComputedFromHeight<Cents, M>,

    pub realized_profit_rel_to_realized_cap: PercentFromHeight<BasisPoints32, M>,
    pub realized_loss_rel_to_realized_cap: PercentFromHeight<BasisPoints32, M>,
    pub net_realized_pnl_rel_to_realized_cap: PercentFromHeight<BasisPointsSigned32, M>,

    pub value_created: ComputedFromHeight<Cents, M>,
    pub value_destroyed: ComputedFromHeight<Cents, M>,
    pub value_created_sum: RollingWindows<Cents, M>,
    pub value_destroyed_sum: RollingWindows<Cents, M>,
    pub sopr: RollingWindows<StoredF64, M>,
    pub sopr_24h_ema: RollingEmas1w1m<StoredF64, M>,
}

impl RealizedCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;

        let realized_cap_cents = cfg.import_computed("realized_cap_cents", v0)?;
        let realized_cap = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("realized_cap"),
            cfg.version,
            realized_cap_cents.height.read_only_boxed_clone(),
            &realized_cap_cents,
        );

        let realized_profit = cfg.import_cumulative("realized_profit", v0)?;
        let realized_profit_ema_1w = cfg.import_computed("realized_profit_ema_1w", v0)?;
        let realized_loss = cfg.import_cumulative("realized_loss", v0)?;
        let realized_loss_ema_1w = cfg.import_computed("realized_loss_ema_1w", v0)?;

        let neg_realized_loss = LazyFromHeight::from_height_source::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_realized_loss"),
            cfg.version + Version::ONE,
            realized_loss.height.read_only_boxed_clone(),
            cfg.indexes,
        );

        let net_realized_pnl = cfg.import_cumulative("net_realized_pnl", v0)?;
        let net_realized_pnl_ema_1w = cfg.import_computed("net_realized_pnl_ema_1w", v0)?;
        let gross_pnl = cfg.import_fiat("realized_gross_pnl", v0)?;

        let realized_profit_rel_to_realized_cap =
            cfg.import_percent_bp32("realized_profit_rel_to_realized_cap", Version::new(2))?;
        let realized_loss_rel_to_realized_cap =
            cfg.import_percent_bp32("realized_loss_rel_to_realized_cap", Version::new(2))?;
        let net_realized_pnl_rel_to_realized_cap =
            cfg.import_percent_bps32("net_realized_pnl_rel_to_realized_cap", Version::new(2))?;

        let realized_price = cfg.import_price("realized_price", v1)?;

        let realized_price_ratio = cfg.import_ratio("realized_price", v1)?;
        let mvrv = LazyFromHeight::from_lazy::<Identity<StoredF32>, BasisPoints32>(
            &cfg.name("mvrv"),
            cfg.version,
            &realized_price_ratio.ratio,
        );

        let value_created = cfg.import_computed("value_created", v0)?;
        let value_destroyed = cfg.import_computed("value_destroyed", v0)?;
        let value_created_sum = cfg.import_rolling("value_created", v1)?;
        let value_destroyed_sum = cfg.import_rolling("value_destroyed", v1)?;
        let sopr = cfg.import_rolling("sopr", v1)?;
        let sopr_24h_ema = cfg.import_emas_1w_1m("sopr_24h", v1)?;

        Ok(Self {
            realized_cap_cents,
            realized_cap,
            realized_price,
            realized_price_ratio,
            realized_cap_change_1m: cfg.import_computed("realized_cap_change_1m", v0)?,
            mvrv,
            realized_profit,
            realized_profit_ema_1w,
            realized_loss,
            realized_loss_ema_1w,
            neg_realized_loss,
            net_realized_pnl,
            net_realized_pnl_ema_1w,
            gross_pnl,
            realized_profit_rel_to_realized_cap,
            realized_loss_rel_to_realized_cap,
            net_realized_pnl_rel_to_realized_cap,
            value_created,
            value_destroyed,
            value_created_sum,
            value_destroyed_sum,
            sopr,
            sopr_24h_ema,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.realized_cap_cents
            .height
            .len()
            .min(self.realized_profit.height.len())
            .min(self.realized_loss.height.len())
            .min(self.value_created.height.len())
            .min(self.value_destroyed.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &impl RealizedOps) -> Result<()> {
        self.realized_cap_cents
            .height
            .truncate_push(height, state.cap())?;
        self.realized_profit
            .height
            .truncate_push(height, state.profit())?;
        self.realized_loss
            .height
            .truncate_push(height, state.loss())?;
        self.value_created
            .height
            .truncate_push(height, state.value_created())?;
        self.value_destroyed
            .height
            .truncate_push(height, state.value_destroyed())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.realized_cap_cents.height as &mut dyn AnyStoredVec,
            &mut self.realized_profit.height,
            &mut self.realized_loss.height,
            &mut self.value_created.height,
            &mut self.value_destroyed.height,
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
        sum_others!(self, starting_indexes, others, exit; value_created.height);
        sum_others!(self, starting_indexes, others, exit; value_destroyed.height);

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.realized_profit
            .compute_rest(starting_indexes.height, exit)?;
        self.realized_loss
            .compute_rest(starting_indexes.height, exit)?;

        self.net_realized_pnl
            .compute(starting_indexes.height, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    &self.realized_profit.height,
                    &self.realized_loss.height,
                    |(i, profit, loss, ..)| {
                        (
                            i,
                            CentsSigned::new(profit.inner() as i64 - loss.inner() as i64),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.gross_pnl.cents.height.compute_add(
            starting_indexes.height,
            &self.realized_profit.height,
            &self.realized_loss.height,
            exit,
        )?;

        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
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

        self.realized_cap_change_1m.height.compute_rolling_change(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.realized_cap_cents.height,
            exit,
        )?;

        self.realized_profit_ema_1w.height.compute_rolling_ema(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.realized_profit.height,
            exit,
        )?;
        self.realized_loss_ema_1w.height.compute_rolling_ema(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.realized_loss.height,
            exit,
        )?;
        self.net_realized_pnl_ema_1w.height.compute_rolling_ema(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.net_realized_pnl.height,
            exit,
        )?;

        self.realized_profit_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.realized_profit.height,
                &self.realized_cap_cents.height,
                exit,
            )?;
        self.realized_loss_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.realized_loss.height,
                &self.realized_cap_cents.height,
                exit,
            )?;
        self.net_realized_pnl_rel_to_realized_cap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.net_realized_pnl.height,
                &self.realized_cap_cents.height,
                exit,
            )?;

        // SOPR: rolling sums of stateful value_created/destroyed, then ratio, then EMAs
        let window_starts = blocks.count.window_starts();
        self.value_created_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.value_created.height,
            exit,
        )?;
        self.value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.value_destroyed.height,
            exit,
        )?;

        for ((sopr, vc), vd) in self
            .sopr
            .as_mut_array()
            .into_iter()
            .zip(self.value_created_sum.as_array())
            .zip(self.value_destroyed_sum.as_array())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &vc.height,
                &vd.height,
                exit,
            )?;
        }

        self.sopr_24h_ema.compute_from_24h(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &blocks.count.height_1m_ago,
            &self.sopr._24h.height,
            exit,
        )?;

        Ok(())
    }
}
