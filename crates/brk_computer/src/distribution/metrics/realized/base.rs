use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints16, BasisPoints32, BasisPointsSigned16,
    Bitcoin, Cents, CentsSats, CentsSigned, CentsSquaredSats, Dollars, Height, Sats, StoredF32, StoredF64, Version,
};
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Exit, ImportableVec, ReadableCloneableVec,
    ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    ComputeIndexes, blocks,
    distribution::state::RealizedState,
    internal::{
        CentsPlus, CentsUnsignedToDollars, ComputedFromHeightCumulative, ComputedFromHeight,
        ComputedFromHeightRatio, FiatFromHeight, NegCentsUnsignedToDollars, PercentFromHeight,
        PercentRollingEmas1w1m, PercentRollingWindows, ValueFromHeightCumulative, LazyFromHeight,
        Price,
        RatioCentsBp16, RatioCentsSignedCentsBps16, RatioCentsSignedDollarsBps16, RatioCents64,
        RollingEmas1w1m, RollingEmas2w, RollingWindows, Identity,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

/// Base realized metrics (always computed).
#[derive(Traversable)]
pub struct RealizedBase<M: StorageMode = Rw> {
    // === Realized Cap ===
    pub realized_cap_cents: ComputedFromHeight<Cents, M>,
    pub realized_cap: LazyFromHeight<Dollars, Cents>,
    pub realized_price: Price<ComputedFromHeight<Cents, M>>,
    pub realized_price_ratio: ComputedFromHeightRatio<M>,
    pub realized_cap_change_1m: ComputedFromHeight<CentsSigned, M>,

    // === Investor Price ===
    pub investor_price: Price<ComputedFromHeight<Cents, M>>,
    pub investor_price_ratio: ComputedFromHeightRatio<M>,

    // === Floor/Ceiling Price Bands ===
    pub lower_price_band: Price<ComputedFromHeight<Cents, M>>,
    pub upper_price_band: Price<ComputedFromHeight<Cents, M>>,

    // === Raw values for aggregation ===
    pub cap_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub investor_cap_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    // === MVRV ===
    pub mvrv: LazyFromHeight<StoredF32>,

    // === Realized Profit/Loss ===
    pub realized_profit: ComputedFromHeightCumulative<Cents, M>,
    pub realized_profit_ema_1w: ComputedFromHeight<Cents, M>,
    pub realized_loss: ComputedFromHeightCumulative<Cents, M>,
    pub realized_loss_ema_1w: ComputedFromHeight<Cents, M>,
    pub neg_realized_loss: LazyFromHeight<Dollars, Cents>,
    pub net_realized_pnl: ComputedFromHeightCumulative<CentsSigned, M>,
    pub net_realized_pnl_ema_1w: ComputedFromHeight<CentsSigned, M>,
    pub gross_pnl: FiatFromHeight<Cents, M>,

    // === Realized vs Realized Cap Ratios ===
    pub realized_profit_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
    pub realized_loss_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
    pub net_realized_pnl_rel_to_realized_cap: PercentFromHeight<BasisPointsSigned16, M>,

    // === Value Created/Destroyed Splits (stored) ===
    pub profit_value_created: ComputedFromHeight<Cents, M>,
    pub profit_value_destroyed: ComputedFromHeight<Cents, M>,
    pub loss_value_created: ComputedFromHeight<Cents, M>,
    pub loss_value_destroyed: ComputedFromHeight<Cents, M>,

    // === Value Created/Destroyed Totals ===
    pub value_created: ComputedFromHeight<Cents, M>,
    pub value_destroyed: ComputedFromHeight<Cents, M>,

    // === Capitulation/Profit Flow (lazy aliases) ===
    pub capitulation_flow: LazyFromHeight<Dollars, Cents>,
    pub profit_flow: LazyFromHeight<Dollars, Cents>,

    // === Value Created/Destroyed Rolling Sums ===
    pub value_created_sum: RollingWindows<Cents, M>,
    pub value_destroyed_sum: RollingWindows<Cents, M>,

    // === SOPR (rolling window ratios) ===
    pub sopr: RollingWindows<StoredF64, M>,
    pub sopr_24h_ema: RollingEmas1w1m<StoredF64, M>,

    // === Sell Side Risk ===
    pub gross_pnl_sum: RollingWindows<Cents, M>,
    pub sell_side_risk_ratio: PercentRollingWindows<BasisPoints16, M>,
    pub sell_side_risk_ratio_24h_ema: PercentRollingEmas1w1m<BasisPoints16, M>,

    // === Net Realized PnL Deltas ===
    pub net_pnl_change_1m: ComputedFromHeight<CentsSigned, M>,
    pub net_pnl_change_1m_rel_to_realized_cap:
        PercentFromHeight<BasisPointsSigned16, M>,
    pub net_pnl_change_1m_rel_to_market_cap:
        PercentFromHeight<BasisPointsSigned16, M>,

    // === Peak Regret ===
    pub peak_regret: ComputedFromHeightCumulative<Cents, M>,
    pub peak_regret_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,

    // === Sent in Profit/Loss ===
    pub sent_in_profit: ValueFromHeightCumulative<M>,
    pub sent_in_profit_ema: RollingEmas2w<M>,
    pub sent_in_loss: ValueFromHeightCumulative<M>,
    pub sent_in_loss_ema: RollingEmas2w<M>,
}

impl RealizedBase {
    /// Import realized base metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);
        let v3 = Version::new(3);
            // Import combined types using forced_import which handles height + derived
        let realized_cap_cents = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("realized_cap_cents"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_cap = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
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

        let realized_profit_ema_1w = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("realized_profit_ema_1w"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_loss = ComputedFromHeightCumulative::forced_import(
            cfg.db,
            &cfg.name("realized_loss"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_loss_ema_1w = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("realized_loss_ema_1w"),
            cfg.version,
            cfg.indexes,
        )?;

        let neg_realized_loss = LazyFromHeight::from_height_source::<NegCentsUnsignedToDollars>(
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

        let net_realized_pnl_ema_1w = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl_ema_1w"),
            cfg.version,
            cfg.indexes,
        )?;

        let peak_regret = ComputedFromHeightCumulative::forced_import(
            cfg.db,
            &cfg.name("realized_peak_regret"),
            cfg.version + v2,
            cfg.indexes,
        )?;

        let gross_pnl = FiatFromHeight::forced_import(
            cfg.db,
            &cfg.name("gross_pnl"),
            cfg.version,
            cfg.indexes,
        )?;

        let realized_profit_rel_to_realized_cap =
            PercentFromHeight::forced_import_bp16(
                cfg.db,
                &cfg.name("realized_profit_rel_to_realized_cap"),
                cfg.version + v1,
                cfg.indexes,
            )?;

        let realized_loss_rel_to_realized_cap =
            PercentFromHeight::forced_import_bp16(
                cfg.db,
                &cfg.name("realized_loss_rel_to_realized_cap"),
                cfg.version + v1,
                cfg.indexes,
            )?;

        let net_realized_pnl_rel_to_realized_cap =
            PercentFromHeight::forced_import_bps16(
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

        let investor_price = Price::forced_import(
            cfg.db,
            &cfg.name("investor_price"),
            cfg.version,
            cfg.indexes,
        )?;

        let investor_price_ratio = ComputedFromHeightRatio::forced_import(
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

        let profit_value_created = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("profit_value_created"),
            cfg.version,
            cfg.indexes,
        )?;
        let profit_value_destroyed = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("profit_value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;
        let loss_value_created = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("loss_value_created"),
            cfg.version,
            cfg.indexes,
        )?;
        let loss_value_destroyed = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("loss_value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;

        let value_created = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("value_created"),
            cfg.version,
            cfg.indexes,
        )?;
        let value_destroyed = ComputedFromHeight::forced_import(
            cfg.db,
            &cfg.name("value_destroyed"),
            cfg.version,
            cfg.indexes,
        )?;

        let capitulation_flow = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("capitulation_flow"),
            cfg.version,
            loss_value_destroyed.height.read_only_boxed_clone(),
            &loss_value_destroyed,
        );
        let profit_flow = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("profit_flow"),
            cfg.version,
            profit_value_destroyed.height.read_only_boxed_clone(),
            &profit_value_destroyed,
        );

        let realized_price_ratio = ComputedFromHeightRatio::forced_import(
            cfg.db,
            &cfg.name("realized_price"),
            cfg.version + v1,
            cfg.indexes,
        )?;

        let mvrv = LazyFromHeight::from_lazy::<Identity<StoredF32>, BasisPoints32>(
            &cfg.name("mvrv"),
            cfg.version,
            &realized_price_ratio.ratio,
        );

        // === Rolling windows ===
        let value_created_sum = RollingWindows::forced_import(
            cfg.db, &cfg.name("value_created"), cfg.version + v1, cfg.indexes,
        )?;
        let value_destroyed_sum = RollingWindows::forced_import(
            cfg.db, &cfg.name("value_destroyed"), cfg.version + v1, cfg.indexes,
        )?;
        let gross_pnl_sum = RollingWindows::forced_import(
            cfg.db, &cfg.name("gross_pnl_sum"), cfg.version + v1, cfg.indexes,
        )?;
        let sopr = RollingWindows::forced_import(
            cfg.db, &cfg.name("sopr"), cfg.version + v1, cfg.indexes,
        )?;
        let sell_side_risk_ratio = PercentRollingWindows::forced_import_bp16(
            cfg.db, &cfg.name("sell_side_risk_ratio"), cfg.version + v1, cfg.indexes,
        )?;

        // === EMA imports ===
        let sopr_24h_ema = RollingEmas1w1m::forced_import(
            cfg.db, &cfg.name("sopr_24h"), cfg.version + v1, cfg.indexes,
        )?;
        let sell_side_risk_ratio_24h_ema = PercentRollingEmas1w1m::forced_import_bp16(
            cfg.db, &cfg.name("sell_side_risk_ratio_24h"), cfg.version + v1, cfg.indexes,
        )?;

        let peak_regret_rel_to_realized_cap =
            PercentFromHeight::forced_import_bp16(
                cfg.db,
                &cfg.name("realized_peak_regret_rel_to_realized_cap"),
                cfg.version + v1,
                cfg.indexes,
            )?;

        Ok(Self {
            realized_cap_cents,
            realized_cap,
            realized_price,
            realized_price_ratio,
            realized_cap_change_1m: ComputedFromHeight::forced_import(
                cfg.db,
                &cfg.name("realized_cap_change_1m"),
                cfg.version,
                cfg.indexes,
            )?,
            investor_price,
            investor_price_ratio,
            lower_price_band,
            upper_price_band,
            cap_raw,
            investor_cap_raw,
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
            profit_value_created,
            profit_value_destroyed,
            loss_value_created,
            loss_value_destroyed,
            value_created,
            value_destroyed,
            capitulation_flow,
            profit_flow,
            value_created_sum,
            value_destroyed_sum,
            sopr,
            sopr_24h_ema,
            gross_pnl_sum,
            sell_side_risk_ratio,
            sell_side_risk_ratio_24h_ema,
            net_pnl_change_1m: ComputedFromHeight::forced_import(
                cfg.db,
                &cfg.name("net_pnl_change_1m"),
                cfg.version + v3,
                cfg.indexes,
            )?,
            net_pnl_change_1m_rel_to_realized_cap:
                PercentFromHeight::forced_import_bps16(
                    cfg.db,
                    &cfg.name("net_pnl_change_1m_rel_to_realized_cap"),
                    cfg.version + v3,
                    cfg.indexes,
                )?,
            net_pnl_change_1m_rel_to_market_cap:
                PercentFromHeight::forced_import_bps16(
                    cfg.db,
                    &cfg.name("net_pnl_change_1m_rel_to_market_cap"),
                    cfg.version + v3,
                    cfg.indexes,
                )?,
            peak_regret,
            peak_regret_rel_to_realized_cap,
            sent_in_profit: ValueFromHeightCumulative::forced_import(
                cfg.db,
                &cfg.name("sent_in_profit"),
                cfg.version,
                cfg.indexes,
            )?,
            sent_in_profit_ema: RollingEmas2w::forced_import(
                cfg.db,
                &cfg.name("sent_in_profit"),
                cfg.version,
                cfg.indexes,
            )?,
            sent_in_loss: ValueFromHeightCumulative::forced_import(
                cfg.db,
                &cfg.name("sent_in_loss"),
                cfg.version,
                cfg.indexes,
            )?,
            sent_in_loss_ema: RollingEmas2w::forced_import(
                cfg.db,
                &cfg.name("sent_in_loss"),
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
            .min(self.investor_price.cents.height.len())
            .min(self.cap_raw.len())
            .min(self.investor_cap_raw.len())
            .min(self.profit_value_created.height.len())
            .min(self.profit_value_destroyed.height.len())
            .min(self.loss_value_created.height.len())
            .min(self.loss_value_destroyed.height.len())
            .min(self.peak_regret.height.len())
            .min(self.sent_in_profit.base.sats.height.len())
            .min(self.sent_in_loss.base.sats.height.len())
    }

    /// Push realized state values to height-indexed vectors.
    pub(crate) fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
        self.realized_cap_cents
            .height
            .truncate_push(height, state.cap())?;
        self.realized_profit
            .height
            .truncate_push(height, state.profit())?;
        self.realized_loss
            .height
            .truncate_push(height, state.loss())?;
        self.investor_price.cents
            .height
            .truncate_push(height, state.investor_price())?;
        self.cap_raw.truncate_push(height, state.cap_raw())?;
        self.investor_cap_raw
            .truncate_push(height, state.investor_cap_raw())?;
        self.profit_value_created
            .height
            .truncate_push(height, state.profit_value_created())?;
        self.profit_value_destroyed
            .height
            .truncate_push(height, state.profit_value_destroyed())?;
        self.loss_value_created
            .height
            .truncate_push(height, state.loss_value_created())?;
        self.loss_value_destroyed
            .height
            .truncate_push(height, state.loss_value_destroyed())?;
        self.peak_regret
            .height
            .truncate_push(height, state.peak_regret())?;
        self.sent_in_profit
            .base
            .sats
            .height
            .truncate_push(height, state.sent_in_profit())?;
        self.sent_in_loss
            .base
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
            &mut self.investor_price.cents.height,
            &mut self.cap_raw as &mut dyn AnyStoredVec,
            &mut self.investor_cap_raw as &mut dyn AnyStoredVec,
            &mut self.profit_value_created.height,
            &mut self.profit_value_destroyed.height,
            &mut self.loss_value_created.height,
            &mut self.loss_value_destroyed.height,
            &mut self.peak_regret.height,
            &mut self.sent_in_profit.base.sats.height,
            &mut self.sent_in_loss.base.sats.height,
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
            .map(|o| o.investor_price.cents.height.version())
            .fold(vecdb::Version::ZERO, |acc, v| acc + v);
        self.investor_price.cents
            .height
            .validate_computed_version_or_reset(investor_price_dep_version)?;

        let start = self
            .cap_raw
            .len()
            .min(self.investor_cap_raw.len())
            .min(self.investor_price.cents.height.len());
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
            self.investor_price.cents
                .height
                .truncate_push(height, investor_price)?;
        }

        {
            let _lock = exit.lock();
            self.investor_price.cents.height.write()?;
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
        self.sent_in_profit.base.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent_in_profit.base.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.sent_in_loss.base.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent_in_loss.base.sats.height)
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
                vec.compute_transform2(
                    starting_indexes.height,
                    &self.realized_profit.height,
                    &self.realized_loss.height,
                    |(i, profit, loss, ..)| {
                        (i, CentsSigned::new(profit.inner() as i64 - loss.inner() as i64))
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

        self.investor_price_ratio.compute_ratio(
            starting_indexes,
            &prices.price.cents.height,
            &self.investor_price.cents.height,
            exit,
        )?;

        self.lower_price_band.cents.height.compute_transform2(
            starting_indexes.height,
            &self.realized_price.cents.height,
            &self.investor_price.cents.height,
            |(i, rp, ip, ..)| {
                let rp = rp.as_u128();
                let ip = ip.as_u128();
                if ip == 0 {
                    (i, Cents::ZERO)
                } else {
                    (i, Cents::from(rp * rp / ip))
                }
            },
            exit,
        )?;

        self.upper_price_band.cents.height.compute_transform2(
            starting_indexes.height,
            &self.investor_price.cents.height,
            &self.realized_price.cents.height,
            |(i, ip, rp, ..)| {
                let ip = ip.as_u128();
                let rp = rp.as_u128();
                if rp == 0 {
                    (i, Cents::ZERO)
                } else {
                    (i, Cents::from(ip * ip / rp))
                }
            },
            exit,
        )?;

        self.realized_cap_change_1m.height.compute_rolling_change(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.realized_cap_cents.height,
            exit,
        )?;

        // Compute value_created/destroyed from stored components
        self.value_created
            .compute_binary::<Cents, Cents, CentsPlus>(
                starting_indexes.height,
                &self.profit_value_created.height,
                &self.loss_value_created.height,
                exit,
            )?;
        self.value_destroyed
            .compute_binary::<Cents, Cents, CentsPlus>(
                starting_indexes.height,
                &self.profit_value_destroyed.height,
                &self.loss_value_destroyed.height,
                exit,
            )?;

        // === Rolling sum intermediates ===
        let window_starts = blocks.count.window_starts();
        self.value_created_sum.compute_rolling_sum(
            starting_indexes.height, &window_starts, &self.value_created.height, exit,
        )?;
        self.value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height, &window_starts, &self.value_destroyed.height, exit,
        )?;
        self.gross_pnl_sum.compute_rolling_sum(
            starting_indexes.height, &window_starts, &self.gross_pnl.cents.height, exit,
        )?;

        // Compute SOPR from rolling sums
        for ((sopr, vc), vd) in self.sopr.as_mut_array().into_iter()
            .zip(self.value_created_sum.as_array())
            .zip(self.value_destroyed_sum.as_array())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height, &vc.height, &vd.height, exit,
            )?;
        }

        // Compute sell-side risk ratios
        for (ssrr, rv) in self.sell_side_risk_ratio.as_mut_array().into_iter()
            .zip(self.gross_pnl_sum.as_array())
        {
            ssrr.compute_binary::<Cents, Cents, RatioCentsBp16>(
                starting_indexes.height, &rv.height, &self.realized_cap_cents.height, exit,
            )?;
        }

        // 7d EMAs
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
        self.net_realized_pnl_ema_1w
            .height
            .compute_rolling_ema(
                starting_indexes.height,
                &blocks.count.height_1w_ago,
                &self.net_realized_pnl.height,
                exit,
            )?;

        // 14-day EMA of sent in profit/loss
        self.sent_in_profit_ema.compute(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent_in_profit.base.sats.height,
            &self.sent_in_profit.base.cents.height,
            exit,
        )?;
        self.sent_in_loss_ema.compute(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent_in_loss.base.sats.height,
            &self.sent_in_loss.base.cents.height,
            exit,
        )?;

        // SOPR EMAs (based on 24h window)
        self.sopr_24h_ema.compute_from_24h(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &blocks.count.height_1m_ago,
            &self.sopr._24h.height,
            exit,
        )?;

        // Sell side risk EMAs (based on 24h window)
        self.sell_side_risk_ratio_24h_ema.compute_from_24h(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &blocks.count.height_1m_ago,
            &self.sell_side_risk_ratio._24h.bps.height,
            exit,
        )?;

        // Realized profit/loss/net relative to realized cap
        self.realized_profit_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp16>(
                starting_indexes.height,
                &self.realized_profit.height,
                &self.realized_cap_cents.height,
                exit,
            )?;
        self.realized_loss_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp16>(
                starting_indexes.height,
                &self.realized_loss.height,
                &self.realized_cap_cents.height,
                exit,
            )?;
        self.net_realized_pnl_rel_to_realized_cap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps16>(
                starting_indexes.height,
                &self.net_realized_pnl.height,
                &self.realized_cap_cents.height,
                exit,
            )?;
        self.peak_regret_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp16>(
                starting_indexes.height,
                &self.peak_regret.height,
                &self.realized_cap_cents.height,
                exit,
            )?;

        // Net realized PnL cumulative 30d delta
        self.net_pnl_change_1m
            .height
            .compute_rolling_change(
                starting_indexes.height,
                &blocks.count.height_1m_ago,
                &self.net_realized_pnl.cumulative.height,
                exit,
            )?;

        self.net_pnl_change_1m_rel_to_realized_cap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps16>(
                starting_indexes.height,
                &self.net_pnl_change_1m.height,
                &self.realized_cap_cents.height,
                exit,
            )?;

        self.net_pnl_change_1m_rel_to_market_cap
            .compute_binary::<CentsSigned, Dollars, RatioCentsSignedDollarsBps16>(
                starting_indexes.height,
                &self.net_pnl_change_1m.height,
                height_to_market_cap,
                exit,
            )?;

        Ok(())
    }
}
