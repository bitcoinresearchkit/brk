use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, BasisPointsSigned32, Bitcoin, Cents, CentsSats, CentsSigned, CentsSquaredSats,
    Dollars, Height, Indexes, StoredF64, Version,
};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode,
    WritableVec,
};

use crate::{
    blocks,
    distribution::state::RealizedState,
    internal::{
        CentsUnsignedToDollars, ComputedFromHeight, ComputedFromHeightCumulative, FiatFromHeight,
        ComputedFromHeightRatio, ComputedFromHeightRatioPercentiles,
        ComputedFromHeightRatioStdDevBands, LazyFromHeight, PercentFromHeight,
        PercentRollingEmas1w1m, PercentRollingWindows, Price, RatioCents64, RatioCentsBp32,
        RatioCentsSignedCentsBps32, RatioCentsSignedDollarsBps32, RatioDollarsBp32,
        RollingEmas1w1m, RollingEmas2w, RollingWindows,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedBase;

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: RealizedBase<M>,

    pub gross_pnl: FiatFromHeight<Cents, M>,

    pub realized_profit_rel_to_realized_cap: PercentFromHeight<BasisPoints32, M>,
    pub realized_loss_rel_to_realized_cap: PercentFromHeight<BasisPoints32, M>,
    pub net_realized_pnl_rel_to_realized_cap: PercentFromHeight<BasisPointsSigned32, M>,

    pub profit_value_created: ComputedFromHeight<Cents, M>,
    pub profit_value_destroyed: ComputedFromHeight<Cents, M>,
    pub loss_value_created: ComputedFromHeight<Cents, M>,
    pub loss_value_destroyed: ComputedFromHeight<Cents, M>,

    pub capitulation_flow: LazyFromHeight<Dollars, Cents>,
    pub profit_flow: LazyFromHeight<Dollars, Cents>,

    pub gross_pnl_sum: RollingWindows<Cents, M>,

    pub net_pnl_change_1m: ComputedFromHeight<CentsSigned, M>,
    pub net_pnl_change_1m_rel_to_realized_cap: PercentFromHeight<BasisPointsSigned32, M>,
    pub net_pnl_change_1m_rel_to_market_cap: PercentFromHeight<BasisPointsSigned32, M>,

    pub investor_price: Price<ComputedFromHeight<Cents, M>>,
    pub investor_price_ratio: ComputedFromHeightRatio<M>,

    pub lower_price_band: Price<ComputedFromHeight<Cents, M>>,
    pub upper_price_band: Price<ComputedFromHeight<Cents, M>>,

    pub cap_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub investor_cap_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    pub sell_side_risk_ratio: PercentRollingWindows<BasisPoints32, M>,
    pub sell_side_risk_ratio_24h_ema: PercentRollingEmas1w1m<BasisPoints32, M>,

    pub peak_regret: ComputedFromHeightCumulative<Cents, M>,
    pub peak_regret_rel_to_realized_cap: PercentFromHeight<BasisPoints32, M>,

    pub realized_cap_rel_to_own_market_cap: PercentFromHeight<BasisPoints32, M>,

    pub realized_profit_sum: RollingWindows<Cents, M>,
    pub realized_loss_sum: RollingWindows<Cents, M>,
    pub realized_profit_to_loss_ratio: RollingWindows<StoredF64, M>,

    pub realized_profit_ema_1w: ComputedFromHeight<Cents, M>,
    pub realized_loss_ema_1w: ComputedFromHeight<Cents, M>,
    pub net_realized_pnl_ema_1w: ComputedFromHeight<CentsSigned, M>,

    pub sopr_24h_ema: RollingEmas1w1m<StoredF64, M>,

    pub sent_in_profit_ema: RollingEmas2w<M>,
    pub sent_in_loss_ema: RollingEmas2w<M>,

    pub realized_price_ratio_percentiles: ComputedFromHeightRatioPercentiles<M>,
    pub realized_price_ratio_std_dev: ComputedFromHeightRatioStdDevBands<M>,
    pub investor_price_ratio_percentiles: ComputedFromHeightRatioPercentiles<M>,
}

impl RealizedFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;

        let core = RealizedBase::forced_import(cfg)?;

        let gross_pnl = cfg.import("realized_gross_pnl", v0)?;

        let profit_value_created = cfg.import("profit_value_created", v0)?;
        let profit_value_destroyed: ComputedFromHeight<Cents> =
            cfg.import("profit_value_destroyed", v0)?;
        let loss_value_created = cfg.import("loss_value_created", v0)?;
        let loss_value_destroyed: ComputedFromHeight<Cents> =
            cfg.import("loss_value_destroyed", v0)?;

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

        let gross_pnl_sum = cfg.import("gross_pnl_sum", Version::ONE)?;

        let investor_price = cfg.import("investor_price", v0)?;
        let investor_price_ratio = cfg.import("investor_price", v0)?;
        let lower_price_band = cfg.import("lower_price_band", v0)?;
        let upper_price_band = cfg.import("upper_price_band", v0)?;

        let cap_raw = cfg.import("cap_raw", v0)?;
        let investor_cap_raw = cfg.import("investor_cap_raw", v0)?;

        let sell_side_risk_ratio =
            cfg.import("sell_side_risk_ratio", Version::new(2))?;
        let sell_side_risk_ratio_24h_ema =
            cfg.import("sell_side_risk_ratio_24h", Version::new(2))?;

        let peak_regret = cfg.import("realized_peak_regret", Version::new(2))?;
        let peak_regret_rel_to_realized_cap =
            cfg.import("realized_peak_regret_rel_to_realized_cap", Version::new(2))?;

        let realized_price_name = cfg.name("realized_price");
        let realized_price_version = cfg.version + v1;
        let investor_price_name = cfg.name("investor_price");
        let investor_price_version = cfg.version;

        let realized_profit_rel_to_realized_cap =
            cfg.import("realized_profit_rel_to_realized_cap", Version::new(2))?;
        let realized_loss_rel_to_realized_cap =
            cfg.import("realized_loss_rel_to_realized_cap", Version::new(2))?;
        let net_realized_pnl_rel_to_realized_cap =
            cfg.import("net_realized_pnl_rel_to_realized_cap", Version::new(2))?;

        let realized_profit_ema_1w = cfg.import("realized_profit_ema_1w", v0)?;
        let realized_loss_ema_1w = cfg.import("realized_loss_ema_1w", v0)?;
        let net_realized_pnl_ema_1w = cfg.import("net_realized_pnl_ema_1w", v0)?;
        let sopr_24h_ema = cfg.import("sopr_24h", v1)?;
        let sent_in_profit_ema = cfg.import("sent_in_profit", v0)?;
        let sent_in_loss_ema = cfg.import("sent_in_loss", v0)?;

        Ok(Self {
            core,
            gross_pnl,
            realized_profit_rel_to_realized_cap,
            realized_loss_rel_to_realized_cap,
            net_realized_pnl_rel_to_realized_cap,
            profit_value_created,
            profit_value_destroyed,
            loss_value_created,
            loss_value_destroyed,
            capitulation_flow,
            profit_flow,
            gross_pnl_sum,
            net_pnl_change_1m: cfg.import("net_pnl_change_1m", Version::new(3))?,
            net_pnl_change_1m_rel_to_realized_cap: cfg
                .import("net_pnl_change_1m_rel_to_realized_cap", Version::new(4))?,
            net_pnl_change_1m_rel_to_market_cap: cfg
                .import("net_pnl_change_1m_rel_to_market_cap", Version::new(4))?,
            investor_price,
            investor_price_ratio,
            lower_price_band,
            upper_price_band,
            cap_raw,
            investor_cap_raw,
            sell_side_risk_ratio,
            sell_side_risk_ratio_24h_ema,
            peak_regret,
            peak_regret_rel_to_realized_cap,
            realized_cap_rel_to_own_market_cap: cfg
                .import("realized_cap_rel_to_own_market_cap", v1)?,
            realized_profit_sum: cfg.import("realized_profit", v1)?,
            realized_loss_sum: cfg.import("realized_loss", v1)?,
            realized_profit_to_loss_ratio: cfg
                .import("realized_profit_to_loss_ratio", v1)?,
            realized_profit_ema_1w,
            realized_loss_ema_1w,
            net_realized_pnl_ema_1w,
            sopr_24h_ema,
            sent_in_profit_ema,
            sent_in_loss_ema,
            realized_price_ratio_percentiles: ComputedFromHeightRatioPercentiles::forced_import(
                cfg.db,
                &realized_price_name,
                realized_price_version,
                cfg.indexes,
            )?,
            realized_price_ratio_std_dev: ComputedFromHeightRatioStdDevBands::forced_import(
                cfg.db,
                &realized_price_name,
                realized_price_version,
                cfg.indexes,
            )?,
            investor_price_ratio_percentiles: ComputedFromHeightRatioPercentiles::forced_import(
                cfg.db,
                &investor_price_name,
                investor_price_version,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.core
            .min_stateful_height_len()
            .min(self.profit_value_created.height.len())
            .min(self.profit_value_destroyed.height.len())
            .min(self.loss_value_created.height.len())
            .min(self.loss_value_destroyed.height.len())
            .min(self.investor_price.cents.height.len())
            .min(self.cap_raw.len())
            .min(self.investor_cap_raw.len())
            .min(self.peak_regret.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
        self.core.truncate_push(height, state)?;
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
        self.investor_price
            .cents
            .height
            .truncate_push(height, state.investor_price())?;
        self.cap_raw.truncate_push(height, state.cap_raw())?;
        self.investor_cap_raw
            .truncate_push(height, state.investor_cap_raw())?;
        self.peak_regret
            .height
            .truncate_push(height, state.peak_regret())?;

        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(&mut self.profit_value_created.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.profit_value_destroyed.height);
        vecs.push(&mut self.loss_value_created.height);
        vecs.push(&mut self.loss_value_destroyed.height);
        vecs.push(&mut self.investor_price.cents.height);
        vecs.push(&mut self.cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor_cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.peak_regret.height);
        vecs
    }

    /// Aggregate Core-level fields from source cohorts.
    /// investor_price, cap_raw, investor_cap_raw come from the stateful scan, not aggregated.
    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&RealizedBase],
        exit: &Exit,
    ) -> Result<()> {
        self.core
            .compute_from_stateful(starting_indexes, others, exit)?;

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.core.compute_rest_part1(starting_indexes, exit)?;
        self.peak_regret
            .compute_rest(starting_indexes.height, exit)?;
        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.core.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_supply,
            exit,
        )?;

        // Realized P/L rel to realized cap
        self.realized_profit_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.core.minimal.realized_profit.height,
                &self.core.minimal.realized_cap_cents.height,
                exit,
            )?;
        self.realized_loss_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.core.minimal.realized_loss.height,
                &self.core.minimal.realized_cap_cents.height,
                exit,
            )?;
        self.net_realized_pnl_rel_to_realized_cap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.core.net_realized_pnl.height,
                &self.core.minimal.realized_cap_cents.height,
                exit,
            )?;

        // EMAs
        self.realized_profit_ema_1w.height.compute_rolling_ema(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.core.minimal.realized_profit.height,
            exit,
        )?;
        self.realized_loss_ema_1w.height.compute_rolling_ema(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.core.minimal.realized_loss.height,
            exit,
        )?;
        self.net_realized_pnl_ema_1w.height.compute_rolling_ema(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.core.net_realized_pnl.height,
            exit,
        )?;
        self.sopr_24h_ema.compute_from_24h(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &blocks.count.height_1m_ago,
            &self.core.sopr._24h.height,
            exit,
        )?;
        self.sent_in_profit_ema.compute(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.core.sent_in_profit.base.sats.height,
            &self.core.sent_in_profit.base.cents.height,
            exit,
        )?;
        self.sent_in_loss_ema.compute(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.core.sent_in_loss.base.sats.height,
            &self.core.sent_in_loss.base.cents.height,
            exit,
        )?;

        // Gross PnL
        self.gross_pnl.cents.height.compute_add(
            starting_indexes.height,
            &self.core.minimal.realized_profit.height,
            &self.core.minimal.realized_loss.height,
            exit,
        )?;

        let window_starts = blocks.count.window_starts();
        self.gross_pnl_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.gross_pnl.cents.height,
            exit,
        )?;

        // Net PnL change 1m
        self.net_pnl_change_1m.height.compute_rolling_change(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.core.net_realized_pnl.cumulative.height,
            exit,
        )?;
        self.net_pnl_change_1m_rel_to_realized_cap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.net_pnl_change_1m.height,
                &self.core.minimal.realized_cap_cents.height,
                exit,
            )?;
        self.net_pnl_change_1m_rel_to_market_cap
            .compute_binary::<CentsSigned, Dollars, RatioCentsSignedDollarsBps32>(
                starting_indexes.height,
                &self.net_pnl_change_1m.height,
                height_to_market_cap,
                exit,
            )?;

        // Peak regret
        self.peak_regret_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.peak_regret.height,
                &self.core.minimal.realized_cap_cents.height,
                exit,
            )?;

        // Investor price ratio and price bands
        self.investor_price_ratio.compute_ratio(
            starting_indexes,
            &prices.price.cents.height,
            &self.investor_price.cents.height,
            exit,
        )?;

        // Use explicit field paths for split borrows
        self.lower_price_band.cents.height.compute_transform2(
            starting_indexes.height,
            &self.core.minimal.realized_price.cents.height,
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
            &self.core.minimal.realized_price.cents.height,
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

        // Sell-side risk ratios
        for (ssrr, rv) in self
            .sell_side_risk_ratio
            .as_mut_array()
            .into_iter()
            .zip(self.gross_pnl_sum.as_array())
        {
            ssrr.compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &rv.height,
                &self.core.minimal.realized_cap_cents.height,
                exit,
            )?;
        }

        self.sell_side_risk_ratio_24h_ema.compute_from_24h(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &blocks.count.height_1m_ago,
            &self.sell_side_risk_ratio._24h.bps.height,
            exit,
        )?;

        // Extended: realized profit/loss rolling sums
        let window_starts = blocks.count.window_starts();
        self.realized_profit_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.core.minimal.realized_profit.height,
            exit,
        )?;
        self.realized_loss_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.core.minimal.realized_loss.height,
            exit,
        )?;

        // Realized cap relative to own market cap
        self.realized_cap_rel_to_own_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &self.core.minimal.realized_cap.height,
                height_to_market_cap,
                exit,
            )?;

        // Realized profit to loss ratios
        for ((ratio, profit), loss) in self
            .realized_profit_to_loss_ratio
            .as_mut_array()
            .into_iter()
            .zip(self.realized_profit_sum.as_array())
            .zip(self.realized_loss_sum.as_array())
        {
            ratio.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &profit.height,
                &loss.height,
                exit,
            )?;
        }

        self.realized_price_ratio_percentiles.compute(
            blocks,
            starting_indexes,
            exit,
            &self.core.minimal.realized_price_ratio.ratio.height,
            &self.core.minimal.realized_price.cents.height,
        )?;

        self.realized_price_ratio_std_dev.compute(
            blocks,
            starting_indexes,
            exit,
            &self.core.minimal.realized_price_ratio.ratio.height,
            &self.core.minimal.realized_price.cents.height,
        )?;

        // Investor price: percentiles
        let investor_price = &self.investor_price.cents.height;
        self.investor_price_ratio_percentiles.compute(
            blocks,
            starting_indexes,
            exit,
            &self.investor_price_ratio.ratio.height,
            investor_price,
        )?;

        Ok(())
    }
}
