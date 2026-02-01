use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, CentsSats, CentsSquaredSats, CentsUnsigned, DateIndex, Dollars, Height, StoredF32,
    StoredF64, Version,
};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, EagerVec, Exit, GenericStoredVec, Ident, ImportableVec,
    IterableCloneableVec, IterableVec, Negate, PcoVec, TypedVecIterator,
};

use crate::{
    ComputeIndexes,
    distribution::state::RealizedState,
    indexes,
    internal::{
        CentsUnsignedToDollars, ComputedFromDateLast, ComputedFromDateRatio,
        ComputedFromHeightLast, ComputedFromHeightSum, ComputedFromHeightSumCum, DollarsMinus,
        DollarsPlus, LazyBinaryFromHeightSum, LazyBinaryFromHeightSumCum,
        LazyComputedValueFromHeightSumCum, LazyFromDateLast, LazyFromHeightLast, LazyFromHeightSum,
        LazyFromHeightSumCum, LazyPriceFromCents, PercentageDollarsF32, PriceFromHeight,
        StoredF32Identity, ValueFromDateLast,
    },
    price,
};

use super::ImportConfig;

/// Realized cap and related metrics.
#[derive(Clone, Traversable)]
pub struct RealizedMetrics {
    // === Realized Cap ===
    pub realized_cap_cents: ComputedFromHeightLast<CentsUnsigned>,
    pub realized_cap: LazyFromHeightLast<Dollars, CentsUnsigned>,
    pub realized_price: PriceFromHeight,
    pub realized_price_extra: ComputedFromDateRatio,
    pub realized_cap_rel_to_own_market_cap: Option<ComputedFromHeightLast<StoredF32>>,
    pub realized_cap_30d_delta: ComputedFromDateLast<Dollars>,

    // === Investor Price (dollar-weighted average acquisition price) ===
    pub investor_price_cents: ComputedFromHeightLast<CentsUnsigned>,
    pub investor_price: LazyPriceFromCents,
    pub investor_price_extra: ComputedFromDateRatio,

    // === Raw values for aggregation (needed to compute investor_price for aggregated cohorts) ===
    /// Raw Σ(price × sats) for realized cap aggregation
    pub cap_raw: BytesVec<Height, CentsSats>,
    /// Raw Σ(price² × sats) for investor_price aggregation
    pub investor_cap_raw: BytesVec<Height, CentsSquaredSats>,

    // === MVRV (Market Value to Realized Value) ===
    // Proxy for realized_price_extra.ratio (close / realized_price = market_cap / realized_cap)
    pub mvrv: LazyFromDateLast<StoredF32>,

    // === Realized Profit/Loss ===
    pub realized_profit: ComputedFromHeightSumCum<Dollars>,
    pub realized_profit_7d_ema: ComputedFromDateLast<Dollars>,
    pub realized_loss: ComputedFromHeightSumCum<Dollars>,
    pub realized_loss_7d_ema: ComputedFromDateLast<Dollars>,
    pub neg_realized_loss: LazyFromHeightSumCum<Dollars>,
    pub net_realized_pnl: ComputedFromHeightSumCum<Dollars>,
    pub net_realized_pnl_7d_ema: ComputedFromDateLast<Dollars>,
    pub realized_value: ComputedFromHeightSum<Dollars>,

    // === Realized vs Realized Cap Ratios (lazy) ===
    pub realized_profit_rel_to_realized_cap:
        LazyBinaryFromHeightSumCum<StoredF32, Dollars, Dollars>,
    pub realized_loss_rel_to_realized_cap: LazyBinaryFromHeightSumCum<StoredF32, Dollars, Dollars>,
    pub net_realized_pnl_rel_to_realized_cap:
        LazyBinaryFromHeightSumCum<StoredF32, Dollars, Dollars>,

    // === Total Realized PnL ===
    pub total_realized_pnl: LazyFromHeightSum<Dollars>,
    pub realized_profit_to_loss_ratio: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,

    // === Value Created/Destroyed Splits (stored) ===
    pub profit_value_created: ComputedFromHeightSum<Dollars>,
    pub profit_value_destroyed: ComputedFromHeightSum<Dollars>,
    pub loss_value_created: ComputedFromHeightSum<Dollars>,
    pub loss_value_destroyed: ComputedFromHeightSum<Dollars>,

    // === Value Created/Destroyed Totals (lazy: profit + loss) ===
    pub value_created: LazyBinaryFromHeightSum<Dollars, Dollars, Dollars>,
    pub value_destroyed: LazyBinaryFromHeightSum<Dollars, Dollars, Dollars>,

    // === Capitulation/Profit Flow (lazy aliases) ===
    pub capitulation_flow: LazyFromHeightSum<Dollars>,
    pub profit_flow: LazyFromHeightSum<Dollars>,

    // === Adjusted Value (lazy: cohort - up_to_1h) ===
    pub adjusted_value_created: Option<LazyBinaryFromHeightSum<Dollars, Dollars, Dollars>>,
    pub adjusted_value_destroyed: Option<LazyBinaryFromHeightSum<Dollars, Dollars, Dollars>>,

    // === SOPR (Spent Output Profit Ratio) ===
    pub sopr: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub sopr_7d_ema: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub sopr_30d_ema: EagerVec<PcoVec<DateIndex, StoredF64>>,
    pub adjusted_sopr: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub adjusted_sopr_7d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,
    pub adjusted_sopr_30d_ema: Option<EagerVec<PcoVec<DateIndex, StoredF64>>>,

    // === Sell Side Risk ===
    pub sell_side_risk_ratio: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub sell_side_risk_ratio_7d_ema: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub sell_side_risk_ratio_30d_ema: EagerVec<PcoVec<DateIndex, StoredF32>>,

    // === Net Realized PnL Deltas ===
    pub net_realized_pnl_cumulative_30d_delta: ComputedFromDateLast<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: ComputedFromDateLast<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: ComputedFromDateLast<StoredF32>,

    // === Peak Regret ===
    /// Realized peak regret: Σ((peak - sell_price) × sats)
    /// where peak = max price during holding period.
    /// "How much more could have been made by selling at peak instead"
    pub peak_regret: ComputedFromHeightSumCum<Dollars>,
    /// Peak regret as % of realized cap
    pub peak_regret_rel_to_realized_cap: LazyBinaryFromHeightSum<StoredF32, Dollars, Dollars>,

    // === Sent in Profit/Loss ===
    /// Sats sent in profit (sats/btc/usd)
    pub sent_in_profit: LazyComputedValueFromHeightSumCum,
    /// 14-day EMA of sent in profit (sats, btc, usd)
    pub sent_in_profit_14d_ema: ValueFromDateLast,
    /// Sats sent in loss (sats/btc/usd)
    pub sent_in_loss: LazyComputedValueFromHeightSumCum,
    /// 14-day EMA of sent in loss (sats, btc, usd)
    pub sent_in_loss_14d_ema: ValueFromDateLast,
}

impl RealizedMetrics {
    /// Import realized metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
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
            realized_cap_cents.height.boxed_clone(),
            &realized_cap_cents,
        );

        let realized_profit = ComputedFromHeightSumCum::forced_import(
            cfg.db,
            &cfg.name("realized_profit"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_profit_7d_ema = ComputedFromDateLast::forced_import(
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

        let realized_loss_7d_ema = ComputedFromDateLast::forced_import(
            cfg.db,
            &cfg.name("realized_loss_7d_ema"),
            cfg.version,
            cfg.indexes,
        )?;

        let neg_realized_loss = LazyFromHeightSumCum::from_computed::<Negate>(
            &cfg.name("neg_realized_loss"),
            cfg.version + v1,
            realized_loss.height.boxed_clone(),
            &realized_loss,
        );

        let net_realized_pnl = ComputedFromHeightSumCum::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl"),
            cfg.version,
            cfg.indexes,
        )?;

        let net_realized_pnl_7d_ema = ComputedFromDateLast::forced_import(
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
            realized_value.height.boxed_clone(),
            &realized_value,
        );

        // Construct lazy ratio vecs
        let realized_profit_rel_to_realized_cap =
            LazyBinaryFromHeightSumCum::from_computed_lazy_last::<PercentageDollarsF32, _>(
                &cfg.name("realized_profit_rel_to_realized_cap"),
                cfg.version + v1,
                realized_profit.height.boxed_clone(),
                realized_cap.height.boxed_clone(),
                &realized_profit,
                &realized_cap,
            );

        let realized_loss_rel_to_realized_cap =
            LazyBinaryFromHeightSumCum::from_computed_lazy_last::<PercentageDollarsF32, _>(
                &cfg.name("realized_loss_rel_to_realized_cap"),
                cfg.version + v1,
                realized_loss.height.boxed_clone(),
                realized_cap.height.boxed_clone(),
                &realized_loss,
                &realized_cap,
            );

        let net_realized_pnl_rel_to_realized_cap =
            LazyBinaryFromHeightSumCum::from_computed_lazy_last::<PercentageDollarsF32, _>(
                &cfg.name("net_realized_pnl_rel_to_realized_cap"),
                cfg.version + v1,
                net_realized_pnl.height.boxed_clone(),
                realized_cap.height.boxed_clone(),
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

        let investor_price = LazyPriceFromCents::from_computed(
            &cfg.name("investor_price"),
            cfg.version,
            &investor_price_cents,
        );

        let investor_price_extra = ComputedFromDateRatio::forced_import_from_lazy(
            cfg.db,
            &cfg.name("investor_price"),
            &investor_price.dollars,
            cfg.version,
            cfg.indexes,
            extended,
        )?;

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
            loss_value_destroyed.height.boxed_clone(),
            &loss_value_destroyed,
        );

        let profit_flow = LazyFromHeightSum::from_computed::<Ident>(
            &cfg.name("profit_flow"),
            cfg.version,
            profit_value_destroyed.height.boxed_clone(),
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
        let realized_price_extra = ComputedFromDateRatio::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            Some(&realized_price),
            cfg.version + v1,
            cfg.indexes,
            extended,
        )?;

        // MVRV is a lazy proxy for realized_price_extra.ratio
        // ratio = close / realized_price = market_cap / realized_cap = MVRV
        let mvrv = LazyFromDateLast::from_source::<StoredF32Identity>(
            &cfg.name("mvrv"),
            cfg.version,
            &realized_price_extra.ratio,
        );

        Ok(Self {
            // === Realized Cap ===
            realized_cap_cents,
            realized_cap: realized_cap.clone(),
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
            realized_cap_30d_delta: ComputedFromDateLast::forced_import(
                cfg.db,
                &cfg.name("realized_cap_30d_delta"),
                cfg.version,
                cfg.indexes,
            )?,

            // === Investor Price ===
            investor_price_cents,
            investor_price,
            investor_price_extra,
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
            realized_profit_to_loss_ratio: extended
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("realized_profit_to_loss_ratio"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,

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

            // === SOPR ===
            sopr: EagerVec::forced_import(cfg.db, &cfg.name("sopr"), cfg.version + v1)?,
            sopr_7d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sopr_7d_ema"),
                cfg.version + v1,
            )?,
            sopr_30d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sopr_30d_ema"),
                cfg.version + v1,
            )?,
            adjusted_sopr: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(cfg.db, &cfg.name("adjusted_sopr"), cfg.version + v1)
                })
                .transpose()?,
            adjusted_sopr_7d_ema: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_sopr_7d_ema"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,
            adjusted_sopr_30d_ema: compute_adjusted
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("adjusted_sopr_30d_ema"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,

            // === Sell Side Risk ===
            sell_side_risk_ratio: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sell_side_risk_ratio"),
                cfg.version + v1,
            )?,
            sell_side_risk_ratio_7d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sell_side_risk_ratio_7d_ema"),
                cfg.version + v1,
            )?,
            sell_side_risk_ratio_30d_ema: EagerVec::forced_import(
                cfg.db,
                &cfg.name("sell_side_risk_ratio_30d_ema"),
                cfg.version + v1,
            )?,

            // === Net Realized PnL Deltas ===
            net_realized_pnl_cumulative_30d_delta: ComputedFromDateLast::forced_import(
                cfg.db,
                &cfg.name("net_realized_pnl_cumulative_30d_delta"),
                cfg.version + v3,
                cfg.indexes,
            )?,
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap:
                ComputedFromDateLast::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap"),
                    cfg.version + v3,
                    cfg.indexes,
                )?,
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap:
                ComputedFromDateLast::forced_import(
                    cfg.db,
                    &cfg.name("net_realized_pnl_cumulative_30d_delta_rel_to_market_cap"),
                    cfg.version + v3,
                    cfg.indexes,
                )?,

            // === ATH Regret ===
            peak_regret: peak_regret.clone(),
            peak_regret_rel_to_realized_cap: LazyBinaryFromHeightSum::from_sumcum_lazy_last::<
                PercentageDollarsF32,
                _,
            >(
                &cfg.name("peak_regret_rel_to_realized_cap"),
                cfg.version + v1,
                peak_regret.height.boxed_clone(),
                realized_cap.height.boxed_clone(),
                &peak_regret,
                &realized_cap,
            ),

            // === Sent in Profit/Loss ===
            sent_in_profit: LazyComputedValueFromHeightSumCum::forced_import(
                cfg.db,
                &cfg.name("sent_in_profit"),
                cfg.version,
                cfg.indexes,
                cfg.price,
            )?,
            sent_in_profit_14d_ema: ValueFromDateLast::forced_import(
                cfg.db,
                &cfg.name("sent_in_profit_14d_ema"),
                cfg.version,
                cfg.compute_dollars(),
                cfg.indexes,
            )?,
            sent_in_loss: LazyComputedValueFromHeightSumCum::forced_import(
                cfg.db,
                &cfg.name("sent_in_loss"),
                cfg.version,
                cfg.indexes,
                cfg.price,
            )?,
            sent_in_loss_14d_ema: ValueFromDateLast::forced_import(
                cfg.db,
                &cfg.name("sent_in_loss_14d_ema"),
                cfg.version,
                cfg.compute_dollars(),
                cfg.indexes,
            )?,
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub fn min_stateful_height_len(&self) -> usize {
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
    pub fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
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
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
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
    pub fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        // Validation logic for computed vecs
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
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

        let mut iters: Vec<_> = others
            .iter()
            .filter_map(|o| Some((o.cap_raw.iter().ok()?, o.investor_cap_raw.iter().ok()?)))
            .collect();

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

            for (cap_iter, investor_cap_iter) in &mut iters {
                sum_cap += cap_iter.get_unwrap(height);
                sum_investor_cap += investor_cap_iter.get_unwrap(height);
            }

            self.cap_raw.truncate_push(height, sum_cap)?;
            self.investor_cap_raw
                .truncate_push(height, sum_investor_cap)?;

            // Compute investor_price from aggregated raw values
            let investor_price = if sum_cap.inner() == 0 {
                CentsUnsigned::ZERO
            } else {
                CentsUnsigned::new((sum_investor_cap / sum_cap.inner()) as u64)
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
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.realized_cap_cents
            .compute_rest(indexes, starting_indexes, exit)?;
        self.realized_profit
            .compute_rest(indexes, starting_indexes, exit)?;
        self.realized_loss
            .compute_rest(indexes, starting_indexes, exit)?;
        self.investor_price_cents
            .compute_rest(indexes, starting_indexes, exit)?;

        // net_realized_pnl = profit - loss
        self.net_realized_pnl
            .compute_all(indexes, starting_indexes, exit, |vec| {
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
        self.realized_value
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_add(
                    starting_indexes.height,
                    &self.realized_profit.height,
                    &self.realized_loss.height,
                    exit,
                )?;
                Ok(())
            })?;

        // Compute derived aggregations for the 4 splits
        // (value_created, value_destroyed, capitulation_flow, profit_flow are derived lazily)
        self.profit_value_created
            .compute_rest(indexes, starting_indexes, exit)?;
        self.profit_value_destroyed
            .compute_rest(indexes, starting_indexes, exit)?;
        self.loss_value_created
            .compute_rest(indexes, starting_indexes, exit)?;
        self.loss_value_destroyed
            .compute_rest(indexes, starting_indexes, exit)?;
        // ATH regret
        self.peak_regret
            .compute_rest(indexes, starting_indexes, exit)?;

        // Volume at profit/loss
        self.sent_in_profit
            .compute_rest(indexes, starting_indexes, exit)?;
        self.sent_in_loss
            .compute_rest(indexes, starting_indexes, exit)?;

        Ok(())
    }

    /// Second phase of computed metrics (realized price from realized cap / supply).
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        // realized_price = realized_cap / supply
        self.realized_price
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    &self.realized_cap.height,
                    height_to_supply,
                    exit,
                )?;
                Ok(())
            })?;

        if let Some(price) = price {
            self.realized_price_extra.compute_rest(
                price,
                starting_indexes,
                exit,
                Some(&self.realized_price.dateindex.0),
            )?;

            self.investor_price_extra.compute_rest(
                price,
                starting_indexes,
                exit,
                Some(&self.investor_price.dateindex.0),
            )?;
        }

        // realized_cap_30d_delta
        self.realized_cap_30d_delta
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_change(
                    starting_indexes.dateindex,
                    &self.realized_cap.dateindex.0,
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        // SOPR = value_created / value_destroyed
        self.sopr.compute_divide(
            starting_indexes.dateindex,
            &self.value_created.dateindex.0,
            &self.value_destroyed.dateindex.0,
            exit,
        )?;

        // 7d EMA of realized profit/loss
        self.realized_profit_7d_ema.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_ema(
                starting_indexes.dateindex,
                &self.realized_profit.dateindex.sum.0,
                7,
                exit,
            )?)
        })?;

        self.realized_loss_7d_ema.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_ema(
                starting_indexes.dateindex,
                &self.realized_loss.dateindex.sum.0,
                7,
                exit,
            )?)
        })?;

        self.net_realized_pnl_7d_ema.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_ema(
                starting_indexes.dateindex,
                &self.net_realized_pnl.dateindex.sum.0,
                7,
                exit,
            )?)
        })?;

        // 14-day EMA of sent in profit (sats and dollars)
        self.sent_in_profit_14d_ema.compute_ema(
            starting_indexes.dateindex,
            &self.sent_in_profit.sats.dateindex.sum.0,
            self.sent_in_profit.dollars.as_ref().map(|d| &d.dateindex.sum.0),
            14,
            exit,
        )?;

        // 14-day EMA of sent in loss (sats and dollars)
        self.sent_in_loss_14d_ema.compute_ema(
            starting_indexes.dateindex,
            &self.sent_in_loss.sats.dateindex.sum.0,
            self.sent_in_loss.dollars.as_ref().map(|d| &d.dateindex.sum.0),
            14,
            exit,
        )?;

        self.sopr_7d_ema
            .compute_ema(starting_indexes.dateindex, &self.sopr, 7, exit)?;

        self.sopr_30d_ema
            .compute_ema(starting_indexes.dateindex, &self.sopr, 30, exit)?;

        // Optional: adjusted SOPR (lazy: cohort - up_to_1h)
        if let (Some(adjusted_sopr), Some(adj_created), Some(adj_destroyed)) = (
            self.adjusted_sopr.as_mut(),
            self.adjusted_value_created.as_ref(),
            self.adjusted_value_destroyed.as_ref(),
        ) {
            adjusted_sopr.compute_divide(
                starting_indexes.dateindex,
                &*adj_created.dateindex,
                &*adj_destroyed.dateindex,
                exit,
            )?;

            if let Some(ema_7d) = self.adjusted_sopr_7d_ema.as_mut() {
                ema_7d.compute_ema(
                    starting_indexes.dateindex,
                    self.adjusted_sopr.as_ref().unwrap(),
                    7,
                    exit,
                )?;
            }

            if let Some(ema_30d) = self.adjusted_sopr_30d_ema.as_mut() {
                ema_30d.compute_ema(
                    starting_indexes.dateindex,
                    self.adjusted_sopr.as_ref().unwrap(),
                    30,
                    exit,
                )?;
            }
        }

        // sell_side_risk_ratio = realized_value / realized_cap
        self.sell_side_risk_ratio.compute_percentage(
            starting_indexes.dateindex,
            &self.realized_value.dateindex.0,
            &self.realized_cap.dateindex.0,
            exit,
        )?;

        self.sell_side_risk_ratio_7d_ema.compute_ema(
            starting_indexes.dateindex,
            &self.sell_side_risk_ratio,
            7,
            exit,
        )?;

        self.sell_side_risk_ratio_30d_ema.compute_ema(
            starting_indexes.dateindex,
            &self.sell_side_risk_ratio,
            30,
            exit,
        )?;

        // Net realized PnL cumulative 30d delta
        self.net_realized_pnl_cumulative_30d_delta
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_change(
                    starting_indexes.dateindex,
                    &self.net_realized_pnl.dateindex.cumulative.0,
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        // Relative to realized cap
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    &self.net_realized_pnl_cumulative_30d_delta.dateindex,
                    &self.realized_cap.dateindex.0,
                    exit,
                )?;
                Ok(())
            })?;

        // Relative to market cap
        if let Some(dateindex_to_market_cap) = dateindex_to_market_cap {
            self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        &self.net_realized_pnl_cumulative_30d_delta.dateindex,
                        dateindex_to_market_cap,
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        // Optional: realized_cap_rel_to_own_market_cap
        if let (Some(rel_vec), Some(height_to_market_cap)) = (
            self.realized_cap_rel_to_own_market_cap.as_mut(),
            height_to_market_cap,
        ) {
            rel_vec.compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.height,
                    &self.realized_cap.height,
                    height_to_market_cap,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // Optional: realized_profit_to_loss_ratio
        if let Some(ratio) = self.realized_profit_to_loss_ratio.as_mut() {
            ratio.compute_divide(
                starting_indexes.dateindex,
                &self.realized_profit.dateindex.sum.0,
                &self.realized_loss.dateindex.sum.0,
                exit,
            )?;
        }

        Ok(())
    }
}
