use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, BasisPointsSigned32, Bitcoin, Cents, CentsSats, CentsSigned, CentsSquaredSats,
    Dollars, Height, Indexes, Version,
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
        CentsUnsignedToDollars, ComputedFromHeight, ComputedFromHeightCumulative,
        ComputedFromHeightRatio, ComputedFromHeightRatioPercentiles, LazyFromHeight,
        PercentFromHeight, PercentRollingEmas1w1m, PercentRollingWindows, Price, RatioCentsBp32,
        RatioCentsSignedCentsBps32, RatioCentsSignedDollarsBps32, RollingEmas2w, RollingWindows,
        ValueFromHeightCumulative,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedCore;

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedBasic<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: RealizedCore<M>,

    // --- Stateful fields ---
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

    pub sent_in_profit: ValueFromHeightCumulative<M>,
    pub sent_in_profit_ema: RollingEmas2w<M>,
    pub sent_in_loss: ValueFromHeightCumulative<M>,
    pub sent_in_loss_ema: RollingEmas2w<M>,

    // --- Investor price & price bands ---
    pub investor_price: Price<ComputedFromHeight<Cents, M>>,
    pub investor_price_ratio: ComputedFromHeightRatio<M>,

    pub lower_price_band: Price<ComputedFromHeight<Cents, M>>,
    pub upper_price_band: Price<ComputedFromHeight<Cents, M>>,

    pub cap_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub investor_cap_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    pub sell_side_risk_ratio: PercentRollingWindows<BasisPoints32, M>,
    pub sell_side_risk_ratio_24h_ema: PercentRollingEmas1w1m<BasisPoints32, M>,

    // --- Peak regret ---
    pub peak_regret: ComputedFromHeightCumulative<Cents, M>,
    pub peak_regret_rel_to_realized_cap: PercentFromHeight<BasisPoints32, M>,

    // --- Realized price ratio percentiles ---
    pub realized_price_ratio_percentiles: ComputedFromHeightRatioPercentiles<M>,
}

impl RealizedBasic {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;

        let core = RealizedCore::forced_import(cfg)?;

        // Stateful fields
        let profit_value_created = cfg.import_computed("profit_value_created", v0)?;
        let profit_value_destroyed = cfg.import_computed("profit_value_destroyed", v0)?;
        let loss_value_created = cfg.import_computed("loss_value_created", v0)?;
        let loss_value_destroyed = cfg.import_computed("loss_value_destroyed", v0)?;

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

        let gross_pnl_sum = cfg.import_rolling("gross_pnl_sum", v1)?;

        // Investor price & price bands
        let investor_price = cfg.import_price("investor_price", v0)?;
        let investor_price_ratio = cfg.import_ratio("investor_price", v0)?;
        let lower_price_band = cfg.import_price("lower_price_band", v0)?;
        let upper_price_band = cfg.import_price("upper_price_band", v0)?;

        let cap_raw = cfg.import_bytes("cap_raw", v0)?;
        let investor_cap_raw = cfg.import_bytes("investor_cap_raw", v0)?;

        let sell_side_risk_ratio =
            cfg.import_percent_rolling_bp32("sell_side_risk_ratio", Version::new(2))?;
        let sell_side_risk_ratio_24h_ema =
            cfg.import_percent_emas_1w_1m_bp32("sell_side_risk_ratio_24h", Version::new(2))?;

        // Peak regret
        let peak_regret = cfg.import_cumulative("realized_peak_regret", Version::new(2))?;
        let peak_regret_rel_to_realized_cap =
            cfg.import_percent_bp32("realized_peak_regret_rel_to_realized_cap", Version::new(2))?;

        // Realized price ratio percentiles
        let realized_price_ratio_percentiles =
            ComputedFromHeightRatioPercentiles::forced_import(
                cfg.db,
                &cfg.name("realized_price"),
                cfg.version + v1,
                cfg.indexes,
            )?;

        Ok(Self {
            core,
            profit_value_created,
            profit_value_destroyed,
            loss_value_created,
            loss_value_destroyed,
            capitulation_flow,
            profit_flow,
            gross_pnl_sum,
            net_pnl_change_1m: cfg.import_computed("net_pnl_change_1m", Version::new(3))?,
            net_pnl_change_1m_rel_to_realized_cap: cfg
                .import_percent_bps32("net_pnl_change_1m_rel_to_realized_cap", Version::new(4))?,
            net_pnl_change_1m_rel_to_market_cap: cfg
                .import_percent_bps32("net_pnl_change_1m_rel_to_market_cap", Version::new(4))?,
            sent_in_profit: cfg.import_value_cumulative("sent_in_profit", v0)?,
            sent_in_profit_ema: cfg.import_emas_2w("sent_in_profit", v0)?,
            sent_in_loss: cfg.import_value_cumulative("sent_in_loss", v0)?,
            sent_in_loss_ema: cfg.import_emas_2w("sent_in_loss", v0)?,
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
            realized_price_ratio_percentiles,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.core
            .min_stateful_height_len()
            .min(self.profit_value_created.height.len())
            .min(self.profit_value_destroyed.height.len())
            .min(self.loss_value_created.height.len())
            .min(self.loss_value_destroyed.height.len())
            .min(self.sent_in_profit.base.sats.height.len())
            .min(self.sent_in_loss.base.sats.height.len())
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
        vecs.push(&mut self.sent_in_profit.base.sats.height);
        vecs.push(&mut self.sent_in_loss.base.sats.height);
        vecs.push(&mut self.investor_price.cents.height);
        vecs.push(&mut self.cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor_cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.peak_regret.height);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        // Core aggregation
        let core_refs: Vec<&RealizedCore> = others.iter().map(|o| &o.core).collect();
        self.core
            .compute_from_stateful(starting_indexes, &core_refs, exit)?;

        // Stateful field aggregation
        sum_others!(self, starting_indexes, others, exit; profit_value_created.height);
        sum_others!(self, starting_indexes, others, exit; profit_value_destroyed.height);
        sum_others!(self, starting_indexes, others, exit; loss_value_created.height);
        sum_others!(self, starting_indexes, others, exit; loss_value_destroyed.height);
        sum_others!(self, starting_indexes, others, exit; sent_in_profit.base.sats.height);
        sum_others!(self, starting_indexes, others, exit; sent_in_loss.base.sats.height);

        // Investor price aggregation from raw values
        let investor_price_dep_version = others
            .iter()
            .map(|o| o.investor_price.cents.height.version())
            .fold(vecdb::Version::ZERO, |acc, v| acc + v);
        self.investor_price
            .cents
            .height
            .validate_computed_version_or_reset(investor_price_dep_version)?;

        let start = self
            .cap_raw
            .len()
            .min(self.investor_cap_raw.len())
            .min(self.investor_price.cents.height.len());
        let end = others.iter().map(|o| o.cap_raw.len()).min().unwrap_or(0);

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
            self.investor_price
                .cents
                .height
                .truncate_push(height, investor_price)?;
        }

        {
            let _lock = exit.lock();
            self.investor_price.cents.height.write()?;
        }

        // Peak regret aggregation
        self.peak_regret.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.peak_regret.height)
                .collect::<Vec<_>>(),
            exit,
        )?;

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
        // Core computation
        self.core.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_supply,
            exit,
        )?;

        // Gross PnL rolling sums
        let window_starts = blocks.count.window_starts();
        self.gross_pnl_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.core.gross_pnl.cents.height,
            exit,
        )?;

        // Sent in profit/loss EMAs
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
                &self.core.realized_cap_cents.height,
                exit,
            )?;

        self.net_pnl_change_1m_rel_to_market_cap
            .compute_binary::<CentsSigned, Dollars, RatioCentsSignedDollarsBps32>(
                starting_indexes.height,
                &self.net_pnl_change_1m.height,
                height_to_market_cap,
                exit,
            )?;

        // Investor price ratio and price bands
        self.investor_price_ratio.compute_ratio(
            starting_indexes,
            &prices.price.cents.height,
            &self.investor_price.cents.height,
            exit,
        )?;

        self.lower_price_band.cents.height.compute_transform2(
            starting_indexes.height,
            &self.core.realized_price.cents.height,
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
            &self.core.realized_price.cents.height,
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
                &self.core.realized_cap_cents.height,
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

        // Peak regret relative to realized cap
        self.peak_regret_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.peak_regret.height,
                &self.core.realized_cap_cents.height,
                exit,
            )?;

        // Realized price ratio percentiles
        self.realized_price_ratio_percentiles.compute(
            blocks,
            starting_indexes,
            exit,
            &self.core.realized_price_ratio.ratio.height,
            &self.core.realized_price.cents.height,
        )?;

        Ok(())
    }
}
