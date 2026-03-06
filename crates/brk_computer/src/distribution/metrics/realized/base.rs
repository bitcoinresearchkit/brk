use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, Bitcoin, Cents, CentsSats, CentsSquaredSats, Dollars, Height, Indexes, Version,
};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, BytesVec, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    distribution::state::RealizedState,
    internal::{
        ComputedFromHeight, ComputedFromHeightCumulative, ComputedFromHeightRatio,
        PercentFromHeight, PercentRollingEmas1w1m, PercentRollingWindows, Price, RatioCentsBp32,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedComplete;

/// Full realized metrics (Source/Extended tier).
///
/// Contains all Complete-tier fields (via Deref to RealizedComplete) plus:
/// - Source-only: peak_regret, peak_regret_rel_to_realized_cap
/// - Extended-only: investor_price, price bands, cap_raw, sell_side_risk_ratio
#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub complete: RealizedComplete<M>,

    // --- Extended-only fields ---
    pub investor_price: Price<ComputedFromHeight<Cents, M>>,
    pub investor_price_ratio: ComputedFromHeightRatio<M>,

    pub lower_price_band: Price<ComputedFromHeight<Cents, M>>,
    pub upper_price_band: Price<ComputedFromHeight<Cents, M>>,

    pub cap_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub investor_cap_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    pub sell_side_risk_ratio: PercentRollingWindows<BasisPoints32, M>,
    pub sell_side_risk_ratio_24h_ema: PercentRollingEmas1w1m<BasisPoints32, M>,

    // --- Source-only fields ---
    pub peak_regret: ComputedFromHeightCumulative<Cents, M>,
    pub peak_regret_rel_to_realized_cap: PercentFromHeight<BasisPoints32, M>,
}

impl RealizedFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;

        let complete = RealizedComplete::forced_import(cfg)?;

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

        let peak_regret = cfg.import_cumulative("realized_peak_regret", Version::new(2))?;
        let peak_regret_rel_to_realized_cap =
            cfg.import_percent_bp32("realized_peak_regret_rel_to_realized_cap", Version::new(2))?;

        Ok(Self {
            complete,
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
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.complete
            .min_stateful_height_len()
            .min(self.investor_price.cents.height.len())
            .min(self.cap_raw.len())
            .min(self.investor_cap_raw.len())
            .min(self.peak_regret.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
        self.complete.truncate_push(height, state)?;
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
        let mut vecs = self.complete.collect_vecs_mut();
        vecs.push(&mut self.investor_price.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor_cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.peak_regret.height as &mut dyn AnyStoredVec);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        // Delegate Complete-tier aggregation
        let complete_refs: Vec<&RealizedComplete> =
            others.iter().map(|o| &o.complete).collect();
        self.complete
            .compute_from_stateful(starting_indexes, &complete_refs, exit)?;

        // Aggregate raw values for investor_price computation
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
            self.investor_price
                .cents
                .height
                .truncate_push(height, investor_price)?;
        }

        {
            let _lock = exit.lock();
            self.investor_price.cents.height.write()?;
        }

        // Source-only: peak_regret
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
        self.complete.compute_rest_part1(starting_indexes, exit)?;

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
        // Compute all Complete-tier fields
        self.complete.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_supply,
            height_to_market_cap,
            exit,
        )?;

        // Extended-only: investor_price ratio and price bands
        self.investor_price_ratio.compute_ratio(
            starting_indexes,
            &prices.price.cents.height,
            &self.investor_price.cents.height,
            exit,
        )?;

        self.lower_price_band.cents.height.compute_transform2(
            starting_indexes.height,
            &self.complete.realized_price.cents.height,
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
            &self.complete.realized_price.cents.height,
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

        // Extended-only: sell-side risk ratios
        for (ssrr, rv) in self
            .sell_side_risk_ratio
            .as_mut_array()
            .into_iter()
            .zip(self.complete.gross_pnl_sum.as_array())
        {
            ssrr.compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &rv.height,
                &self.complete.realized_cap_cents.height,
                exit,
            )?;
        }

        // Extended-only: sell side risk EMAs
        self.sell_side_risk_ratio_24h_ema.compute_from_24h(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &blocks.count.height_1m_ago,
            &self.sell_side_risk_ratio._24h.bps.height,
            exit,
        )?;

        // Source-only: peak_regret relative to realized cap
        self.peak_regret_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.peak_regret.height,
                &self.complete.realized_cap_cents.height,
                exit,
            )?;

        Ok(())
    }
}
