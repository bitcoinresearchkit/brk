use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, Cents, CentsSats, CentsSquaredSats, Dollars, Height, StoredF32, StoredF64, Version,
};
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Exit, Ident, ImportableVec, Negate, ReadableCloneableVec,
    ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    ComputeIndexes, blocks,
    distribution::state::RealizedState,
    internal::{
        CentsUnsignedToDollars, ComputedFromHeightCumulative, ComputedFromHeightLast,
        ComputedFromHeightRatio, DollarsPlus, LazyComputedValueFromHeightCumulative, LazyFromHeightLast,
        PercentageDollarsF32, Price, Ratio64,
        StoredF32Identity, ValueEmaFromHeight,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

/// Base realized metrics (always computed).
#[derive(Traversable)]
pub struct RealizedBase<M: StorageMode = Rw> {
    // === Realized Cap ===
    pub realized_cap_cents: ComputedFromHeightLast<Cents, M>,
    pub realized_cap: LazyFromHeightLast<Dollars, Cents>,
    pub realized_price: Price<ComputedFromHeightLast<Dollars, M>>,
    pub realized_price_extra: ComputedFromHeightRatio<M>,
    pub realized_cap_30d_delta: ComputedFromHeightLast<Dollars, M>,

    // === Investor Price ===
    pub investor_price_cents: ComputedFromHeightLast<Cents, M>,
    pub investor_price: Price<LazyFromHeightLast<Dollars, Cents>>,
    pub investor_price_extra: ComputedFromHeightRatio<M>,

    // === Floor/Ceiling Price Bands ===
    pub lower_price_band: Price<ComputedFromHeightLast<Dollars, M>>,
    pub upper_price_band: Price<ComputedFromHeightLast<Dollars, M>>,

    // === Raw values for aggregation ===
    pub cap_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub investor_cap_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    // === MVRV ===
    pub mvrv: LazyFromHeightLast<StoredF32>,

    // === Realized Profit/Loss ===
    pub realized_profit: ComputedFromHeightCumulative<Dollars, M>,
    pub realized_profit_7d_ema: ComputedFromHeightLast<Dollars, M>,
    pub realized_loss: ComputedFromHeightCumulative<Dollars, M>,
    pub realized_loss_7d_ema: ComputedFromHeightLast<Dollars, M>,
    pub neg_realized_loss: LazyFromHeightLast<Dollars>,
    pub net_realized_pnl: ComputedFromHeightCumulative<Dollars, M>,
    pub net_realized_pnl_7d_ema: ComputedFromHeightLast<Dollars, M>,
    pub realized_value: ComputedFromHeightLast<Dollars, M>,

    // === Realized vs Realized Cap Ratios ===
    pub realized_profit_rel_to_realized_cap: ComputedFromHeightLast<StoredF32, M>,
    pub realized_loss_rel_to_realized_cap: ComputedFromHeightLast<StoredF32, M>,
    pub net_realized_pnl_rel_to_realized_cap: ComputedFromHeightLast<StoredF32, M>,

    // === Total Realized PnL ===
    pub total_realized_pnl: LazyFromHeightLast<Dollars>,

    // === Value Created/Destroyed Splits (stored) ===
    pub profit_value_created: ComputedFromHeightLast<Dollars, M>,
    pub profit_value_destroyed: ComputedFromHeightLast<Dollars, M>,
    pub loss_value_created: ComputedFromHeightLast<Dollars, M>,
    pub loss_value_destroyed: ComputedFromHeightLast<Dollars, M>,

    // === Value Created/Destroyed Totals ===
    pub value_created: ComputedFromHeightLast<Dollars, M>,
    pub value_destroyed: ComputedFromHeightLast<Dollars, M>,

    // === Capitulation/Profit Flow (lazy aliases) ===
    pub capitulation_flow: LazyFromHeightLast<Dollars>,
    pub profit_flow: LazyFromHeightLast<Dollars>,

    // === Value Created/Destroyed Rolling Sums ===
    pub value_created_24h: ComputedFromHeightLast<Dollars, M>,
    pub value_created_7d: ComputedFromHeightLast<Dollars, M>,
    pub value_created_30d: ComputedFromHeightLast<Dollars, M>,
    pub value_created_1y: ComputedFromHeightLast<Dollars, M>,
    pub value_destroyed_24h: ComputedFromHeightLast<Dollars, M>,
    pub value_destroyed_7d: ComputedFromHeightLast<Dollars, M>,
    pub value_destroyed_30d: ComputedFromHeightLast<Dollars, M>,
    pub value_destroyed_1y: ComputedFromHeightLast<Dollars, M>,

    // === SOPR (rolling window ratios) ===
    pub sopr: LazyFromHeightLast<StoredF64>,
    pub sopr_24h: ComputedFromHeightLast<StoredF64, M>,
    pub sopr_7d: ComputedFromHeightLast<StoredF64, M>,
    pub sopr_30d: ComputedFromHeightLast<StoredF64, M>,
    pub sopr_1y: ComputedFromHeightLast<StoredF64, M>,
    pub sopr_24h_7d_ema: ComputedFromHeightLast<StoredF64, M>,
    pub sopr_7d_ema: LazyFromHeightLast<StoredF64>,
    pub sopr_24h_30d_ema: ComputedFromHeightLast<StoredF64, M>,
    pub sopr_30d_ema: LazyFromHeightLast<StoredF64>,

    // === Sell Side Risk Rolling Sum Intermediates ===
    pub realized_value_24h: ComputedFromHeightLast<Dollars, M>,
    pub realized_value_7d: ComputedFromHeightLast<Dollars, M>,
    pub realized_value_30d: ComputedFromHeightLast<Dollars, M>,
    pub realized_value_1y: ComputedFromHeightLast<Dollars, M>,

    // === Sell Side Risk (rolling window ratios) ===
    pub sell_side_risk_ratio: LazyFromHeightLast<StoredF32>,
    pub sell_side_risk_ratio_24h: ComputedFromHeightLast<StoredF32, M>,
    pub sell_side_risk_ratio_7d: ComputedFromHeightLast<StoredF32, M>,
    pub sell_side_risk_ratio_30d: ComputedFromHeightLast<StoredF32, M>,
    pub sell_side_risk_ratio_1y: ComputedFromHeightLast<StoredF32, M>,
    pub sell_side_risk_ratio_24h_7d_ema: ComputedFromHeightLast<StoredF32, M>,
    pub sell_side_risk_ratio_7d_ema: LazyFromHeightLast<StoredF32>,
    pub sell_side_risk_ratio_24h_30d_ema: ComputedFromHeightLast<StoredF32, M>,
    pub sell_side_risk_ratio_30d_ema: LazyFromHeightLast<StoredF32>,

    // === Net Realized PnL Deltas ===
    pub net_realized_pnl_cumulative_30d_delta: ComputedFromHeightLast<Dollars, M>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
        ComputedFromHeightLast<StoredF32, M>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
        ComputedFromHeightLast<StoredF32, M>,

    // === Peak Regret ===
    pub peak_regret: ComputedFromHeightCumulative<Dollars, M>,
    pub peak_regret_rel_to_realized_cap: ComputedFromHeightLast<StoredF32, M>,

    // === Sent in Profit/Loss ===
    pub sent_in_profit: LazyComputedValueFromHeightCumulative<M>,
    pub sent_in_profit_14d_ema: ValueEmaFromHeight<M>,
    pub sent_in_loss: LazyComputedValueFromHeightCumulative<M>,
    pub sent_in_loss_14d_ema: ValueEmaFromHeight<M>,
}

impl RealizedBase {
    /// Import realized base metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);
        let v3 = Version::new(3);
            // Import combined types using forced_import which handles height + derived
        let realized_cap_cents = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("realized_cap_cents"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_cap = LazyFromHeightLast::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("realized_cap"),
            cfg.version,
            realized_cap_cents.height.read_only_boxed_clone(),
            &realized_cap_cents,
        );

        let realized_profit = ComputedFromHeightCumulative::forced_import(
            cfg.db,
            &cfg.name("realized_profit"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_profit_7d_ema = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("realized_profit_7d_ema"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_loss = ComputedFromHeightCumulative::forced_import(
            cfg.db,
            &cfg.name("realized_loss"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_loss_7d_ema = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("realized_loss_7d_ema"),
            cfg.version,
            cfg.indexes,
        )?;

        let neg_realized_loss = LazyFromHeightLast::from_height_source::<Negate>(
            &cfg.name("neg_realized_loss"),
            cfg.version + v1,
            realized_loss.height.read_only_boxed_clone(),
            cfg.indexes,
        );

        let net_realized_pnl = ComputedFromHeightCumulative::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl"),
            cfg.version,
            cfg.indexes,
        )?;

        let net_realized_pnl_7d_ema = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl_7d_ema"),
            cfg.version,
            cfg.indexes,
        )?;

        let peak_regret = ComputedFromHeightCumulative::forced_import(
            cfg.db,
            &cfg.name("realized_peak_regret"),
            cfg.version + v2,
            cfg.indexes,
        )?;

        let realized_value = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("realized_value"),
            cfg.version,
            cfg.indexes,
        )?;

        let total_realized_pnl = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("total_realized_pnl"),
            cfg.version + v1,
            realized_value.height.read_only_boxed_clone(),
            &realized_value,
        );

        let realized_profit_rel_to_realized_cap = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("realized_profit_rel_to_realized_cap"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        let realized_loss_rel_to_realized_cap = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("realized_loss_rel_to_realized_cap"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        let net_realized_pnl_rel_to_realized_cap = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl_rel_to_realized_cap"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        let realized_price = Price::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        let investor_price_cents = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("investor_price_cents"),
            cfg.version,
            cfg.indexes,
        )?;

        let investor_price = Price::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("investor_price"),
            cfg.version,
            &investor_price_cents,
        );

        let investor_price_extra = ComputedFromHeightRatio::forced_import(
            cfg.db,
            &cfg.name("investor_price"),
            cfg.version,
            cfg.indexes,
        )?;

        let lower_price_band = Price::forced_import(
            cfg.db,
            &cfg.name("lower_price_band"),
            cfg.version,
            cfg.indexes,
        )?;

        let upper_price_band = Price::forced_import(
            cfg.db,
            &cfg.name("upper_price_band"),
            cfg.version,
            cfg.indexes,
        )?;

        let cap_raw = BytesVec::forced_import(cfg.db, &cfg.name("cap_raw"), cfg.version)?;
        let investor_cap_raw =
            BytesVec::forced_import(cfg.db, &cfg.name("investor_cap_raw"), cfg.version)?;

        let profit_value_created = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("profit_value_created"),
            cfg.version,
            cfg.indexes,
        )?;
        let profit_value_destroyed = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("profit_value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;
        let loss_value_created = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("loss_value_created"),
            cfg.version,
            cfg.indexes,
        )?;
        let loss_value_destroyed = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("loss_value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;

        let value_created = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("value_created"),
            cfg.version,
            cfg.indexes,
        )?;
        let value_destroyed = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;

        let capitulation_flow = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("capitulation_flow"),
            cfg.version,
            loss_value_destroyed.height.read_only_boxed_clone(),
            &loss_value_destroyed,
        );
        let profit_flow = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("profit_flow"),
            cfg.version,
            profit_value_destroyed.height.read_only_boxed_clone(),
            &profit_value_destroyed,
        );

        let realized_price_extra = ComputedFromHeightRatio::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        let mvrv = LazyFromHeightLast::from_computed::<StoredF32Identity>(
            &cfg.name("mvrv"),
            cfg.version,
            realized_price_extra.ratio.height.read_only_boxed_clone(),
            &realized_price_extra.ratio,
        );

        // === Rolling sum intermediates ===
        macro_rules! import_rolling {
            ($name:expr) => {
                ComputedFromHeightLast::forced_import(
                    cfg.db,
                    &cfg.name($name),
                    cfg.version + v1,
                    cfg.indexes,
                )?
            };
        }

        let value_created_24h = import_rolling!("value_created_24h");
        let value_created_7d = import_rolling!("value_created_7d");
        let value_created_30d = import_rolling!("value_created_30d");
        let value_created_1y = import_rolling!("value_created_1y");
        let value_destroyed_24h = import_rolling!("value_destroyed_24h");
        let value_destroyed_7d = import_rolling!("value_destroyed_7d");
        let value_destroyed_30d = import_rolling!("value_destroyed_30d");
        let value_destroyed_1y = import_rolling!("value_destroyed_1y");

        let realized_value_24h = import_rolling!("realized_value_24h");
        let realized_value_7d = import_rolling!("realized_value_7d");
        let realized_value_30d = import_rolling!("realized_value_30d");
        let realized_value_1y = import_rolling!("realized_value_1y");

        // === Rolling window stored ratios ===
        let sopr_24h = import_rolling!("sopr_24h");
        let sopr_7d = import_rolling!("sopr_7d");
        let sopr_30d = import_rolling!("sopr_30d");
        let sopr_1y = import_rolling!("sopr_1y");
        let sopr = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sopr"),
            cfg.version + v1,
            sopr_24h.height.read_only_boxed_clone(),
            &sopr_24h,
        );

        let sell_side_risk_ratio_24h = import_rolling!("sell_side_risk_ratio_24h");
        let sell_side_risk_ratio_7d = import_rolling!("sell_side_risk_ratio_7d");
        let sell_side_risk_ratio_30d = import_rolling!("sell_side_risk_ratio_30d");
        let sell_side_risk_ratio_1y = import_rolling!("sell_side_risk_ratio_1y");
        let sell_side_risk_ratio = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sell_side_risk_ratio"),
            cfg.version + v1,
            sell_side_risk_ratio_24h.height.read_only_boxed_clone(),
            &sell_side_risk_ratio_24h,
        );

        // === EMA imports + identity aliases ===
        let sopr_24h_7d_ema = import_rolling!("sopr_24h_7d_ema");
        let sopr_7d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sopr_7d_ema"),
            cfg.version + v1,
            sopr_24h_7d_ema.height.read_only_boxed_clone(),
            &sopr_24h_7d_ema,
        );
        let sopr_24h_30d_ema = import_rolling!("sopr_24h_30d_ema");
        let sopr_30d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sopr_30d_ema"),
            cfg.version + v1,
            sopr_24h_30d_ema.height.read_only_boxed_clone(),
            &sopr_24h_30d_ema,
        );

        let sell_side_risk_ratio_24h_7d_ema = import_rolling!("sell_side_risk_ratio_24h_7d_ema");
        let sell_side_risk_ratio_7d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sell_side_risk_ratio_7d_ema"),
            cfg.version + v1,
            sell_side_risk_ratio_24h_7d_ema
                .height
                .read_only_boxed_clone(),
            &sell_side_risk_ratio_24h_7d_ema,
        );
        let sell_side_risk_ratio_24h_30d_ema = import_rolling!("sell_side_risk_ratio_24h_30d_ema");
        let sell_side_risk_ratio_30d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sell_side_risk_ratio_30d_ema"),
            cfg.version + v1,
            sell_side_risk_ratio_24h_30d_ema
                .height
                .read_only_boxed_clone(),
            &sell_side_risk_ratio_24h_30d_ema,
        );

        let peak_regret_rel_to_realized_cap = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("peak_regret_rel_to_realized_cap"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        Ok(Self {
            realized_cap_cents,
            realized_cap,
            realized_price,
            realized_price_extra,
            realized_cap_30d_delta: ComputedFromHeightLast::forced_import(
                cfg.db,
                &cfg.name("realized_cap_30d_delta"),
                cfg.version,
                cfg.indexes,
            )?,
            investor_price_cents,
            investor_price,
            investor_price_extra,
            lower_price_band,
            upper_price_band,
            cap_raw,
            investor_cap_raw,
            mvrv,
            realized_profit,
            realized_profit_7d_ema,
            realized_loss,
            realized_loss_7d_ema,
            neg_realized_loss,
            net_realized_pnl,
            net_realized_pnl_7d_ema,
            realized_value,
            realized_profit_rel_to_realized_cap,
            realized_loss_rel_to_realized_cap,
            net_realized_pnl_rel_to_realized_cap,
            total_realized_pnl,
            profit_value_created,
            profit_value_destroyed,
            loss_value_created,
            loss_value_destroyed,
            value_created,
            value_destroyed,
            capitulation_flow,
            profit_flow,
            value_created_24h,
            value_created_7d,
            value_created_30d,
            value_created_1y,
            value_destroyed_24h,
            value_destroyed_7d,
            value_destroyed_30d,
            value_destroyed_1y,
            sopr,
            sopr_24h,
            sopr_7d,
            sopr_30d,
            sopr_1y,
            sopr_24h_7d_ema,
            sopr_7d_ema,
            sopr_24h_30d_ema,
            sopr_30d_ema,
            realized_value_24h,
            realized_value_7d,
            realized_value_30d,
            realized_value_1y,
            sell_side_risk_ratio,
            sell_side_risk_ratio_24h,
            sell_side_risk_ratio_7d,
            sell_side_risk_ratio_30d,
            sell_side_risk_ratio_1y,
            sell_side_risk_ratio_24h_7d_ema,
            sell_side_risk_ratio_7d_ema,
            sell_side_risk_ratio_24h_30d_ema,
            sell_side_risk_ratio_30d_ema,
            net_realized_pnl_cumulative_30d_delta: ComputedFromHeightLast::forced_import(
                cfg.db,
                &cfg.name("net_realized_pnl_cumulative_30d_delta"),
                cfg.version + v3,
                cfg.indexes,
            )?,
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
                ComputedFromHeightLast::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap"),
                    cfg.version + v3,
                    cfg.indexes,
                )?,
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
                ComputedFromHeightLast::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta_rel_to_market_cap"),
                    cfg.version + v3,
                    cfg.indexes,
                )?,
            peak_regret,
            peak_regret_rel_to_realized_cap,
            sent_in_profit: LazyComputedValueFromHeightCumulative::forced_import(
                cfg.db,
                &cfg.name("sent_in_profit"),
                cfg.version,
                cfg.indexes,
            )?,
            sent_in_profit_14d_ema: ValueEmaFromHeight::forced_import(
                cfg.db,
                &cfg.name("sent_in_profit_14d_ema"),
                cfg.version,
                cfg.indexes,
            )?,
            sent_in_loss: LazyComputedValueFromHeightCumulative::forced_import(
                cfg.db,
                &cfg.name("sent_in_loss"),
                cfg.version,
                cfg.indexes,
            )?,
            sent_in_loss_14d_ema: ValueEmaFromHeight::forced_import(
                cfg.db,
                &cfg.name("sent_in_loss_14d_ema"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.realized_cap
            .height
            .len()
            .min(self.realized_profit.height.len())
            .min(self.realized_loss.height.len())
            .min(self.investor_price_cents.height.len())
            .min(self.cap_raw.len())
            .min(self.investor_cap_raw.len())
            .min(self.profit_value_created.height.len())
            .min(self.profit_value_destroyed.height.len())
            .min(self.loss_value_created.height.len())
            .min(self.loss_value_destroyed.height.len())
            .min(self.peak_regret.height.len())
            .min(self.sent_in_profit.sats.height.len())
            .min(self.sent_in_loss.sats.height.len())
    }

    /// Push realized state values to height-indexed vectors.
    pub(crate) fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
        self.realized_cap_cents
            .height
            .truncate_push(height, state.cap())?;
        self.realized_profit
            .height
            .truncate_push(height, state.profit().to_dollars())?;
        self.realized_loss
            .height
            .truncate_push(height, state.loss().to_dollars())?;
        self.investor_price_cents
            .height
            .truncate_push(height, state.investor_price())?;
        self.cap_raw.truncate_push(height, state.cap_raw())?;
        self.investor_cap_raw
            .truncate_push(height, state.investor_cap_raw())?;
        self.profit_value_created
            .height
            .truncate_push(height, state.profit_value_created().to_dollars())?;
        self.profit_value_destroyed
            .height
            .truncate_push(height, state.profit_value_destroyed().to_dollars())?;
        self.loss_value_created
            .height
            .truncate_push(height, state.loss_value_created().to_dollars())?;
        self.loss_value_destroyed
            .height
            .truncate_push(height, state.loss_value_destroyed().to_dollars())?;
        self.peak_regret
            .height
            .truncate_push(height, state.peak_regret().to_dollars())?;
        self.sent_in_profit
            .sats
            .height
            .truncate_push(height, state.sent_in_profit())?;
        self.sent_in_loss
            .sats
            .height
            .truncate_push(height, state.sent_in_loss())?;

        Ok(())
    }

    /// Returns a Vec of mutable references to all stored vecs for parallel writing.
    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.realized_cap_cents.height as &mut dyn AnyStoredVec,
            &mut self.realized_profit.height,
            &mut self.realized_loss.height,
            &mut self.investor_price_cents.height,
            &mut self.cap_raw as &mut dyn AnyStoredVec,
            &mut self.investor_cap_raw as &mut dyn AnyStoredVec,
            &mut self.profit_value_created.height,
            &mut self.profit_value_destroyed.height,
            &mut self.loss_value_created.height,
            &mut self.loss_value_destroyed.height,
            &mut self.peak_regret.height,
            &mut self.sent_in_profit.sats.height,
            &mut self.sent_in_loss.sats.height,
        ]
    }

    /// Compute aggregate values from separate cohorts.
    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.realized_cap_cents.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.realized_cap_cents.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.realized_profit.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.realized_profit.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.realized_loss.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.realized_loss.height)
                .collect::<Vec<_>>(),
            exit,
        )?;

        // Aggregate raw values for investor_price computation
        let investor_price_dep_version = others
            .iter()
            .map(|o| o.investor_price_cents.height.version())
            .fold(vecdb::Version::ZERO, |acc, v| acc + v);
        self.investor_price_cents
            .height
            .validate_computed_version_or_reset(investor_price_dep_version)?;

        let start = self
            .cap_raw
            .len()
            .min(self.investor_cap_raw.len())
            .min(self.investor_price_cents.height.len());
        let end = others.iter().map(|o| o.cap_raw.len()).min().unwrap_or(0);

        // Pre-collect all cohort data to avoid per-element BytesVec reads in nested loop
        let cap_ranges: Vec<Vec<CentsSats>> = others
            .iter()
            .map(|o| o.cap_raw.collect_range_at(start, end))
            .collect();
        let investor_cap_ranges: Vec<Vec<CentsSquaredSats>> = others
            .iter()
            .map(|o| o.investor_cap_raw.collect_range_at(start, end))
            .collect();

        for i in start..end {
            let height = Height::from(i);
            let local_i = i - start;

            let mut sum_cap = CentsSats::ZERO;
            let mut sum_investor_cap = CentsSquaredSats::ZERO;

            for idx in 0..others.len() {
                sum_cap += cap_ranges[idx][local_i];
                sum_investor_cap += investor_cap_ranges[idx][local_i];
            }

            self.cap_raw.truncate_push(height, sum_cap)?;
            self.investor_cap_raw
                .truncate_push(height, sum_investor_cap)?;

            let investor_price = if sum_cap.inner() == 0 {
                Cents::ZERO
            } else {
                Cents::new((sum_investor_cap / sum_cap.inner()) as u64)
            };
            self.investor_price_cents
                .height
                .truncate_push(height, investor_price)?;
        }

        {
            let _lock = exit.lock();
            self.investor_price_cents.height.write()?;
        }

        self.profit_value_created.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.profit_value_created.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.profit_value_destroyed.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.profit_value_destroyed.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.loss_value_created.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.loss_value_created.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.loss_value_destroyed.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.loss_value_destroyed.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.peak_regret.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.peak_regret.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.sent_in_profit.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent_in_profit.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.sent_in_loss.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent_in_loss.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;

        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.realized_profit
            .compute_rest(starting_indexes.height, exit)?;
        self.realized_loss
            .compute_rest(starting_indexes.height, exit)?;

        self.net_realized_pnl
            .compute(starting_indexes.height, exit, |vec| {
                vec.compute_subtract(
                    starting_indexes.height,
                    &self.realized_profit.height,
                    &self.realized_loss.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.realized_value.height.compute_add(
            starting_indexes.height,
            &self.realized_profit.height,
            &self.realized_loss.height,
            exit,
        )?;

        self.peak_regret
            .compute_rest(starting_indexes.height, exit)?;

        Ok(())
    }

    /// Second phase of computed metrics (base-only parts: realized price, rolling sums, EMAs).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2_base(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized_price.usd.height.compute_divide(
            starting_indexes.height,
            &self.realized_cap.height,
            height_to_supply,
            exit,
        )?;

        self.realized_price_extra.compute_ratio(
            starting_indexes,
            &prices.price.usd,
            &self.realized_price.usd.height,
            exit,
        )?;

        self.investor_price_extra.compute_ratio(
            starting_indexes,
            &prices.price.usd,
            &self.investor_price.usd.height,
            exit,
        )?;

        self.realized_cap_30d_delta.height.compute_rolling_change(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.realized_cap.height,
            exit,
        )?;

        // Compute value_created/destroyed from stored components
        self.value_created
            .compute_binary::<Dollars, Dollars, DollarsPlus>(
                starting_indexes.height,
                &self.profit_value_created.height,
                &self.loss_value_created.height,
                exit,
            )?;
        self.value_destroyed
            .compute_binary::<Dollars, Dollars, DollarsPlus>(
                starting_indexes.height,
                &self.profit_value_destroyed.height,
                &self.loss_value_destroyed.height,
                exit,
            )?;

        // === Rolling sum intermediates ===
        macro_rules! rolling_sum {
            ($target:expr, $window:expr, $source:expr) => {
                $target.height.compute_rolling_sum(
                    starting_indexes.height,
                    $window,
                    $source,
                    exit,
                )?
            };
        }

        rolling_sum!(
            self.value_created_24h,
            &blocks.count.height_24h_ago,
            &self.value_created.height
        );
        rolling_sum!(
            self.value_created_7d,
            &blocks.count.height_1w_ago,
            &self.value_created.height
        );
        rolling_sum!(
            self.value_created_30d,
            &blocks.count.height_1m_ago,
            &self.value_created.height
        );
        rolling_sum!(
            self.value_created_1y,
            &blocks.count.height_1y_ago,
            &self.value_created.height
        );
        rolling_sum!(
            self.value_destroyed_24h,
            &blocks.count.height_24h_ago,
            &self.value_destroyed.height
        );
        rolling_sum!(
            self.value_destroyed_7d,
            &blocks.count.height_1w_ago,
            &self.value_destroyed.height
        );
        rolling_sum!(
            self.value_destroyed_30d,
            &blocks.count.height_1m_ago,
            &self.value_destroyed.height
        );
        rolling_sum!(
            self.value_destroyed_1y,
            &blocks.count.height_1y_ago,
            &self.value_destroyed.height
        );

        // Realized value rolling sums
        rolling_sum!(
            self.realized_value_24h,
            &blocks.count.height_24h_ago,
            &self.realized_value.height
        );
        rolling_sum!(
            self.realized_value_7d,
            &blocks.count.height_1w_ago,
            &self.realized_value.height
        );
        rolling_sum!(
            self.realized_value_30d,
            &blocks.count.height_1m_ago,
            &self.realized_value.height
        );
        rolling_sum!(
            self.realized_value_1y,
            &blocks.count.height_1y_ago,
            &self.realized_value.height
        );

        // Compute SOPR from rolling sums
        self.sopr_24h.compute_binary::<Dollars, Dollars, Ratio64>(
            starting_indexes.height,
            &self.value_created_24h.height,
            &self.value_destroyed_24h.height,
            exit,
        )?;
        self.sopr_7d.compute_binary::<Dollars, Dollars, Ratio64>(
            starting_indexes.height,
            &self.value_created_7d.height,
            &self.value_destroyed_7d.height,
            exit,
        )?;
        self.sopr_30d.compute_binary::<Dollars, Dollars, Ratio64>(
            starting_indexes.height,
            &self.value_created_30d.height,
            &self.value_destroyed_30d.height,
            exit,
        )?;
        self.sopr_1y.compute_binary::<Dollars, Dollars, Ratio64>(
            starting_indexes.height,
            &self.value_created_1y.height,
            &self.value_destroyed_1y.height,
            exit,
        )?;

        // Compute sell-side risk ratios
        self.sell_side_risk_ratio_24h
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                starting_indexes.height,
                &self.realized_value_24h.height,
                &self.realized_cap.height,
                exit,
            )?;
        self.sell_side_risk_ratio_7d
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                starting_indexes.height,
                &self.realized_value_7d.height,
                &self.realized_cap.height,
                exit,
            )?;
        self.sell_side_risk_ratio_30d
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                starting_indexes.height,
                &self.realized_value_30d.height,
                &self.realized_cap.height,
                exit,
            )?;
        self.sell_side_risk_ratio_1y
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                starting_indexes.height,
                &self.realized_value_1y.height,
                &self.realized_cap.height,
                exit,
            )?;

        // 7d rolling averages
        self.realized_profit_7d_ema.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.realized_profit.height,
            exit,
        )?;
        self.realized_loss_7d_ema.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.realized_loss.height,
            exit,
        )?;
        self.net_realized_pnl_7d_ema
            .height
            .compute_rolling_average(
                starting_indexes.height,
                &blocks.count.height_1w_ago,
                &self.net_realized_pnl.height,
                exit,
            )?;

        // 14-day rolling average of sent in profit/loss
        self.sent_in_profit_14d_ema.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent_in_profit.sats.height,
            &self.sent_in_profit.usd.height,
            exit,
        )?;
        self.sent_in_loss_14d_ema.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent_in_loss.sats.height,
            &self.sent_in_loss.usd.height,
            exit,
        )?;

        // SOPR EMAs
        self.sopr_24h_7d_ema.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.sopr.height,
            exit,
        )?;
        self.sopr_24h_30d_ema.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.sopr.height,
            exit,
        )?;

        // Sell side risk EMAs
        self.sell_side_risk_ratio_24h_7d_ema
            .height
            .compute_rolling_average(
                starting_indexes.height,
                &blocks.count.height_1w_ago,
                &self.sell_side_risk_ratio.height,
                exit,
            )?;
        self.sell_side_risk_ratio_24h_30d_ema
            .height
            .compute_rolling_average(
                starting_indexes.height,
                &blocks.count.height_1m_ago,
                &self.sell_side_risk_ratio.height,
                exit,
            )?;

        // Realized profit/loss/net relative to realized cap
        self.realized_profit_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                starting_indexes.height,
                &self.realized_profit.height,
                &self.realized_cap.height,
                exit,
            )?;
        self.realized_loss_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                starting_indexes.height,
                &self.realized_loss.height,
                &self.realized_cap.height,
                exit,
            )?;
        self.net_realized_pnl_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                starting_indexes.height,
                &self.net_realized_pnl.height,
                &self.realized_cap.height,
                exit,
            )?;
        self.peak_regret_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                starting_indexes.height,
                &self.peak_regret.height,
                &self.realized_cap.height,
                exit,
            )?;

        // Net realized PnL cumulative 30d delta
        self.net_realized_pnl_cumulative_30d_delta
            .height
            .compute_rolling_change(
                starting_indexes.height,
                &blocks.count.height_1m_ago,
                &self.net_realized_pnl.cumulative.height,
                exit,
            )?;

        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap
            .height
            .compute_percentage(
                starting_indexes.height,
                &self.net_realized_pnl_cumulative_30d_delta.height,
                &self.realized_cap.height,
                exit,
            )?;

        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap
            .height
            .compute_percentage(
                starting_indexes.height,
                &self.net_realized_pnl_cumulative_30d_delta.height,
                height_to_market_cap,
                exit,
            )?;

        Ok(())
    }
}
