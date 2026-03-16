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
    distribution::state::{WithCapital, CohortState, CostBasisData, RealizedState},
    internal::{
        CentsUnsignedToDollars, PerBlock, PerBlockCumulative,
        PerBlockCumulativeWithSums, FiatPerBlockCumulativeWithSums,
        LazyPerBlock, PercentPerBlock, PercentRollingWindows, Price,
        PriceWithRatioExtendedPerBlock, RatioCents64, RatioCentsBp32,
        RatioCentsSignedCentsBps32, RatioCentsSignedDollarsBps32, RatioDollarsBp32,
        RatioPerBlockPercentiles, RatioPerBlockStdDevBands, RatioSma, RollingWindows,
        RollingWindowsFrom1w,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedCore;

#[derive(Traversable)]
pub struct RealizedProfit<M: StorageMode = Rw> {
    pub to_rcap: PercentPerBlock<BasisPoints32, M>,
    pub value_created: PerBlockCumulativeWithSums<Cents, Cents, M>,
    pub value_destroyed: PerBlockCumulativeWithSums<Cents, Cents, M>,
    pub distribution_flow: LazyPerBlock<Dollars, Cents>,
}

#[derive(Traversable)]
pub struct RealizedLoss<M: StorageMode = Rw> {
    pub to_rcap: PercentPerBlock<BasisPoints32, M>,
    pub value_created: PerBlockCumulativeWithSums<Cents, Cents, M>,
    pub value_destroyed: PerBlockCumulativeWithSums<Cents, Cents, M>,
    pub capitulation_flow: LazyPerBlock<Dollars, Cents>,
}

#[derive(Traversable)]
pub struct RealizedNetPnl<M: StorageMode = Rw> {
    pub to_rcap: PercentPerBlock<BasisPointsSigned32, M>,
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
    pub value: PerBlockCumulative<Cents, M>,
    pub to_rcap: PercentPerBlock<BasisPoints32, M>,
}

#[derive(Traversable)]
pub struct RealizedInvestor<M: StorageMode = Rw> {
    pub price: PriceWithRatioExtendedPerBlock<M>,
    pub investor_lower_band: Price<PerBlock<Cents, M>>,
    pub investor_upper_band: Price<PerBlock<Cents, M>>,
    #[traversable(hidden)]
    pub cap_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,
}

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: RealizedCore<M>,

    pub profit: RealizedProfit<M>,
    pub loss: RealizedLoss<M>,
    pub gross_pnl: FiatPerBlockCumulativeWithSums<Cents, M>,
    pub sell_side_risk_ratio: PercentRollingWindows<BasisPoints32, M>,
    pub net_pnl: RealizedNetPnl<M>,
    pub sopr: RealizedSopr<M>,
    pub peak_regret: RealizedPeakRegret<M>,
    pub investor: RealizedInvestor<M>,

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

        // Profit
        let profit_value_destroyed: PerBlockCumulativeWithSums<Cents, Cents> =
            cfg.import("profit_value_destroyed", v1)?;
        let profit_flow = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("distribution_flow"),
            cfg.version,
            profit_value_destroyed.base.height.read_only_boxed_clone(),
            &profit_value_destroyed.base,
        );
        let profit = RealizedProfit {
            to_rcap: cfg.import("realized_profit_to_rcap", Version::new(2))?,
            value_created: cfg.import("profit_value_created", v1)?,
            value_destroyed: profit_value_destroyed,
            distribution_flow: profit_flow,
        };

        // Loss
        let loss_value_destroyed: PerBlockCumulativeWithSums<Cents, Cents> =
            cfg.import("loss_value_destroyed", v1)?;
        let capitulation_flow = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("capitulation_flow"),
            cfg.version,
            loss_value_destroyed.base.height.read_only_boxed_clone(),
            &loss_value_destroyed.base,
        );
        let loss = RealizedLoss {
            to_rcap: cfg.import("realized_loss_to_rcap", Version::new(2))?,
            value_created: cfg.import("loss_value_created", v1)?,
            value_destroyed: loss_value_destroyed,
            capitulation_flow,
        };

        // Gross PnL
        let gross_pnl: FiatPerBlockCumulativeWithSums<Cents> =
            cfg.import("realized_gross_pnl", v1)?;
        let sell_side_risk_ratio = cfg.import("sell_side_risk_ratio", Version::new(2))?;

        // Net PnL
        let net_pnl = RealizedNetPnl {
            to_rcap: cfg
                .import("net_realized_pnl_to_rcap", Version::new(2))?,
            change_1m_to_rcap: cfg
                .import("net_pnl_change_1m_to_rcap", Version::new(4))?,
            change_1m_to_mcap: cfg
                .import("net_pnl_change_1m_to_mcap", Version::new(4))?,
        };

        // SOPR
        let sopr = RealizedSopr {
            ratio_extended: cfg.import("sopr", v1)?,
        };

        // Peak regret
        let peak_regret = RealizedPeakRegret {
            value: cfg.import("realized_peak_regret", Version::new(2))?,
            to_rcap: cfg
                .import("realized_peak_regret_to_rcap", Version::new(2))?,
        };

        // Investor
        let investor = RealizedInvestor {
            price: cfg.import("investor_price", v0)?,
            investor_lower_band: cfg.import("investor_lower_band", v0)?,
            investor_upper_band: cfg.import("investor_upper_band", v0)?,
            cap_raw: cfg.import("investor_cap_raw", v0)?,
        };

        // Price ratio stats
        let realized_price_name = cfg.name("realized_price");
        let realized_price_version = cfg.version + v1;

        Ok(Self {
            core,
            profit,
            loss,
            gross_pnl,
            sell_side_risk_ratio,
            net_pnl,
            sopr,
            peak_regret,
            investor,
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
        self.profit
            .value_created
            .base
            .height
            .len()
            .min(self.profit.value_destroyed.base.height.len())
            .min(self.loss.value_created.base.height.len())
            .min(self.loss.value_destroyed.base.height.len())
            .min(self.investor.price.cents.height.len())
            .min(self.cap_raw.len())
            .min(self.investor.cap_raw.len())
            .min(self.peak_regret.value.base.height.len())
    }

    #[inline(always)]
    pub(crate) fn push_state(
        &mut self,
        state: &CohortState<RealizedState, CostBasisData<WithCapital>>,
    ) {
        self.core.push_state(state);
        self.profit
            .value_created
            .base
            .height
            .push(state.realized.profit_value_created());
        self.profit
            .value_destroyed
            .base
            .height
            .push(state.realized.profit_value_destroyed());
        self.loss
            .value_created
            .base
            .height
            .push(state.realized.loss_value_created());
        self.loss
            .value_destroyed
            .base
            .height
            .push(state.realized.loss_value_destroyed());
        self.investor
            .price
            .cents
            .height
            .push(state.realized.investor_price());
        self.cap_raw
            .push(state.realized.cap_raw());
        self.investor
            .cap_raw
            .push(state.realized.investor_cap_raw());
        self.peak_regret
            .value
            .base
            .height
            .push(state.realized.peak_regret());
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(&mut self.profit.value_created.base.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.profit.value_destroyed.base.height);
        vecs.push(&mut self.loss.value_created.base.height);
        vecs.push(&mut self.loss.value_destroyed.base.height);
        vecs.push(&mut self.investor.price.cents.height);
        vecs.push(&mut self.cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor.cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.peak_regret.value.base.height);
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
    pub(crate) fn push_accum(
        &mut self,
        accum: &RealizedFullAccum,
    ) {
        self.profit
            .value_created
            .base
            .height
            .push(accum.profit_value_created());
        self.profit
            .value_destroyed
            .base
            .height
            .push(accum.profit_value_destroyed());
        self.loss
            .value_created
            .base
            .height
            .push(accum.loss_value_created());
        self.loss
            .value_destroyed
            .base
            .height
            .push(accum.loss_value_destroyed());
        self.cap_raw
            .push(accum.cap_raw);
        self.investor
            .cap_raw
            .push(accum.investor_cap_raw);

        let investor_price = {
            let cap = accum.cap_raw.as_u128();
            if cap == 0 {
                Cents::ZERO
            } else {
                Cents::new((accum.investor_cap_raw / cap) as u64)
            }
        };
        self.investor
            .price
            .cents
            .height
            .push(investor_price);

        self.peak_regret
            .value
            .base
            .height
            .push(accum.peak_regret());
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.core
            .compute_rest_part1(starting_indexes, exit)?;

        self.peak_regret
            .value
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
            prices,
            starting_indexes,
            height_to_supply,
            exit,
        )?;

        // SOPR ratios from lazy rolling sums
        for ((sopr, vc), vd) in self
            .sopr
            .ratio_extended
            .as_mut_array()
            .into_iter()
            .zip(self.core.minimal.sopr.value_created.sum.as_array()[1..].iter())
            .zip(self.core.minimal.sopr.value_destroyed.sum.as_array()[1..].iter())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &vc.height,
                &vd.height,
                exit,
            )?;
        }

        // Profit/loss/net_pnl rel to realized cap
        self.profit
            .to_rcap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.core.minimal.profit.base.cents.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;
        self.loss
            .to_rcap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.core.minimal.loss.base.cents.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;
        self.net_pnl
            .to_rcap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.core.net_pnl.base.cents.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;

        // Profit/loss value created/destroyed cumulatives (rolling sums are lazy)
        self.profit
            .value_created
            .compute_rest(starting_indexes.height, exit)?;
        self.profit
            .value_destroyed
            .compute_rest(starting_indexes.height, exit)?;
        self.loss
            .value_created
            .compute_rest(starting_indexes.height, exit)?;
        self.loss
            .value_destroyed
            .compute_rest(starting_indexes.height, exit)?;

        // Gross PnL
        self.gross_pnl.base.cents.height.compute_add(
            starting_indexes.height,
            &self.core.minimal.profit.base.cents.height,
            &self.core.minimal.loss.base.cents.height,
            exit,
        )?;
        self.gross_pnl
            .compute_rest(starting_indexes.height, exit)?;

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

        // Peak regret rel to rcap
        self.peak_regret
            .to_rcap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.peak_regret.value.base.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;

        // Investor price ratio, percentiles and bands
        self.investor.price.compute_rest(
            prices,
            starting_indexes,
            exit,
        )?;

        self.investor
            .investor_lower_band
            .cents
            .height
            .compute_transform2(
                starting_indexes.height,
                &self.core.minimal.price.cents.height,
                &self.investor.price.cents.height,
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

        self.investor
            .investor_upper_band
            .cents
            .height
            .compute_transform2(
                starting_indexes.height,
                &self.investor.price.cents.height,
                &self.core.minimal.price.cents.height,
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
    profit_value_created: CentsSats,
    profit_value_destroyed: CentsSats,
    loss_value_created: CentsSats,
    loss_value_destroyed: CentsSats,
    pub(crate) cap_raw: CentsSats,
    pub(crate) investor_cap_raw: CentsSquaredSats,
    peak_regret: CentsSats,
}

impl RealizedFullAccum {
    pub(crate) fn add(&mut self, state: &RealizedState) {
        self.profit_value_created += CentsSats::new(state.profit_value_created_raw());
        self.profit_value_destroyed += CentsSats::new(state.profit_value_destroyed_raw());
        self.loss_value_created += CentsSats::new(state.loss_value_created_raw());
        self.loss_value_destroyed += CentsSats::new(state.loss_value_destroyed_raw());
        self.cap_raw += state.cap_raw();
        self.investor_cap_raw += state.investor_cap_raw();
        self.peak_regret += CentsSats::new(state.peak_regret_raw());
    }

    pub(crate) fn profit_value_created(&self) -> Cents {
        self.profit_value_created.to_cents()
    }

    pub(crate) fn profit_value_destroyed(&self) -> Cents {
        self.profit_value_destroyed.to_cents()
    }

    pub(crate) fn loss_value_created(&self) -> Cents {
        self.loss_value_created.to_cents()
    }

    pub(crate) fn loss_value_destroyed(&self) -> Cents {
        self.loss_value_destroyed.to_cents()
    }

    pub(crate) fn peak_regret(&self) -> Cents {
        self.peak_regret.to_cents()
    }
}
