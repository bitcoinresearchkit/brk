use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, BasisPointsSigned32, Bitcoin, Cents, CentsSats, CentsSigned, CentsSquaredSats,
    Dollars, Height, Indexes, Sats, StoredF64, Version,
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
        CentsUnsignedToDollars, ComputedPerBlock, ComputedPerBlockCumulative, FiatPerBlock,
        FiatRollingDelta1m, FiatRollingDeltaExcept1m, LazyPerBlock, PercentPerBlock,
        PercentRollingWindows, Price, RatioCents64, RatioCentsBp32, RatioCentsSignedCentsBps32,
        RatioCentsSignedDollarsBps32, RatioDollarsBp32, RatioPerBlock, RatioPerBlockPercentiles,
        RatioPerBlockStdDevBands, RollingWindows, RollingWindowsFrom1w,
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
    pub base: RealizedBase<M>,

    pub gross_pnl: FiatPerBlock<Cents, M>,

    pub profit_rel_to_rcap: PercentPerBlock<BasisPoints32, M>,
    pub loss_rel_to_rcap: PercentPerBlock<BasisPoints32, M>,
    pub net_pnl_rel_to_rcap: PercentPerBlock<BasisPointsSigned32, M>,

    pub profit_value_created: ComputedPerBlock<Cents, M>,
    pub profit_value_destroyed: ComputedPerBlock<Cents, M>,
    pub loss_value_created: ComputedPerBlock<Cents, M>,
    pub loss_value_destroyed: ComputedPerBlock<Cents, M>,

    pub profit_value_created_sum: RollingWindows<Cents, M>,
    pub profit_value_destroyed_sum: RollingWindows<Cents, M>,
    pub loss_value_created_sum: RollingWindows<Cents, M>,
    pub loss_value_destroyed_sum: RollingWindows<Cents, M>,

    pub capitulation_flow: LazyPerBlock<Dollars, Cents>,
    pub profit_flow: LazyPerBlock<Dollars, Cents>,

    pub gross_pnl_sum: RollingWindows<Cents, M>,

    pub net_pnl_cumulative: ComputedPerBlock<CentsSigned, M>,
    #[traversable(rename = "net_pnl_sum")]
    pub net_pnl_sum_extended: RollingWindowsFrom1w<CentsSigned, M>,

    pub net_pnl_delta: FiatRollingDelta1m<CentsSigned, CentsSigned, M>,
    #[traversable(rename = "net_pnl_delta")]
    pub net_pnl_delta_extended: FiatRollingDeltaExcept1m<CentsSigned, CentsSigned, M>,
    pub net_pnl_change_1m_rel_to_rcap: PercentPerBlock<BasisPointsSigned32, M>,
    pub net_pnl_change_1m_rel_to_mcap: PercentPerBlock<BasisPointsSigned32, M>,

    #[traversable(rename = "cap_delta")]
    pub cap_delta_extended: FiatRollingDeltaExcept1m<Cents, CentsSigned, M>,

    pub investor_price: Price<ComputedPerBlock<Cents, M>>,
    pub investor_price_ratio: RatioPerBlock<M>,

    pub lower_price_band: Price<ComputedPerBlock<Cents, M>>,
    pub upper_price_band: Price<ComputedPerBlock<Cents, M>>,

    pub cap_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub investor_cap_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    pub sell_side_risk_ratio: PercentRollingWindows<BasisPoints32, M>,

    pub peak_regret: ComputedPerBlockCumulative<Cents, M>,
    pub peak_regret_rel_to_realized_cap: PercentPerBlock<BasisPoints32, M>,

    pub cap_rel_to_own_mcap: PercentPerBlock<BasisPoints32, M>,

    #[traversable(rename = "profit_sum")]
    pub profit_sum_extended: RollingWindowsFrom1w<Cents, M>,
    #[traversable(rename = "loss_sum")]
    pub loss_sum_extended: RollingWindowsFrom1w<Cents, M>,
    pub profit_to_loss_ratio: RollingWindows<StoredF64, M>,

    #[traversable(rename = "value_created_sum")]
    pub value_created_sum_extended: RollingWindowsFrom1w<Cents, M>,
    #[traversable(rename = "value_destroyed_sum")]
    pub value_destroyed_sum_extended: RollingWindowsFrom1w<Cents, M>,
    #[traversable(rename = "sopr")]
    pub sopr_extended: RollingWindowsFrom1w<StoredF64, M>,

    #[traversable(rename = "sent_in_profit_sum")]
    pub sent_in_profit_sum_extended: RollingWindowsFrom1w<Sats, M>,
    #[traversable(rename = "sent_in_loss_sum")]
    pub sent_in_loss_sum_extended: RollingWindowsFrom1w<Sats, M>,

    pub price_ratio_percentiles: RatioPerBlockPercentiles<M>,
    pub price_ratio_std_dev: RatioPerBlockStdDevBands<M>,
    pub investor_price_ratio_percentiles: RatioPerBlockPercentiles<M>,
}

impl RealizedFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;

        let base = RealizedBase::forced_import(cfg)?;

        let gross_pnl = cfg.import("realized_gross_pnl", v0)?;

        let profit_value_created = cfg.import("profit_value_created", v0)?;
        let profit_value_destroyed: ComputedPerBlock<Cents> =
            cfg.import("profit_value_destroyed", v0)?;
        let loss_value_created = cfg.import("loss_value_created", v0)?;
        let loss_value_destroyed: ComputedPerBlock<Cents> =
            cfg.import("loss_value_destroyed", v0)?;

        let profit_value_created_sum = cfg.import("profit_value_created", v1)?;
        let profit_value_destroyed_sum = cfg.import("profit_value_destroyed", v1)?;
        let loss_value_created_sum = cfg.import("loss_value_created", v1)?;
        let loss_value_destroyed_sum = cfg.import("loss_value_destroyed", v1)?;

        let capitulation_flow = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("capitulation_flow"),
            cfg.version,
            loss_value_destroyed.height.read_only_boxed_clone(),
            &loss_value_destroyed,
        );
        let profit_flow = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
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

        let sell_side_risk_ratio = cfg.import("sell_side_risk_ratio", Version::new(2))?;

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

        let value_created_sum_extended = cfg.import("value_created", v1)?;
        let value_destroyed_sum_extended = cfg.import("value_destroyed", v1)?;
        let sopr_extended = cfg.import("sopr", v1)?;

        Ok(Self {
            base,
            gross_pnl,
            profit_rel_to_rcap: realized_profit_rel_to_realized_cap,
            loss_rel_to_rcap: realized_loss_rel_to_realized_cap,
            net_pnl_rel_to_rcap: net_realized_pnl_rel_to_realized_cap,
            profit_value_created,
            profit_value_destroyed,
            loss_value_created,
            loss_value_destroyed,
            profit_value_created_sum,
            profit_value_destroyed_sum,
            loss_value_created_sum,
            loss_value_destroyed_sum,
            capitulation_flow,
            profit_flow,
            gross_pnl_sum,
            net_pnl_cumulative: cfg.import("net_realized_pnl_cumulative", Version::ONE)?,
            net_pnl_sum_extended: cfg.import("net_realized_pnl", Version::ONE)?,
            net_pnl_delta: cfg.import("net_pnl_delta", Version::new(5))?,
            net_pnl_delta_extended: cfg.import("net_pnl_delta", Version::new(5))?,
            net_pnl_change_1m_rel_to_rcap: cfg
                .import("net_pnl_change_1m_rel_to_realized_cap", Version::new(4))?,
            net_pnl_change_1m_rel_to_mcap: cfg
                .import("net_pnl_change_1m_rel_to_market_cap", Version::new(4))?,
            cap_delta_extended: cfg.import("realized_cap_delta", Version::new(5))?,
            investor_price,
            investor_price_ratio,
            lower_price_band,
            upper_price_band,
            cap_raw,
            investor_cap_raw,
            sell_side_risk_ratio,
            peak_regret,
            peak_regret_rel_to_realized_cap,
            cap_rel_to_own_mcap: cfg.import("realized_cap_rel_to_own_market_cap", v1)?,
            profit_sum_extended: cfg.import("realized_profit", v1)?,
            loss_sum_extended: cfg.import("realized_loss", v1)?,
            profit_to_loss_ratio: cfg.import("realized_profit_to_loss_ratio", v1)?,
            value_created_sum_extended,
            value_destroyed_sum_extended,
            sopr_extended,
            sent_in_profit_sum_extended: cfg.import("sent_in_profit", v1)?,
            sent_in_loss_sum_extended: cfg.import("sent_in_loss", v1)?,
            price_ratio_percentiles: RatioPerBlockPercentiles::forced_import(
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
            investor_price_ratio_percentiles: RatioPerBlockPercentiles::forced_import(
                cfg.db,
                &investor_price_name,
                investor_price_version,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.base
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
        self.base.truncate_push(height, state)?;
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
        let mut vecs = self.base.collect_vecs_mut();
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

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&RealizedBase],
        exit: &Exit,
    ) -> Result<()> {
        self.base
            .compute_from_stateful(starting_indexes, others, exit)?;

        Ok(())
    }

    pub(crate) fn push_from_accum(
        &mut self,
        accum: &RealizedFullAccum,
        height: Height,
    ) -> Result<()> {
        self.profit_value_created
            .height
            .truncate_push(height, accum.profit_value_created)?;
        self.profit_value_destroyed
            .height
            .truncate_push(height, accum.profit_value_destroyed)?;
        self.loss_value_created
            .height
            .truncate_push(height, accum.loss_value_created)?;
        self.loss_value_destroyed
            .height
            .truncate_push(height, accum.loss_value_destroyed)?;
        self.cap_raw.truncate_push(height, accum.cap_raw)?;
        self.investor_cap_raw
            .truncate_push(height, accum.investor_cap_raw)?;

        let investor_price = {
            let cap = accum.cap_raw.as_u128();
            if cap == 0 {
                Cents::ZERO
            } else {
                Cents::new((accum.investor_cap_raw / cap) as u64)
            }
        };
        self.investor_price
            .cents
            .height
            .truncate_push(height, investor_price)?;

        self.peak_regret
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
        self.base
            .compute_rest_part1(blocks, starting_indexes, exit)?;

        self.net_pnl_cumulative.height.compute_cumulative(
            starting_indexes.height,
            &self.base.core.net_pnl.height,
            exit,
        )?;

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
        self.base.core.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_supply,
            exit,
        )?;

        let window_starts = blocks.lookback.window_starts();

        // Extended rolling sum (1w, 1m, 1y) for net_realized_pnl
        self.net_pnl_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.base.core.net_pnl.height,
            exit,
        )?;

        // Extended rolling windows (1w, 1m, 1y) for value_created/destroyed/sopr
        self.value_created_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.base.core.value_created.height,
            exit,
        )?;
        self.value_destroyed_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.base.core.value_destroyed.height,
            exit,
        )?;
        for ((sopr, vc), vd) in self
            .sopr_extended
            .as_mut_array()
            .into_iter()
            .zip(self.value_created_sum_extended.as_array())
            .zip(self.value_destroyed_sum_extended.as_array())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &vc.height,
                &vd.height,
                exit,
            )?;
        }

        // Realized P/L rel to realized cap
        self.profit_rel_to_rcap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.base.core.minimal.profit.height,
                &self.base.core.minimal.cap_cents.height,
                exit,
            )?;
        self.loss_rel_to_rcap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.base.core.minimal.loss.height,
                &self.base.core.minimal.cap_cents.height,
                exit,
            )?;
        self.net_pnl_rel_to_rcap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.base.core.net_pnl.height,
                &self.base.core.minimal.cap_cents.height,
                exit,
            )?;

        // Sent in profit/loss extended rolling sums (1w, 1m, 1y)
        self.sent_in_profit_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.base.sent_in_profit.height,
            exit,
        )?;
        self.sent_in_loss_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.base.sent_in_loss.height,
            exit,
        )?;

        // Profit/loss value created/destroyed rolling sums
        self.profit_value_created_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.profit_value_created.height,
            exit,
        )?;
        self.profit_value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.profit_value_destroyed.height,
            exit,
        )?;
        self.loss_value_created_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.loss_value_created.height,
            exit,
        )?;
        self.loss_value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.loss_value_destroyed.height,
            exit,
        )?;

        // Gross PnL
        self.gross_pnl.cents.height.compute_add(
            starting_indexes.height,
            &self.base.core.minimal.profit.height,
            &self.base.core.minimal.loss.height,
            exit,
        )?;

        self.gross_pnl_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.gross_pnl.cents.height,
            exit,
        )?;

        // Net PnL delta (1m base + 24h/1w/1y extended)
        self.net_pnl_delta.compute(
            starting_indexes.height,
            &blocks.lookback.height_1m_ago,
            &self.net_pnl_cumulative.height,
            exit,
        )?;
        self.net_pnl_delta_extended.compute(
            starting_indexes.height,
            &window_starts,
            &self.net_pnl_cumulative.height,
            exit,
        )?;
        self.net_pnl_change_1m_rel_to_rcap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.net_pnl_delta.change_1m.cents.height,
                &self.base.core.minimal.cap_cents.height,
                exit,
            )?;
        self.net_pnl_change_1m_rel_to_mcap
            .compute_binary::<CentsSigned, Dollars, RatioCentsSignedDollarsBps32>(
                starting_indexes.height,
                &self.net_pnl_delta.change_1m.cents.height,
                height_to_market_cap,
                exit,
            )?;

        // Realized cap delta extended (24h/1w/1y — 1m is in RealizedCore)
        self.cap_delta_extended.compute(
            starting_indexes.height,
            &window_starts,
            &self.base.core.minimal.cap_cents.height,
            exit,
        )?;

        // Peak regret
        self.peak_regret_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.peak_regret.height,
                &self.base.core.minimal.cap_cents.height,
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
            &self.base.core.minimal.price.cents.height,
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
            &self.base.core.minimal.price.cents.height,
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
                &self.base.core.minimal.cap_cents.height,
                exit,
            )?;
        }

        // Extended: realized profit/loss rolling sums (1w, 1m, 1y)
        self.profit_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.base.core.minimal.profit.height,
            exit,
        )?;
        self.loss_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.base.core.minimal.loss.height,
            exit,
        )?;

        // Realized cap relative to own market cap
        self.cap_rel_to_own_mcap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &self.base.core.minimal.cap.height,
                height_to_market_cap,
                exit,
            )?;

        // Realized profit to loss ratios
        self.profit_to_loss_ratio
            ._24h
            .compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &self.base.core.minimal.profit_sum._24h.height,
                &self.base.core.minimal.loss_sum._24h.height,
                exit,
            )?;
        for ((ratio, profit), loss) in self
            .profit_to_loss_ratio
            .as_mut_array_from_1w()
            .into_iter()
            .zip(self.profit_sum_extended.as_array())
            .zip(self.loss_sum_extended.as_array())
        {
            ratio.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &profit.height,
                &loss.height,
                exit,
            )?;
        }

        self.price_ratio_percentiles.compute(
            blocks,
            starting_indexes,
            exit,
            &self.base.core.minimal.price_ratio.ratio.height,
            &self.base.core.minimal.price.cents.height,
        )?;

        self.price_ratio_std_dev.compute(
            blocks,
            starting_indexes,
            exit,
            &self.base.core.minimal.price_ratio.ratio.height,
            &self.base.core.minimal.price.cents.height,
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
