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
        CentsUnsignedToDollars, ComputedPerBlock, ComputedPerBlockCumulative, FiatPerBlock,
        FiatRollingDelta1m, FiatRollingDeltaExcept1m, LazyPerBlock, PercentPerBlock,
        PercentRollingWindows, Price, PriceWithRatioExtendedPerBlock, RatioCents64, RatioCentsBp32,
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
    pub rel_to_rcap: PercentPerBlock<BasisPoints32, M>,
    pub value_created: ComputedPerBlock<Cents, M>,
    pub value_destroyed: ComputedPerBlock<Cents, M>,
    #[traversable(wrap = "value_created", rename = "sum")]
    pub value_created_sum: RollingWindows<Cents, M>,
    #[traversable(wrap = "value_destroyed", rename = "sum")]
    pub value_destroyed_sum: RollingWindows<Cents, M>,
    pub distribution_flow: LazyPerBlock<Dollars, Cents>,
    #[traversable(rename = "sum")]
    pub sum_extended: RollingWindowsFrom1w<Cents, M>,
}

#[derive(Traversable)]
pub struct RealizedLoss<M: StorageMode = Rw> {
    pub rel_to_rcap: PercentPerBlock<BasisPoints32, M>,
    pub value_created: ComputedPerBlock<Cents, M>,
    pub value_destroyed: ComputedPerBlock<Cents, M>,
    #[traversable(wrap = "value_created", rename = "sum")]
    pub value_created_sum: RollingWindows<Cents, M>,
    #[traversable(wrap = "value_destroyed", rename = "sum")]
    pub value_destroyed_sum: RollingWindows<Cents, M>,
    pub capitulation_flow: LazyPerBlock<Dollars, Cents>,
    #[traversable(rename = "sum")]
    pub sum_extended: RollingWindowsFrom1w<Cents, M>,
}

#[derive(Traversable)]
pub struct RealizedGrossPnl<M: StorageMode = Rw> {
    pub raw: FiatPerBlock<Cents, M>,
    pub sum: RollingWindows<Cents, M>,
    pub sell_side_risk_ratio: PercentRollingWindows<BasisPoints32, M>,
}

#[derive(Traversable)]
pub struct RealizedNetPnl<M: StorageMode = Rw> {
    pub rel_to_rcap: PercentPerBlock<BasisPointsSigned32, M>,
    pub cumulative: ComputedPerBlock<CentsSigned, M>,
    #[traversable(rename = "sum")]
    pub sum_extended: RollingWindowsFrom1w<CentsSigned, M>,
    pub delta: FiatRollingDelta1m<CentsSigned, CentsSigned, M>,
    #[traversable(rename = "delta")]
    pub delta_extended: FiatRollingDeltaExcept1m<CentsSigned, CentsSigned, M>,
    #[traversable(wrap = "change_1m", rename = "rel_to_rcap")]
    pub change_1m_rel_to_rcap: PercentPerBlock<BasisPointsSigned32, M>,
    #[traversable(wrap = "change_1m", rename = "rel_to_mcap")]
    pub change_1m_rel_to_mcap: PercentPerBlock<BasisPointsSigned32, M>,
}

#[derive(Traversable)]
pub struct RealizedSopr<M: StorageMode = Rw> {
    #[traversable(wrap = "value_created", rename = "sum")]
    pub value_created_sum_extended: RollingWindowsFrom1w<Cents, M>,
    #[traversable(wrap = "value_destroyed", rename = "sum")]
    pub value_destroyed_sum_extended: RollingWindowsFrom1w<Cents, M>,
    #[traversable(rename = "ratio")]
    pub ratio_extended: RollingWindowsFrom1w<StoredF64, M>,
}

#[derive(Traversable)]
pub struct RealizedPeakRegret<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub value: ComputedPerBlockCumulative<Cents, M>,
    pub rel_to_rcap: PercentPerBlock<BasisPoints32, M>,
}

#[derive(Traversable)]
pub struct RealizedInvestor<M: StorageMode = Rw> {
    pub price: PriceWithRatioExtendedPerBlock<M>,
    pub lower_price_band: Price<ComputedPerBlock<Cents, M>>,
    pub upper_price_band: Price<ComputedPerBlock<Cents, M>>,
    #[traversable(wrap = "cap", rename = "raw")]
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
    pub gross_pnl: RealizedGrossPnl<M>,
    pub net_pnl: RealizedNetPnl<M>,
    pub sopr: RealizedSopr<M>,
    pub peak_regret: RealizedPeakRegret<M>,
    pub investor: RealizedInvestor<M>,

    pub profit_to_loss_ratio: RollingWindows<StoredF64, M>,

    #[traversable(wrap = "cap", rename = "delta")]
    pub cap_delta_extended: FiatRollingDeltaExcept1m<Cents, CentsSigned, M>,

    #[traversable(wrap = "cap", rename = "raw")]
    pub cap_raw: M::Stored<BytesVec<Height, CentsSats>>,
    #[traversable(wrap = "cap", rename = "rel_to_own_mcap")]
    pub cap_rel_to_own_mcap: PercentPerBlock<BasisPoints32, M>,

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
        let profit_value_destroyed: ComputedPerBlock<Cents> =
            cfg.import("profit_value_destroyed", v0)?;
        let profit_flow = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("profit_flow"),
            cfg.version,
            profit_value_destroyed.height.read_only_boxed_clone(),
            &profit_value_destroyed,
        );
        let profit = RealizedProfit {
            rel_to_rcap: cfg.import("realized_profit_rel_to_realized_cap", Version::new(2))?,
            value_created: cfg.import("profit_value_created", v0)?,
            value_destroyed: profit_value_destroyed,
            value_created_sum: cfg.import("profit_value_created", v1)?,
            value_destroyed_sum: cfg.import("profit_value_destroyed", v1)?,
            distribution_flow: profit_flow,
            sum_extended: cfg.import("realized_profit", v1)?,
        };

        // Loss
        let loss_value_destroyed: ComputedPerBlock<Cents> =
            cfg.import("loss_value_destroyed", v0)?;
        let capitulation_flow = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("capitulation_flow"),
            cfg.version,
            loss_value_destroyed.height.read_only_boxed_clone(),
            &loss_value_destroyed,
        );
        let loss = RealizedLoss {
            rel_to_rcap: cfg.import("realized_loss_rel_to_realized_cap", Version::new(2))?,
            value_created: cfg.import("loss_value_created", v0)?,
            value_destroyed: loss_value_destroyed,
            value_created_sum: cfg.import("loss_value_created", v1)?,
            value_destroyed_sum: cfg.import("loss_value_destroyed", v1)?,
            capitulation_flow,
            sum_extended: cfg.import("realized_loss", v1)?,
        };

        // Gross PnL
        let gross_pnl = RealizedGrossPnl {
            raw: cfg.import("realized_gross_pnl", v0)?,
            sum: cfg.import("gross_pnl_sum", v1)?,
            sell_side_risk_ratio: cfg.import("sell_side_risk_ratio", Version::new(2))?,
        };

        // Net PnL
        let net_pnl = RealizedNetPnl {
            rel_to_rcap: cfg
                .import("net_realized_pnl_rel_to_realized_cap", Version::new(2))?,
            cumulative: cfg.import("net_realized_pnl_cumulative", v1)?,
            sum_extended: cfg.import("net_realized_pnl", v1)?,
            delta: cfg.import("net_pnl_delta", Version::new(5))?,
            delta_extended: cfg.import("net_pnl_delta", Version::new(5))?,
            change_1m_rel_to_rcap: cfg
                .import("net_pnl_change_1m_rel_to_realized_cap", Version::new(4))?,
            change_1m_rel_to_mcap: cfg
                .import("net_pnl_change_1m_rel_to_market_cap", Version::new(4))?,
        };

        // SOPR
        let sopr = RealizedSopr {
            value_created_sum_extended: cfg.import("value_created", v1)?,
            value_destroyed_sum_extended: cfg.import("value_destroyed", v1)?,
            ratio_extended: cfg.import("sopr", v1)?,
        };

        // Peak regret
        let peak_regret = RealizedPeakRegret {
            value: cfg.import("realized_peak_regret", Version::new(2))?,
            rel_to_rcap: cfg
                .import("realized_peak_regret_rel_to_realized_cap", Version::new(2))?,
        };

        // Investor
        let investor = RealizedInvestor {
            price: cfg.import("investor_price", v0)?,
            lower_price_band: cfg.import("lower_price_band", v0)?,
            upper_price_band: cfg.import("upper_price_band", v0)?,
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
            net_pnl,
            sopr,
            peak_regret,
            investor,
            profit_to_loss_ratio: cfg.import("realized_profit_to_loss_ratio", v1)?,
            cap_delta_extended: cfg.import("realized_cap_delta", Version::new(5))?,
            cap_raw: cfg.import("cap_raw", v0)?,
            cap_rel_to_own_mcap: cfg.import("realized_cap_rel_to_own_market_cap", v1)?,
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
            .height
            .len()
            .min(self.profit.value_destroyed.height.len())
            .min(self.loss.value_created.height.len())
            .min(self.loss.value_destroyed.height.len())
            .min(self.investor.price.cents.height.len())
            .min(self.cap_raw.len())
            .min(self.investor.cap_raw.len())
            .min(self.peak_regret.value.raw.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        state: &CohortState<RealizedState, CostBasisData<WithCapital>>,
    ) -> Result<()> {
        self.core.truncate_push(height, state)?;
        self.profit
            .value_created
            .height
            .truncate_push(height, state.realized.profit_value_created())?;
        self.profit
            .value_destroyed
            .height
            .truncate_push(height, state.realized.profit_value_destroyed())?;
        self.loss
            .value_created
            .height
            .truncate_push(height, state.realized.loss_value_created())?;
        self.loss
            .value_destroyed
            .height
            .truncate_push(height, state.realized.loss_value_destroyed())?;
        self.investor
            .price
            .cents
            .height
            .truncate_push(height, state.realized.investor_price())?;
        self.cap_raw
            .truncate_push(height, state.realized.cap_raw())?;
        self.investor
            .cap_raw
            .truncate_push(height, state.realized.investor_cap_raw())?;
        self.peak_regret
            .value
            .raw
            .height
            .truncate_push(height, state.realized.peak_regret())?;

        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(&mut self.profit.value_created.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.profit.value_destroyed.height);
        vecs.push(&mut self.loss.value_created.height);
        vecs.push(&mut self.loss.value_destroyed.height);
        vecs.push(&mut self.investor.price.cents.height);
        vecs.push(&mut self.cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor.cap_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.peak_regret.value.raw.height);
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

    pub(crate) fn push_from_accum(
        &mut self,
        accum: &RealizedFullAccum,
        height: Height,
    ) -> Result<()> {
        self.profit
            .value_created
            .height
            .truncate_push(height, accum.profit_value_created)?;
        self.profit
            .value_destroyed
            .height
            .truncate_push(height, accum.profit_value_destroyed)?;
        self.loss
            .value_created
            .height
            .truncate_push(height, accum.loss_value_created)?;
        self.loss
            .value_destroyed
            .height
            .truncate_push(height, accum.loss_value_destroyed)?;
        self.cap_raw
            .truncate_push(height, accum.cap_raw)?;
        self.investor
            .cap_raw
            .truncate_push(height, accum.investor_cap_raw)?;

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
            .truncate_push(height, investor_price)?;

        self.peak_regret
            .value
            .raw
            .height
            .truncate_push(height, accum.peak_regret)?;

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.core
            .compute_rest_part1(blocks, starting_indexes, exit)?;

        self.net_pnl.cumulative.height.compute_cumulative(
            starting_indexes.height,
            &self.core.net_pnl.raw.height,
            exit,
        )?;

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
            blocks,
            prices,
            starting_indexes,
            height_to_supply,
            exit,
        )?;

        let window_starts = blocks.lookback.window_starts();

        // Net PnL rolling sums (1w, 1m, 1y)
        self.net_pnl.sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.core.net_pnl.raw.height,
            exit,
        )?;

        // SOPR: value created/destroyed rolling sums and ratios
        self.sopr.value_created_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.core.minimal.sopr.value_created.raw.height,
            exit,
        )?;
        self.sopr
            .value_destroyed_sum_extended
            .compute_rolling_sum(
                starting_indexes.height,
                &window_starts,
                &self.core.minimal.sopr.value_destroyed.raw.height,
                exit,
            )?;
        for ((sopr, vc), vd) in self
            .sopr
            .ratio_extended
            .as_mut_array()
            .into_iter()
            .zip(self.sopr.value_created_sum_extended.as_array())
            .zip(self.sopr.value_destroyed_sum_extended.as_array())
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
            .rel_to_rcap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.core.minimal.profit.raw.cents.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;
        self.loss
            .rel_to_rcap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.core.minimal.loss.raw.cents.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;
        self.net_pnl
            .rel_to_rcap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.core.net_pnl.raw.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;

        // Profit/loss value created/destroyed rolling sums
        self.profit.value_created_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.profit.value_created.height,
            exit,
        )?;
        self.profit.value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.profit.value_destroyed.height,
            exit,
        )?;
        self.loss.value_created_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.loss.value_created.height,
            exit,
        )?;
        self.loss.value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.loss.value_destroyed.height,
            exit,
        )?;

        // Gross PnL
        self.gross_pnl.raw.cents.height.compute_add(
            starting_indexes.height,
            &self.core.minimal.profit.raw.cents.height,
            &self.core.minimal.loss.raw.cents.height,
            exit,
        )?;

        self.gross_pnl.sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.gross_pnl.raw.cents.height,
            exit,
        )?;

        // Net PnL delta (1m base + 24h/1w/1y extended)
        self.net_pnl.delta.compute(
            starting_indexes.height,
            &blocks.lookback._1m,
            &self.net_pnl.cumulative.height,
            exit,
        )?;
        self.net_pnl.delta_extended.compute(
            starting_indexes.height,
            &window_starts,
            &self.net_pnl.cumulative.height,
            exit,
        )?;
        self.net_pnl
            .change_1m_rel_to_rcap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.net_pnl.delta.change_1m.cents.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;
        self.net_pnl
            .change_1m_rel_to_mcap
            .compute_binary::<CentsSigned, Dollars, RatioCentsSignedDollarsBps32>(
                starting_indexes.height,
                &self.net_pnl.delta.change_1m.cents.height,
                height_to_market_cap,
                exit,
            )?;

        // Realized cap delta extended (24h/1w/1y — 1m is in RealizedCore)
        self.cap_delta_extended.compute(
            starting_indexes.height,
            &window_starts,
            &self.core.minimal.cap.cents.height,
            exit,
        )?;

        // Peak regret rel to rcap
        self.peak_regret
            .rel_to_rcap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.peak_regret.value.raw.height,
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
            .lower_price_band
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
            .upper_price_band
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
            .gross_pnl
            .sell_side_risk_ratio
            .as_mut_array()
            .into_iter()
            .zip(self.gross_pnl.sum.as_array())
        {
            ssrr.compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &rv.height,
                &self.core.minimal.cap.cents.height,
                exit,
            )?;
        }

        // Profit/loss sum extended (1w, 1m, 1y)
        self.profit.sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.core.minimal.profit.raw.cents.height,
            exit,
        )?;
        self.loss.sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.core.minimal.loss.raw.cents.height,
            exit,
        )?;

        // Realized cap relative to own market cap
        self.cap_rel_to_own_mcap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &self.core.minimal.cap.usd.height,
                height_to_market_cap,
                exit,
            )?;

        // Realized profit to loss ratios
        self.profit_to_loss_ratio
            ._24h
            .compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &self.core.minimal.profit.sum._24h.cents.height,
                &self.core.minimal.loss.sum._24h.cents.height,
                exit,
            )?;
        for ((ratio, profit), loss) in self
            .profit_to_loss_ratio
            .as_mut_array_from_1w()
            .into_iter()
            .zip(self.profit.sum_extended.as_array())
            .zip(self.loss.sum_extended.as_array())
        {
            ratio.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &profit.height,
                &loss.height,
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
    pub(crate) profit_value_created: Cents,
    pub(crate) profit_value_destroyed: Cents,
    pub(crate) loss_value_created: Cents,
    pub(crate) loss_value_destroyed: Cents,
    pub(crate) cap_raw: CentsSats,
    pub(crate) investor_cap_raw: CentsSquaredSats,
    pub(crate) peak_regret: Cents,
}

impl RealizedFullAccum {
    pub(crate) fn add(&mut self, state: &RealizedState) {
        self.profit_value_created += state.profit_value_created();
        self.profit_value_destroyed += state.profit_value_destroyed();
        self.loss_value_created += state.loss_value_created();
        self.loss_value_destroyed += state.loss_value_destroyed();
        self.cap_raw += state.cap_raw();
        self.investor_cap_raw += state.investor_cap_raw();
        self.peak_regret += state.peak_regret();
    }
}
