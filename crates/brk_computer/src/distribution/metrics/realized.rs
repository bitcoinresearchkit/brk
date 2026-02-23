use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, Cents, CentsSats, CentsSquaredSats, Dollars, Height, StoredF32, StoredF64,
    Version,
};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Exit, WritableVec, Ident, ImportableVec,
    ReadableCloneableVec, ReadableVec, Negate, Rw, StorageMode,
};

use crate::{
    ComputeIndexes, blocks,
    distribution::state::RealizedState,
    internal::{
        CentsUnsignedToDollars, ComputedFromHeightLast, ComputedFromHeightRatio,
        ComputedFromHeightSum, ComputedFromHeightSumCum, DollarsMinus, DollarsPlus,
        DollarsSquaredDivide, LazyBinaryFromHeightLast, LazyBinaryFromHeightSum,
        LazyBinaryFromHeightSumCum, LazyBinaryPriceFromHeight,
        LazyComputedValueFromHeightSumCum, LazyFromHeightLast, LazyFromHeightSum,
        LazyFromHeightSumCum, LazyPriceFromCents, PercentageDollarsF32, Price, PriceFromHeight,
        Ratio64, StoredF32Identity, ValueEmaFromHeight,
    },
    prices,
};

use super::ImportConfig;

/// Realized cap and related metrics.
#[derive(Traversable)]
pub struct RealizedMetrics<M: StorageMode = Rw> {
    // === Realized Cap ===
    pub realized_cap_cents: ComputedFromHeightLast<Cents, M>,
    pub realized_cap: LazyFromHeightLast<Dollars, Cents>,
    pub realized_price: Price<ComputedFromHeightLast<Dollars, M>>,
    pub realized_price_extra: ComputedFromHeightRatio<M>,
    pub realized_cap_rel_to_own_market_cap: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub realized_cap_30d_delta: ComputedFromHeightLast<Dollars, M>,

    // === Investor Price (dollar-weighted average acquisition price) ===
    pub investor_price_cents: ComputedFromHeightLast<Cents, M>,
    pub investor_price: LazyPriceFromCents,
    pub investor_price_extra: ComputedFromHeightRatio<M>,

    // === Floor/Ceiling Price Bands (lazy: realized²/investor, investor²/realized) ===
    pub lower_price_band: LazyBinaryPriceFromHeight,
    pub upper_price_band: LazyBinaryPriceFromHeight,

    // === Raw values for aggregation (needed to compute investor_price for aggregated cohorts) ===
    /// Raw Σ(price × sats) for realized cap aggregation
    pub cap_raw: M::Stored<BytesVec<Height, CentsSats>>,
    /// Raw Σ(price² × sats) for investor_price aggregation
    pub investor_cap_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    // === MVRV (Market Value to Realized Value) ===
    // Proxy for realized_price_extra.ratio (close / realized_price = market_cap / realized_cap)
    pub mvrv: LazyFromHeightLast<StoredF32>,

    // === Realized Profit/Loss ===
    pub realized_profit: ComputedFromHeightSumCum<Dollars, M>,
    pub realized_profit_7d_ema: ComputedFromHeightLast<Dollars, M>,
    pub realized_loss: ComputedFromHeightSumCum<Dollars, M>,
    pub realized_loss_7d_ema: ComputedFromHeightLast<Dollars, M>,
    pub neg_realized_loss: LazyFromHeightSumCum<Dollars>,
    pub net_realized_pnl: ComputedFromHeightSumCum<Dollars, M>,
    pub net_realized_pnl_7d_ema: ComputedFromHeightLast<Dollars, M>,
    pub realized_value: ComputedFromHeightSum<Dollars, M>,

    // === Realized vs Realized Cap Ratios (lazy) ===
    pub realized_profit_rel_to_realized_cap:
        LazyBinaryFromHeightSumCum<StoredF32, Dollars, Dollars>,
    pub realized_loss_rel_to_realized_cap: LazyBinaryFromHeightSumCum<StoredF32, Dollars, Dollars>,
    pub net_realized_pnl_rel_to_realized_cap:
        LazyBinaryFromHeightSumCum<StoredF32, Dollars, Dollars>,

    // === Total Realized PnL ===
    pub total_realized_pnl: LazyFromHeightSum<Dollars>,

    // === Realized Profit/Loss Rolling Sums ===
    pub realized_profit_24h: Option<ComputedFromHeightLast<Dollars, M>>,
    pub realized_profit_7d: Option<ComputedFromHeightLast<Dollars, M>>,
    pub realized_profit_30d: Option<ComputedFromHeightLast<Dollars, M>>,
    pub realized_profit_1y: Option<ComputedFromHeightLast<Dollars, M>>,
    pub realized_loss_24h: Option<ComputedFromHeightLast<Dollars, M>>,
    pub realized_loss_7d: Option<ComputedFromHeightLast<Dollars, M>>,
    pub realized_loss_30d: Option<ComputedFromHeightLast<Dollars, M>>,
    pub realized_loss_1y: Option<ComputedFromHeightLast<Dollars, M>>,

    // === Realized Profit to Loss Ratio (lazy from rolling sums) ===
    pub realized_profit_to_loss_ratio_24h: Option<LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>>,
    pub realized_profit_to_loss_ratio_7d: Option<LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>>,
    pub realized_profit_to_loss_ratio_30d: Option<LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>>,
    pub realized_profit_to_loss_ratio_1y: Option<LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>>,

    // === Value Created/Destroyed Splits (stored) ===
    pub profit_value_created: ComputedFromHeightSum<Dollars, M>,
    pub profit_value_destroyed: ComputedFromHeightSum<Dollars, M>,
    pub loss_value_created: ComputedFromHeightSum<Dollars, M>,
    pub loss_value_destroyed: ComputedFromHeightSum<Dollars, M>,

    // === Value Created/Destroyed Totals (lazy: profit + loss) ===
    pub value_created: LazyBinaryFromHeightSum<Dollars, Dollars, Dollars>,
    pub value_destroyed: LazyBinaryFromHeightSum<Dollars, Dollars, Dollars>,

    // === Capitulation/Profit Flow (lazy aliases) ===
    pub capitulation_flow: LazyFromHeightSum<Dollars>,
    pub profit_flow: LazyFromHeightSum<Dollars>,

    // === Adjusted Value (lazy: cohort - up_to_1h) ===
    pub adjusted_value_created: Option<LazyBinaryFromHeightSum<Dollars, Dollars, Dollars>>,
    pub adjusted_value_destroyed: Option<LazyBinaryFromHeightSum<Dollars, Dollars, Dollars>>,

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
    pub sopr_24h: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub sopr_7d: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub sopr_30d: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub sopr_1y: LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>,
    pub sopr_24h_7d_ema: ComputedFromHeightLast<StoredF64, M>,
    pub sopr_7d_ema: LazyFromHeightLast<StoredF64>,
    pub sopr_24h_30d_ema: ComputedFromHeightLast<StoredF64, M>,
    pub sopr_30d_ema: LazyFromHeightLast<StoredF64>,

    // === Adjusted Value Created/Destroyed Rolling Sums ===
    pub adjusted_value_created_24h: Option<ComputedFromHeightLast<Dollars, M>>,
    pub adjusted_value_created_7d: Option<ComputedFromHeightLast<Dollars, M>>,
    pub adjusted_value_created_30d: Option<ComputedFromHeightLast<Dollars, M>>,
    pub adjusted_value_created_1y: Option<ComputedFromHeightLast<Dollars, M>>,
    pub adjusted_value_destroyed_24h: Option<ComputedFromHeightLast<Dollars, M>>,
    pub adjusted_value_destroyed_7d: Option<ComputedFromHeightLast<Dollars, M>>,
    pub adjusted_value_destroyed_30d: Option<ComputedFromHeightLast<Dollars, M>>,
    pub adjusted_value_destroyed_1y: Option<ComputedFromHeightLast<Dollars, M>>,

    // === Adjusted SOPR (rolling window ratios) ===
    pub adjusted_sopr: Option<LazyFromHeightLast<StoredF64>>,
    pub adjusted_sopr_24h: Option<LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>>,
    pub adjusted_sopr_7d: Option<LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>>,
    pub adjusted_sopr_30d: Option<LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>>,
    pub adjusted_sopr_1y: Option<LazyBinaryFromHeightLast<StoredF64, Dollars, Dollars>>,
    pub adjusted_sopr_24h_7d_ema: Option<ComputedFromHeightLast<StoredF64, M>>,
    pub adjusted_sopr_7d_ema: Option<LazyFromHeightLast<StoredF64>>,
    pub adjusted_sopr_24h_30d_ema: Option<ComputedFromHeightLast<StoredF64, M>>,
    pub adjusted_sopr_30d_ema: Option<LazyFromHeightLast<StoredF64>>,

    // === Sell Side Risk Rolling Sum Intermediates ===
    pub realized_value_24h: ComputedFromHeightLast<Dollars, M>,
    pub realized_value_7d: ComputedFromHeightLast<Dollars, M>,
    pub realized_value_30d: ComputedFromHeightLast<Dollars, M>,
    pub realized_value_1y: ComputedFromHeightLast<Dollars, M>,

    // === Sell Side Risk (rolling window ratios) ===
    pub sell_side_risk_ratio: LazyFromHeightLast<StoredF32>,
    pub sell_side_risk_ratio_24h: LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub sell_side_risk_ratio_7d: LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub sell_side_risk_ratio_30d: LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub sell_side_risk_ratio_1y: LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub sell_side_risk_ratio_24h_7d_ema: ComputedFromHeightLast<StoredF32, M>,
    pub sell_side_risk_ratio_7d_ema: LazyFromHeightLast<StoredF32>,
    pub sell_side_risk_ratio_24h_30d_ema: ComputedFromHeightLast<StoredF32, M>,
    pub sell_side_risk_ratio_30d_ema: LazyFromHeightLast<StoredF32>,

    // === Net Realized PnL Deltas ===
    pub net_realized_pnl_cumulative_30d_delta: ComputedFromHeightLast<Dollars, M>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: ComputedFromHeightLast<StoredF32, M>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: ComputedFromHeightLast<StoredF32, M>,

    // === Peak Regret ===
    /// Realized peak regret: Σ((peak - sell_price) × sats)
    /// where peak = max price during holding period.
    /// "How much more could have been made by selling at peak instead"
    pub peak_regret: ComputedFromHeightSumCum<Dollars, M>,
    /// Peak regret as % of realized cap
    pub peak_regret_rel_to_realized_cap: LazyBinaryFromHeightSum<StoredF32, Dollars, Dollars>,

    // === Sent in Profit/Loss ===
    /// Sats sent in profit (sats/btc/usd)
    pub sent_in_profit: LazyComputedValueFromHeightSumCum<M>,
    /// 14-day EMA of sent in profit (sats, btc, usd)
    pub sent_in_profit_14d_ema: ValueEmaFromHeight<M>,
    /// Sats sent in loss (sats/btc/usd)
    pub sent_in_loss: LazyComputedValueFromHeightSumCum<M>,
    /// 14-day EMA of sent in loss (sats, btc, usd)
    pub sent_in_loss_14d_ema: ValueEmaFromHeight<M>,
}

impl RealizedMetrics {
    /// Import realized metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);
        let v3 = Version::new(3);
        let extended = cfg.extended();
        let compute_adjusted = cfg.compute_adjusted();

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

        let realized_profit = ComputedFromHeightSumCum::forced_import(
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

        let realized_loss = ComputedFromHeightSumCum::forced_import(
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

        let neg_realized_loss = LazyFromHeightSumCum::from_computed::<Negate>(
            &cfg.name("neg_realized_loss"),
            cfg.version + v1,
            realized_loss.height.read_only_boxed_clone(),
            &realized_loss,
        );

        let net_realized_pnl = ComputedFromHeightSumCum::forced_import(
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

        let peak_regret = ComputedFromHeightSumCum::forced_import(
            cfg.db,
            &cfg.name("realized_peak_regret"),
            cfg.version + v2,
            cfg.indexes,
        )?;

        // realized_value is the source for total_realized_pnl (they're identical)
        let realized_value = ComputedFromHeightSum::forced_import(
            cfg.db,
            &cfg.name("realized_value"),
            cfg.version,
            cfg.indexes,
        )?;

        // total_realized_pnl is a lazy alias to realized_value
        let total_realized_pnl = LazyFromHeightSum::from_computed::<Ident>(
            &cfg.name("total_realized_pnl"),
            cfg.version + v1,
            realized_value.height.read_only_boxed_clone(),
            &realized_value,
        );

        // Construct lazy ratio vecs
        let realized_profit_rel_to_realized_cap =
            LazyBinaryFromHeightSumCum::from_computed_lazy_last::<PercentageDollarsF32, _>(
                &cfg.name("realized_profit_rel_to_realized_cap"),
                cfg.version + v1,
                realized_profit.height.read_only_boxed_clone(),
                realized_cap.height.read_only_boxed_clone(),
                &realized_profit,
                &realized_cap,
            );

        let realized_loss_rel_to_realized_cap =
            LazyBinaryFromHeightSumCum::from_computed_lazy_last::<PercentageDollarsF32, _>(
                &cfg.name("realized_loss_rel_to_realized_cap"),
                cfg.version + v1,
                realized_loss.height.read_only_boxed_clone(),
                realized_cap.height.read_only_boxed_clone(),
                &realized_loss,
                &realized_cap,
            );

        let net_realized_pnl_rel_to_realized_cap =
            LazyBinaryFromHeightSumCum::from_computed_lazy_last::<PercentageDollarsF32, _>(
                &cfg.name("net_realized_pnl_rel_to_realized_cap"),
                cfg.version + v1,
                net_realized_pnl.height.read_only_boxed_clone(),
                realized_cap.height.read_only_boxed_clone(),
                &net_realized_pnl,
                &realized_cap,
            );

        let realized_price = PriceFromHeight::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        // Investor price (dollar-weighted average acquisition price)
        let investor_price_cents = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("investor_price_cents"),
            cfg.version,
            cfg.indexes,
        )?;

        let investor_price = LazyPriceFromCents::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("investor_price"),
            cfg.version,
            &investor_price_cents,
        );

        let investor_price_extra = ComputedFromHeightRatio::forced_import_from_lazy(
            cfg.db,
            &cfg.name("investor_price"),
            &investor_price.usd,
            cfg.version,
            cfg.indexes,
            extended,
        )?;

        // Floor price = realized² / investor (lower band)
        let lower_price_band =
            LazyBinaryPriceFromHeight::from_price_and_lazy_price::<DollarsSquaredDivide>(
                &cfg.name("lower_price_band"),
                cfg.version,
                &realized_price,
                &investor_price,
            );

        // Ceiling price = investor² / realized (upper band)
        let upper_price_band =
            LazyBinaryPriceFromHeight::from_lazy_price_and_price::<DollarsSquaredDivide>(
                &cfg.name("upper_price_band"),
                cfg.version,
                &investor_price,
                &realized_price,
            );

        // Raw values for aggregation
        let cap_raw = BytesVec::forced_import(cfg.db, &cfg.name("cap_raw"), cfg.version)?;
        let investor_cap_raw =
            BytesVec::forced_import(cfg.db, &cfg.name("investor_cap_raw"), cfg.version)?;

        // Import the 4 splits (stored)
        let profit_value_created = ComputedFromHeightSum::forced_import(
            cfg.db,
            &cfg.name("profit_value_created"),
            cfg.version,
            cfg.indexes,
        )?;

        let profit_value_destroyed = ComputedFromHeightSum::forced_import(
            cfg.db,
            &cfg.name("profit_value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;

        let loss_value_created = ComputedFromHeightSum::forced_import(
            cfg.db,
            &cfg.name("loss_value_created"),
            cfg.version,
            cfg.indexes,
        )?;

        let loss_value_destroyed = ComputedFromHeightSum::forced_import(
            cfg.db,
            &cfg.name("loss_value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;

        // Create lazy totals (profit + loss)
        let value_created = LazyBinaryFromHeightSum::from_computed::<DollarsPlus>(
            &cfg.name("value_created"),
            cfg.version,
            &profit_value_created,
            &loss_value_created,
        );

        let value_destroyed = LazyBinaryFromHeightSum::from_computed::<DollarsPlus>(
            &cfg.name("value_destroyed"),
            cfg.version,
            &profit_value_destroyed,
            &loss_value_destroyed,
        );

        // Create lazy aliases
        let capitulation_flow = LazyFromHeightSum::from_computed::<Ident>(
            &cfg.name("capitulation_flow"),
            cfg.version,
            loss_value_destroyed.height.read_only_boxed_clone(),
            &loss_value_destroyed,
        );

        let profit_flow = LazyFromHeightSum::from_computed::<Ident>(
            &cfg.name("profit_flow"),
            cfg.version,
            profit_value_destroyed.height.read_only_boxed_clone(),
            &profit_value_destroyed,
        );

        // Create lazy adjusted vecs if compute_adjusted and up_to_1h is available
        let adjusted_value_created =
            (compute_adjusted && cfg.up_to_1h_realized.is_some()).then(|| {
                let up_to_1h = cfg.up_to_1h_realized.unwrap();
                LazyBinaryFromHeightSum::from_binary::<
                    DollarsMinus,
                    Dollars,
                    Dollars,
                    Dollars,
                    Dollars,
                >(
                    &cfg.name("adjusted_value_created"),
                    cfg.version,
                    &value_created,
                    &up_to_1h.value_created,
                )
            });
        let adjusted_value_destroyed =
            (compute_adjusted && cfg.up_to_1h_realized.is_some()).then(|| {
                let up_to_1h = cfg.up_to_1h_realized.unwrap();
                LazyBinaryFromHeightSum::from_binary::<
                    DollarsMinus,
                    Dollars,
                    Dollars,
                    Dollars,
                    Dollars,
                >(
                    &cfg.name("adjusted_value_destroyed"),
                    cfg.version,
                    &value_destroyed,
                    &up_to_1h.value_destroyed,
                )
            });

        // Create realized_price_extra first so we can reference its ratio for MVRV proxy
        let realized_price_extra = ComputedFromHeightRatio::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            Some(&realized_price),
            cfg.version + v1,
            cfg.indexes,
            extended,
        )?;

        // MVRV is a lazy proxy for realized_price_extra.ratio
        // ratio = close / realized_price = market_cap / realized_cap = MVRV
        let mvrv = LazyFromHeightLast::from_computed::<StoredF32Identity>(
            &cfg.name("mvrv"),
            cfg.version,
            realized_price_extra.ratio.height.read_only_boxed_clone(),
            &realized_price_extra.ratio,
        );

        // === Rolling sum intermediates (must be imported before lazy ratios reference them) ===
        macro_rules! import_rolling {
            ($name:expr) => {
                ComputedFromHeightLast::forced_import(cfg.db, &cfg.name($name), cfg.version + v1, cfg.indexes)?
            };
        }
        macro_rules! import_rolling_opt {
            ($cond:expr, $name:expr) => {
                $cond.then(|| ComputedFromHeightLast::forced_import(cfg.db, &cfg.name($name), cfg.version + v1, cfg.indexes)).transpose()?
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

        let adjusted_value_created_24h = import_rolling_opt!(compute_adjusted, "adjusted_value_created_24h");
        let adjusted_value_created_7d = import_rolling_opt!(compute_adjusted, "adjusted_value_created_7d");
        let adjusted_value_created_30d = import_rolling_opt!(compute_adjusted, "adjusted_value_created_30d");
        let adjusted_value_created_1y = import_rolling_opt!(compute_adjusted, "adjusted_value_created_1y");
        let adjusted_value_destroyed_24h = import_rolling_opt!(compute_adjusted, "adjusted_value_destroyed_24h");
        let adjusted_value_destroyed_7d = import_rolling_opt!(compute_adjusted, "adjusted_value_destroyed_7d");
        let adjusted_value_destroyed_30d = import_rolling_opt!(compute_adjusted, "adjusted_value_destroyed_30d");
        let adjusted_value_destroyed_1y = import_rolling_opt!(compute_adjusted, "adjusted_value_destroyed_1y");

        let realized_value_24h = import_rolling!("realized_value_24h");
        let realized_value_7d = import_rolling!("realized_value_7d");
        let realized_value_30d = import_rolling!("realized_value_30d");
        let realized_value_1y = import_rolling!("realized_value_1y");

        let realized_profit_24h = import_rolling_opt!(extended, "realized_profit_24h");
        let realized_profit_7d = import_rolling_opt!(extended, "realized_profit_7d");
        let realized_profit_30d = import_rolling_opt!(extended, "realized_profit_30d");
        let realized_profit_1y = import_rolling_opt!(extended, "realized_profit_1y");
        let realized_loss_24h = import_rolling_opt!(extended, "realized_loss_24h");
        let realized_loss_7d = import_rolling_opt!(extended, "realized_loss_7d");
        let realized_loss_30d = import_rolling_opt!(extended, "realized_loss_30d");
        let realized_loss_1y = import_rolling_opt!(extended, "realized_loss_1y");

        // === Rolling window lazy ratios (from rolling sum intermediates) ===
        let sopr_24h = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("sopr_24h"), cfg.version + v1, &value_created_24h, &value_destroyed_24h,
        );
        let sopr_7d = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("sopr_7d"), cfg.version + v1, &value_created_7d, &value_destroyed_7d,
        );
        let sopr_30d = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("sopr_30d"), cfg.version + v1, &value_created_30d, &value_destroyed_30d,
        );
        let sopr_1y = LazyBinaryFromHeightLast::from_computed_last::<Ratio64>(
            &cfg.name("sopr_1y"), cfg.version + v1, &value_created_1y, &value_destroyed_1y,
        );
        let sopr = LazyFromHeightLast::from_binary::<Ident, Dollars, Dollars>(
            &cfg.name("sopr"), cfg.version + v1, &sopr_24h,
        );

        macro_rules! lazy_binary_from_opt_last {
            ($transform:ty, $name:expr, $s1:expr, $s2:expr) => {
                ($s1.is_some() && $s2.is_some()).then(|| {
                    LazyBinaryFromHeightLast::from_computed_last::<$transform>(
                        &cfg.name($name), cfg.version + v1,
                        $s1.as_ref().unwrap(), $s2.as_ref().unwrap(),
                    )
                })
            };
        }
        let adjusted_sopr_24h = lazy_binary_from_opt_last!(Ratio64, "adjusted_sopr_24h", adjusted_value_created_24h, adjusted_value_destroyed_24h);
        let adjusted_sopr_7d = lazy_binary_from_opt_last!(Ratio64, "adjusted_sopr_7d", adjusted_value_created_7d, adjusted_value_destroyed_7d);
        let adjusted_sopr_30d = lazy_binary_from_opt_last!(Ratio64, "adjusted_sopr_30d", adjusted_value_created_30d, adjusted_value_destroyed_30d);
        let adjusted_sopr_1y = lazy_binary_from_opt_last!(Ratio64, "adjusted_sopr_1y", adjusted_value_created_1y, adjusted_value_destroyed_1y);
        let adjusted_sopr = adjusted_sopr_24h.as_ref().map(|sopr_24h| {
            LazyFromHeightLast::from_binary::<Ident, Dollars, Dollars>(
                &cfg.name("adjusted_sopr"), cfg.version + v1, sopr_24h,
            )
        });

        let sell_side_risk_ratio_24h = LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<PercentageDollarsF32, _>(
            &cfg.name("sell_side_risk_ratio_24h"), cfg.version + v1, &realized_value_24h, &realized_cap,
        );
        let sell_side_risk_ratio_7d = LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<PercentageDollarsF32, _>(
            &cfg.name("sell_side_risk_ratio_7d"), cfg.version + v1, &realized_value_7d, &realized_cap,
        );
        let sell_side_risk_ratio_30d = LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<PercentageDollarsF32, _>(
            &cfg.name("sell_side_risk_ratio_30d"), cfg.version + v1, &realized_value_30d, &realized_cap,
        );
        let sell_side_risk_ratio_1y = LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<PercentageDollarsF32, _>(
            &cfg.name("sell_side_risk_ratio_1y"), cfg.version + v1, &realized_value_1y, &realized_cap,
        );
        let sell_side_risk_ratio = LazyFromHeightLast::from_binary::<Ident, Dollars, Dollars>(
            &cfg.name("sell_side_risk_ratio"), cfg.version + v1, &sell_side_risk_ratio_24h,
        );

        let realized_profit_to_loss_ratio_24h = lazy_binary_from_opt_last!(Ratio64, "realized_profit_to_loss_ratio_24h", realized_profit_24h, realized_loss_24h);
        let realized_profit_to_loss_ratio_7d = lazy_binary_from_opt_last!(Ratio64, "realized_profit_to_loss_ratio_7d", realized_profit_7d, realized_loss_7d);
        let realized_profit_to_loss_ratio_30d = lazy_binary_from_opt_last!(Ratio64, "realized_profit_to_loss_ratio_30d", realized_profit_30d, realized_loss_30d);
        let realized_profit_to_loss_ratio_1y = lazy_binary_from_opt_last!(Ratio64, "realized_profit_to_loss_ratio_1y", realized_profit_1y, realized_loss_1y);

        // === EMA imports + identity aliases ===
        let sopr_24h_7d_ema = import_rolling!("sopr_24h_7d_ema");
        let sopr_7d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sopr_7d_ema"), cfg.version + v1,
            sopr_24h_7d_ema.height.read_only_boxed_clone(), &sopr_24h_7d_ema,
        );
        let sopr_24h_30d_ema = import_rolling!("sopr_24h_30d_ema");
        let sopr_30d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sopr_30d_ema"), cfg.version + v1,
            sopr_24h_30d_ema.height.read_only_boxed_clone(), &sopr_24h_30d_ema,
        );

        let adjusted_sopr_24h_7d_ema = import_rolling_opt!(compute_adjusted, "adjusted_sopr_24h_7d_ema");
        let adjusted_sopr_7d_ema = adjusted_sopr_24h_7d_ema.as_ref().map(|ema| {
            LazyFromHeightLast::from_computed::<Ident>(
                &cfg.name("adjusted_sopr_7d_ema"), cfg.version + v1,
                ema.height.read_only_boxed_clone(), ema,
            )
        });
        let adjusted_sopr_24h_30d_ema = import_rolling_opt!(compute_adjusted, "adjusted_sopr_24h_30d_ema");
        let adjusted_sopr_30d_ema = adjusted_sopr_24h_30d_ema.as_ref().map(|ema| {
            LazyFromHeightLast::from_computed::<Ident>(
                &cfg.name("adjusted_sopr_30d_ema"), cfg.version + v1,
                ema.height.read_only_boxed_clone(), ema,
            )
        });

        let sell_side_risk_ratio_24h_7d_ema = import_rolling!("sell_side_risk_ratio_24h_7d_ema");
        let sell_side_risk_ratio_7d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sell_side_risk_ratio_7d_ema"), cfg.version + v1,
            sell_side_risk_ratio_24h_7d_ema.height.read_only_boxed_clone(), &sell_side_risk_ratio_24h_7d_ema,
        );
        let sell_side_risk_ratio_24h_30d_ema = import_rolling!("sell_side_risk_ratio_24h_30d_ema");
        let sell_side_risk_ratio_30d_ema = LazyFromHeightLast::from_computed::<Ident>(
            &cfg.name("sell_side_risk_ratio_30d_ema"), cfg.version + v1,
            sell_side_risk_ratio_24h_30d_ema.height.read_only_boxed_clone(), &sell_side_risk_ratio_24h_30d_ema,
        );

        let peak_regret_rel_to_realized_cap = LazyBinaryFromHeightSum::from_sumcum_lazy_last::<
            PercentageDollarsF32,
            _,
        >(
            &cfg.name("peak_regret_rel_to_realized_cap"),
            cfg.version + v1,
            peak_regret.height.read_only_boxed_clone(),
            realized_cap.height.read_only_boxed_clone(),
            &peak_regret,
            &realized_cap,
        );

        Ok(Self {
            // === Realized Cap ===
            realized_cap_cents,
            realized_cap,
            realized_price,
            realized_price_extra,
            realized_cap_rel_to_own_market_cap: extended
                .then(|| {
                    ComputedFromHeightLast::forced_import(
                        cfg.db,
                        &cfg.name("realized_cap_rel_to_own_market_cap"),
                        cfg.version,
                        cfg.indexes,
                    )
                })
                .transpose()?,
            realized_cap_30d_delta: ComputedFromHeightLast::forced_import(
                cfg.db,
                &cfg.name("realized_cap_30d_delta"),
                cfg.version,
                cfg.indexes,
            )?,

            // === Investor Price ===
            investor_price_cents,
            investor_price,
            investor_price_extra,

            // === Floor/Ceiling Price Bands ===
            lower_price_band,
            upper_price_band,

            cap_raw,
            investor_cap_raw,

            // === MVRV ===
            mvrv,

            // === Realized Profit/Loss ===
            realized_profit,
            realized_profit_7d_ema,
            realized_loss,
            realized_loss_7d_ema,
            neg_realized_loss,
            net_realized_pnl,
            net_realized_pnl_7d_ema,
            realized_value,

            // === Realized vs Realized Cap Ratios (lazy) ===
            realized_profit_rel_to_realized_cap,
            realized_loss_rel_to_realized_cap,
            net_realized_pnl_rel_to_realized_cap,

            // === Total Realized PnL ===
            total_realized_pnl,

            // === Realized Profit/Loss Rolling Sums ===
            realized_profit_24h,
            realized_profit_7d,
            realized_profit_30d,
            realized_profit_1y,
            realized_loss_24h,
            realized_loss_7d,
            realized_loss_30d,
            realized_loss_1y,

            // === Realized Profit to Loss Ratio (lazy from rolling sums) ===
            realized_profit_to_loss_ratio_24h,
            realized_profit_to_loss_ratio_7d,
            realized_profit_to_loss_ratio_30d,
            realized_profit_to_loss_ratio_1y,

            // === Value Created/Destroyed Splits (stored) ===
            profit_value_created,
            profit_value_destroyed,
            loss_value_created,
            loss_value_destroyed,

            // === Value Created/Destroyed Totals (lazy: profit + loss) ===
            value_created,
            value_destroyed,

            // === Capitulation/Profit Flow (lazy aliases) ===
            capitulation_flow,
            profit_flow,

            // === Adjusted Value (lazy: cohort - up_to_1h) ===
            adjusted_value_created,
            adjusted_value_destroyed,

            // === Value Created/Destroyed Rolling Sums ===
            value_created_24h,
            value_created_7d,
            value_created_30d,
            value_created_1y,
            value_destroyed_24h,
            value_destroyed_7d,
            value_destroyed_30d,
            value_destroyed_1y,

            // === SOPR (rolling window ratios) ===
            sopr,
            sopr_24h,
            sopr_7d,
            sopr_30d,
            sopr_1y,
            sopr_24h_7d_ema,
            sopr_7d_ema,
            sopr_24h_30d_ema,
            sopr_30d_ema,

            // === Adjusted Value Created/Destroyed Rolling Sums ===
            adjusted_value_created_24h,
            adjusted_value_created_7d,
            adjusted_value_created_30d,
            adjusted_value_created_1y,
            adjusted_value_destroyed_24h,
            adjusted_value_destroyed_7d,
            adjusted_value_destroyed_30d,
            adjusted_value_destroyed_1y,

            // === Adjusted SOPR (rolling window ratios) ===
            adjusted_sopr,
            adjusted_sopr_24h,
            adjusted_sopr_7d,
            adjusted_sopr_30d,
            adjusted_sopr_1y,
            adjusted_sopr_24h_7d_ema,
            adjusted_sopr_7d_ema,
            adjusted_sopr_24h_30d_ema,
            adjusted_sopr_30d_ema,

            // === Sell Side Risk Rolling Sum Intermediates ===
            realized_value_24h,
            realized_value_7d,
            realized_value_30d,
            realized_value_1y,

            // === Sell Side Risk (rolling window ratios) ===
            sell_side_risk_ratio,
            sell_side_risk_ratio_24h,
            sell_side_risk_ratio_7d,
            sell_side_risk_ratio_30d,
            sell_side_risk_ratio_1y,
            sell_side_risk_ratio_24h_7d_ema,
            sell_side_risk_ratio_7d_ema,
            sell_side_risk_ratio_24h_30d_ema,
            sell_side_risk_ratio_30d_ema,

            // === Net Realized PnL Deltas ===
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

            // === ATH Regret ===
            peak_regret,
            peak_regret_rel_to_realized_cap,

            // === Sent in Profit/Loss ===
            sent_in_profit: LazyComputedValueFromHeightSumCum::forced_import(
                cfg.db,
                &cfg.name("sent_in_profit"),
                cfg.version,
                cfg.indexes,
                cfg.prices,
            )?,
            sent_in_profit_14d_ema: ValueEmaFromHeight::forced_import(
                cfg.db,
                &cfg.name("sent_in_profit_14d_ema"),
                cfg.version,
                cfg.indexes,
            )?,
            sent_in_loss: LazyComputedValueFromHeightSumCum::forced_import(
                cfg.db,
                &cfg.name("sent_in_loss"),
                cfg.version,
                cfg.indexes,
                cfg.prices,
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
    /// State values are CentsUnsigned (deterministic), converted to Dollars for storage.
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
        // Push raw values for aggregation
        self.cap_raw.truncate_push(height, state.cap_raw())?;
        self.investor_cap_raw
            .truncate_push(height, state.investor_cap_raw())?;
        // Push the 4 splits (totals are derived lazily)
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
        // ATH regret
        self.peak_regret
            .height
            .truncate_push(height, state.peak_regret().to_dollars())?;

        // Volume at profit/loss
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

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.realized_cap_cents.height as &mut dyn AnyStoredVec,
            &mut self.realized_profit.height,
            &mut self.realized_loss.height,
            &mut self.investor_price_cents.height,
            // Raw values for aggregation
            &mut self.cap_raw as &mut dyn AnyStoredVec,
            &mut self.investor_cap_raw as &mut dyn AnyStoredVec,
            // The 4 splits (totals are derived lazily)
            &mut self.profit_value_created.height,
            &mut self.profit_value_destroyed.height,
            &mut self.loss_value_created.height,
            &mut self.loss_value_destroyed.height,
            // ATH regret
            &mut self.peak_regret.height,
            // Sent in profit/loss
            &mut self.sent_in_profit.sats.height,
            &mut self.sent_in_loss.sats.height,
        ]
        .into_par_iter()
    }

    /// Validate computed versions against base version.
    pub(crate) fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        // Validation logic for computed vecs
        Ok(())
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
        // (BytesVec doesn't have compute_sum_of_others, so we manually iterate)
        // Validate version for investor_price_cents (same pattern as compute_sum_of_others)
        let investor_price_dep_version = others
            .iter()
            .map(|o| o.investor_price_cents.height.version())
            .fold(vecdb::Version::ZERO, |acc, v| acc + v);
        self.investor_price_cents
            .height
            .validate_computed_version_or_reset(investor_price_dep_version)?;

        // Start from where the target vecs left off (handles fresh/reset vecs)
        let start = self
            .cap_raw
            .len()
            .min(self.investor_cap_raw.len())
            .min(self.investor_price_cents.height.len());
        // End at the minimum length across all source vecs
        let end = others.iter().map(|o| o.cap_raw.len()).min().unwrap_or(0);

        for i in start..end {
            let height = Height::from(i);

            let mut sum_cap = CentsSats::ZERO;
            let mut sum_investor_cap = CentsSquaredSats::ZERO;

            for o in others.iter() {
                sum_cap += o.cap_raw.collect_one_at(i).unwrap();
                sum_investor_cap += o.investor_cap_raw.collect_one_at(i).unwrap();
            }

            self.cap_raw.truncate_push(height, sum_cap)?;
            self.investor_cap_raw
                .truncate_push(height, sum_investor_cap)?;

            // Compute investor_price from aggregated raw values
            let investor_price = if sum_cap.inner() == 0 {
                Cents::ZERO
            } else {
                Cents::new((sum_investor_cap / sum_cap.inner()) as u64)
            };
            self.investor_price_cents
                .height
                .truncate_push(height, investor_price)?;
        }

        // Write to persist computed_version (same pattern as compute_sum_of_others)
        {
            let _lock = exit.lock();
            self.investor_price_cents.height.write()?;
        }

        // Aggregate the 4 splits (totals are derived lazily)
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
        // ATH regret
        self.peak_regret.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.peak_regret.height)
                .collect::<Vec<_>>(),
            exit,
        )?;

        // Volume at profit/loss
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
        // realized_cap_cents: ComputedFromHeightLast - day1 is lazy, nothing to compute
        // investor_price_cents: ComputedFromHeightLast - day1 is lazy, nothing to compute

        // realized_profit/loss: ComputedFromHeightSumCum - compute cumulative from height
        self.realized_profit
            .compute_cumulative(starting_indexes, exit)?;
        self.realized_loss
            .compute_cumulative(starting_indexes, exit)?;

        // net_realized_pnl = profit - loss
        self.net_realized_pnl
            .compute(starting_indexes, exit, |vec| {
                vec.compute_subtract(
                    starting_indexes.height,
                    &self.realized_profit.height,
                    &self.realized_loss.height,
                    exit,
                )?;
                Ok(())
            })?;

        // realized_value = profit + loss
        // Note: total_realized_pnl is a lazy alias to realized_value since both
        // compute profit + loss with sum aggregation, making them identical.
        // ComputedFromHeightSum: day1 is lazy, just compute the height vec directly
        self.realized_value.height.compute_add(
            starting_indexes.height,
            &self.realized_profit.height,
            &self.realized_loss.height,
            exit,
        )?;

        // Compute derived aggregations for the 4 splits
        // (value_created, value_destroyed, capitulation_flow, profit_flow are derived lazily)
        // ComputedFromHeightSum: day1 is lazy, nothing to compute

        // ATH regret: ComputedFromHeightSumCum - compute cumulative from height
        self.peak_regret
            .compute_cumulative(starting_indexes, exit)?;

        // Volume at profit/loss: LazyComputedValueFromHeightSumCum - compute cumulative
        self.sent_in_profit
            .compute_cumulative(starting_indexes, exit)?;
        self.sent_in_loss
            .compute_cumulative(starting_indexes, exit)?;

        Ok(())
    }

    /// Second phase of computed metrics (realized price from realized cap / supply).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        height_to_market_cap: Option<&impl ReadableVec<Height, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        // realized_price = realized_cap / supply
        self.realized_price.height.compute_divide(
            starting_indexes.height,
            &self.realized_cap.height,
            height_to_supply,
            exit,
        )?;

        self.realized_price_extra.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            Some(&self.realized_price.height),
        )?;

        self.investor_price_extra.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            Some(&self.investor_price.height),
        )?;

        // realized_cap_30d_delta: height-level rolling change
        self.realized_cap_30d_delta.height.compute_rolling_change(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.realized_cap.height,
            exit,
        )?;

        // === Rolling sum intermediates (must be computed before lazy ratios/EMAs that read them) ===
        macro_rules! rolling_sum {
            ($target:expr, $window:expr, $source:expr) => {
                $target.height.compute_rolling_sum(
                    starting_indexes.height, $window, $source, exit,
                )?
            };
        }

        // Value created/destroyed rolling sums (from lazy binary totals)
        rolling_sum!(self.value_created_24h, &blocks.count.height_24h_ago, &self.value_created.height);
        rolling_sum!(self.value_created_7d, &blocks.count.height_1w_ago, &self.value_created.height);
        rolling_sum!(self.value_created_30d, &blocks.count.height_1m_ago, &self.value_created.height);
        rolling_sum!(self.value_created_1y, &blocks.count.height_1y_ago, &self.value_created.height);
        rolling_sum!(self.value_destroyed_24h, &blocks.count.height_24h_ago, &self.value_destroyed.height);
        rolling_sum!(self.value_destroyed_7d, &blocks.count.height_1w_ago, &self.value_destroyed.height);
        rolling_sum!(self.value_destroyed_30d, &blocks.count.height_1m_ago, &self.value_destroyed.height);
        rolling_sum!(self.value_destroyed_1y, &blocks.count.height_1y_ago, &self.value_destroyed.height);

        // Adjusted value created/destroyed rolling sums (from lazy adjusted totals)
        if let Some(source) = self.adjusted_value_created.as_ref() {
            macro_rules! rolling_sum_opt {
                ($target:expr, $window:expr) => {
                    if let Some(f) = $target.as_mut() {
                        f.height.compute_rolling_sum(
                            starting_indexes.height, $window, &source.height, exit,
                        )?;
                    }
                };
            }
            rolling_sum_opt!(self.adjusted_value_created_24h, &blocks.count.height_24h_ago);
            rolling_sum_opt!(self.adjusted_value_created_7d, &blocks.count.height_1w_ago);
            rolling_sum_opt!(self.adjusted_value_created_30d, &blocks.count.height_1m_ago);
            rolling_sum_opt!(self.adjusted_value_created_1y, &blocks.count.height_1y_ago);
        }
        if let Some(source) = self.adjusted_value_destroyed.as_ref() {
            macro_rules! rolling_sum_opt {
                ($target:expr, $window:expr) => {
                    if let Some(f) = $target.as_mut() {
                        f.height.compute_rolling_sum(
                            starting_indexes.height, $window, &source.height, exit,
                        )?;
                    }
                };
            }
            rolling_sum_opt!(self.adjusted_value_destroyed_24h, &blocks.count.height_24h_ago);
            rolling_sum_opt!(self.adjusted_value_destroyed_7d, &blocks.count.height_1w_ago);
            rolling_sum_opt!(self.adjusted_value_destroyed_30d, &blocks.count.height_1m_ago);
            rolling_sum_opt!(self.adjusted_value_destroyed_1y, &blocks.count.height_1y_ago);
        }

        // Realized value rolling sums (for sell_side_risk_ratio)
        rolling_sum!(self.realized_value_24h, &blocks.count.height_24h_ago, &self.realized_value.height);
        rolling_sum!(self.realized_value_7d, &blocks.count.height_1w_ago, &self.realized_value.height);
        rolling_sum!(self.realized_value_30d, &blocks.count.height_1m_ago, &self.realized_value.height);
        rolling_sum!(self.realized_value_1y, &blocks.count.height_1y_ago, &self.realized_value.height);

        // Realized profit/loss rolling sums (for realized_profit_to_loss_ratio)
        if let Some(f) = self.realized_profit_24h.as_mut() {
            f.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_24h_ago, &self.realized_profit.height, exit)?;
        }
        if let Some(f) = self.realized_profit_7d.as_mut() {
            f.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1w_ago, &self.realized_profit.height, exit)?;
        }
        if let Some(f) = self.realized_profit_30d.as_mut() {
            f.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1m_ago, &self.realized_profit.height, exit)?;
        }
        if let Some(f) = self.realized_profit_1y.as_mut() {
            f.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1y_ago, &self.realized_profit.height, exit)?;
        }
        if let Some(f) = self.realized_loss_24h.as_mut() {
            f.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_24h_ago, &self.realized_loss.height, exit)?;
        }
        if let Some(f) = self.realized_loss_7d.as_mut() {
            f.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1w_ago, &self.realized_loss.height, exit)?;
        }
        if let Some(f) = self.realized_loss_30d.as_mut() {
            f.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1m_ago, &self.realized_loss.height, exit)?;
        }
        if let Some(f) = self.realized_loss_1y.as_mut() {
            f.height.compute_rolling_sum(starting_indexes.height, &blocks.count.height_1y_ago, &self.realized_loss.height, exit)?;
        }

        // 7d rolling average of realized profit (height-level)
        self.realized_profit_7d_ema
            .height
            .compute_rolling_average(
                starting_indexes.height,
                &blocks.count.height_1w_ago,
                &self.realized_profit.height,
                exit,
            )?;

        // 7d rolling average of realized loss (height-level)
        self.realized_loss_7d_ema.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.realized_loss.height,
            exit,
        )?;

        // 7d rolling average of net realized PnL (height-level)
        self.net_realized_pnl_7d_ema
            .height
            .compute_rolling_average(
                starting_indexes.height,
                &blocks.count.height_1w_ago,
                &self.net_realized_pnl.height,
                exit,
            )?;

        // 14-day rolling average of sent in profit (sats and dollars)
        self.sent_in_profit_14d_ema.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent_in_profit.sats.height,
            &self.sent_in_profit.usd.height,
            exit,
        )?;

        // 14-day rolling average of sent in loss (sats and dollars)
        self.sent_in_loss_14d_ema.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent_in_loss.sats.height,
            &self.sent_in_loss.usd.height,
            exit,
        )?;

        // 7d/30d rolling average of SOPR (from 24h rolling ratio)
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

        // Optional: adjusted SOPR rolling averages (from 24h rolling ratio)
        if let Some(adjusted_sopr) = self.adjusted_sopr.as_ref() {
            if let Some(ema_7d) = self.adjusted_sopr_24h_7d_ema.as_mut() {
                ema_7d.height.compute_rolling_average(
                    starting_indexes.height,
                    &blocks.count.height_1w_ago,
                    &adjusted_sopr.height,
                    exit,
                )?;
            }

            if let Some(ema_30d) = self.adjusted_sopr_24h_30d_ema.as_mut() {
                ema_30d.height.compute_rolling_average(
                    starting_indexes.height,
                    &blocks.count.height_1m_ago,
                    &adjusted_sopr.height,
                    exit,
                )?;
            }
        }

        // 7d/30d rolling average of sell_side_risk_ratio (from 24h rolling ratio)
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

        // Net realized PnL cumulative 30d delta (height-level rolling change)
        self.net_realized_pnl_cumulative_30d_delta
            .height
            .compute_rolling_change(
                starting_indexes.height,
                &blocks.count.height_1m_ago,
                &*self.net_realized_pnl.rest.height_cumulative,
                exit,
            )?;

        // Relative to realized cap (height-level)
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap
            .height
            .compute_percentage(
                starting_indexes.height,
                &self.net_realized_pnl_cumulative_30d_delta.height,
                &self.realized_cap.height,
                exit,
            )?;

        // Relative to market cap (height-level)
        if let Some(height_to_market_cap) = height_to_market_cap {
            self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap
                .height
                .compute_percentage(
                    starting_indexes.height,
                    &self.net_realized_pnl_cumulative_30d_delta.height,
                    height_to_market_cap,
                    exit,
                )?;

            // Optional: realized_cap_rel_to_own_market_cap
            if let Some(rel_vec) = self.realized_cap_rel_to_own_market_cap.as_mut() {
                rel_vec.height.compute_percentage(
                    starting_indexes.height,
                    &self.realized_cap.height,
                    height_to_market_cap,
                    exit,
                )?;
            }
        }

        Ok(())
    }
}
