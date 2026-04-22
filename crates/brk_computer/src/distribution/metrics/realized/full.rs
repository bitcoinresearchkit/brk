use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, BasisPointsSigned32, Bitcoin, Cents, CentsSats, CentsSigned, CentsSquaredSats,
    Dollars, Height, Indexes, StoredF64, Version,
};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, BytesVec, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    distribution::state::{CohortState, CostBasisData, RealizedState, WithCapital},
    internal::{
        ValuePerBlockCumulativeRolling, FiatPerBlockCumulativeWithSums, PercentPerBlock,
        PercentRollingWindows, PriceWithRatioExtendedPerBlock, RatioCents64, RatioCentsBp32,
        RatioCentsSignedCentsBps32, RatioCentsSignedDollarsBps32, RatioDollarsBp32,
        RatioPerBlockPercentiles, RatioPerBlockStdDevBands, RatioSma, RollingWindows,
        RollingWindowsFrom1w,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedCore;

#[derive(Traversable)]
pub struct RealizedNetPnl<M: StorageMode = Rw> {
    #[traversable(wrap = "change_1m", rename = "to_rcap")]
    pub change_1m_to_rcap: PercentPerBlock<BasisPointsSigned32, M>,
    #[traversable(wrap = "change_1m", rename = "to_mcap")]
    pub change_1m_to_mcap: PercentPerBlock<BasisPointsSigned32, M>,
}

#[derive(Traversable)]
pub struct RealizedSopr<M: StorageMode = Rw> {
    #[traversable(rename = "ratio")]
    pub ratio_extended: RollingWindowsFrom1w<StoredF64, M>,
}

#[derive(Traversable)]
pub struct RealizedPeakRegret<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub value: FiatPerBlockCumulativeWithSums<Cents, M>,
}

#[derive(Traversable)]
pub struct RealizedCapitalized<M: StorageMode = Rw> {
    pub price: PriceWithRatioExtendedPerBlock<M>,
    #[traversable(hidden)]
    pub cap_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,
}

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: RealizedCore<M>,

    pub gross_pnl: FiatPerBlockCumulativeWithSums<Cents, M>,
    pub sell_side_risk_ratio: PercentRollingWindows<BasisPoints32, M>,
    pub net_pnl: RealizedNetPnl<M>,
    pub sopr: RealizedSopr<M>,
    pub peak_regret: RealizedPeakRegret<M>,
    pub capitalized: RealizedCapitalized<M>,

    pub profit_to_loss_ratio: RollingWindows<StoredF64, M>,

    #[traversable(hidden)]
    pub cap_raw: M::Stored<BytesVec<Height, CentsSats>>,
    #[traversable(wrap = "cap", rename = "to_own_mcap")]
    pub cap_to_own_mcap: PercentPerBlock<BasisPoints32, M>,

    #[traversable(wrap = "price", rename = "percentiles")]
    pub price_ratio_percentiles: RatioPerBlockPercentiles<M>,
    #[traversable(wrap = "price", rename = "sma")]
    pub price_ratio_sma: RatioSma<M>,
    #[traversable(wrap = "price", rename = "std_dev")]
    pub price_ratio_std_dev: RatioPerBlockStdDevBands<M>,
}

impl RealizedFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;

        let core = RealizedCore::forced_import(cfg)?;

        // Gross PnL
        let gross_pnl: FiatPerBlockCumulativeWithSums<Cents> =
            cfg.import("realized_gross_pnl", v1)?;
        let sell_side_risk_ratio = cfg.import("sell_side_risk_ratio", Version::new(2))?;

        // Net PnL
        let net_pnl = RealizedNetPnl {
            change_1m_to_rcap: cfg.import("net_pnl_change_1m_to_rcap", Version::new(4))?,
            change_1m_to_mcap: cfg.import("net_pnl_change_1m_to_mcap", Version::new(4))?,
        };

        // SOPR
        let sopr = RealizedSopr {
            ratio_extended: cfg.import("sopr", v1)?,
        };

        // Peak regret
        let peak_regret = RealizedPeakRegret {
            value: cfg.import("realized_peak_regret", Version::new(3))?,
        };

        // Capitalized
        let capitalized = RealizedCapitalized {
            price: cfg.import("capitalized_price", v0)?,
            cap_raw: cfg.import("capitalized_cap_raw", v0)?,
        };

        // Price ratio stats
        let realized_price_name = cfg.name("realized_price");
        let realized_price_version = cfg.version + v1;

        Ok(Self {
            core,
            gross_pnl,
            sell_side_risk_ratio,
            net_pnl,
            sopr,
            peak_regret,
            capitalized,
            profit_to_loss_ratio: cfg.import("realized_profit_to_loss_ratio", v1)?,
            cap_raw: cfg.import("cap_raw", v0)?,
            cap_to_own_mcap: cfg.import("realized_cap_to_own_mcap", v1)?,
            price_ratio_percentiles: RatioPerBlockPercentiles::forced_import(
                cfg.db,
                &realized_price_name,
                realized_price_version,
                cfg.indexes,
            )?,
            price_ratio_sma: RatioSma::forced_import(
                cfg.db,
                &realized_price_name,
                realized_price_version,
                cfg.indexes,
            )?,
            price_ratio_std_dev: RatioPerBlockStdDevBands::forced_import(
                cfg.db,
                &realized_price_name,
                realized_price_version,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.capitalized
            .price
            .cents
            .height
            .len()
            .min(self.cap_raw.len())
            .min(self.capitalized.cap_raw.len())
            .min(self.peak_regret.value.block.cents.len())
    }

    #[inline(always)]
    pub(crate) fn push_state(
        &mut self,
        state: &CohortState<RealizedState, CostBasisData<WithCapital>>,
    ) {
        self.core.push_state(state);
        self.capitalized
            .price
            .cents
            .height
            .push(state.realized.capitalized_price());
        self.cap_raw.push(state.realized.cap_raw());
        self.capitalized
            .cap_raw
            .push(state.realized.capitalized_cap_raw());
        self.peak_regret
            .value
            .block
            .cents
            .push(state.realized.peak_regret());
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(&mut self.capitalized.price.cents.height);
        vecs.push(&mut self.cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.capitalized.cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.peak_regret.value.block.cents);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&RealizedCore],
        exit: &Exit,
    ) -> Result<()> {
        self.core
            .compute_from_stateful(starting_indexes, others, exit)?;

        Ok(())
    }

    #[inline(always)]
    pub(crate) fn push_accum(&mut self, accum: &RealizedFullAccum) {
        self.cap_raw.push(accum.cap_raw);
        self.capitalized.cap_raw.push(accum.capitalized_cap_raw);

        let capitalized_price = {
            let cap = accum.cap_raw.as_u128();
            if cap == 0 {
                Cents::ZERO
            } else {
                Cents::new((accum.capitalized_cap_raw / cap) as u64)
            }
        };
        self.capitalized.price.cents.height.push(capitalized_price);

        self.peak_regret.value.block.cents.push(accum.peak_regret());
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.core.compute_rest_part1(starting_indexes, exit)?;

        self.peak_regret
            .value
            .compute_rest(starting_indexes.height, exit)?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        activity_transfer_volume: &ValuePerBlockCumulativeRolling,
        exit: &Exit,
    ) -> Result<()> {
        self.core.compute_rest_part2(
            prices,
            starting_indexes,
            height_to_supply,
            &activity_transfer_volume.sum._24h.cents.height,
            exit,
        )?;

        // SOPR ratios from lazy rolling sums (1w, 1m, 1y)
        for ((sopr, vc), vd) in self
            .sopr
            .ratio_extended
            .as_mut_array()
            .into_iter()
            .zip(activity_transfer_volume.sum.0.as_array()[1..].iter())
            .zip(self.core.sopr.value_destroyed.sum.as_array()[1..].iter())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &vc.cents.height,
                &vd.height,
                exit,
            )?;
        }

        // Gross PnL
        self.gross_pnl.block.cents.compute_add(
            starting_indexes.height,
            &self.core.minimal.profit.block.cents,
            &self.core.minimal.loss.block.cents,
            exit,
        )?;
        self.gross_pnl.compute_rest(starting_indexes.height, exit)?;

        // Net PnL 1m change relative to rcap and mcap
        self.net_pnl
            .change_1m_to_rcap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.core.net_pnl.delta.absolute._1m.cents.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;
        self.net_pnl
            .change_1m_to_mcap
            .compute_binary::<CentsSigned, Dollars, RatioCentsSignedDollarsBps32>(
                starting_indexes.height,
                &self.core.net_pnl.delta.absolute._1m.cents.height,
                height_to_market_cap,
                exit,
            )?;

        // Capitalized price ratio, percentiles and bands
        self.capitalized
            .price
            .compute_rest(prices, starting_indexes, exit)?;

        // Sell-side risk ratios
        for (ssrr, rv) in self
            .sell_side_risk_ratio
            .as_mut_array()
            .into_iter()
            .zip(self.gross_pnl.sum.as_array())
        {
            ssrr.compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &rv.cents.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;
        }

        // Realized cap relative to own market cap
        self.cap_to_own_mcap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &self.core.minimal.cap.usd.height,
                height_to_market_cap,
                exit,
            )?;

        // Realized profit to loss ratios
        for ((ratio, profit), loss) in self
            .profit_to_loss_ratio
            .as_mut_array()
            .into_iter()
            .zip(self.core.minimal.profit.sum.as_array())
            .zip(self.core.minimal.loss.sum.as_array())
        {
            ratio.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &profit.cents.height,
                &loss.cents.height,
                exit,
            )?;
        }

        // Price ratio: percentiles, sma and std dev bands
        self.price_ratio_percentiles.compute(
            starting_indexes,
            exit,
            &self.core.minimal.price.ratio.height,
            &self.core.minimal.price.cents.height,
        )?;

        self.price_ratio_sma.compute(
            blocks,
            starting_indexes,
            exit,
            &self.core.minimal.price.ratio.height,
        )?;

        self.price_ratio_std_dev.compute(
            blocks,
            starting_indexes,
            exit,
            &self.core.minimal.price.ratio.height,
            &self.core.minimal.price.cents.height,
            &self.price_ratio_sma,
        )?;

        Ok(())
    }
}

#[derive(Default)]
pub struct RealizedFullAccum {
    pub(crate) cap_raw: CentsSats,
    pub(crate) capitalized_cap_raw: CentsSquaredSats,
    peak_regret: CentsSats,
}

impl RealizedFullAccum {
    pub(crate) fn add(&mut self, state: &RealizedState) {
        self.cap_raw += state.cap_raw();
        self.capitalized_cap_raw += state.capitalized_cap_raw();
        self.peak_regret += CentsSats::new(state.peak_regret_raw());
    }

    pub(crate) fn peak_regret(&self) -> Cents {
        self.peak_regret.to_cents()
    }
}
