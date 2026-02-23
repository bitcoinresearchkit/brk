use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Sats, StoredF32, StoredF64, Version};

use crate::internal::{
    LazyBinaryFromHeightLast, NegPercentageDollarsF32, PercentageDollarsF32, PercentageSatsF64,
};

use super::{ImportConfig, RealizedMetrics, SupplyMetrics, UnrealizedMetrics};

/// Relative metrics comparing cohort values to global values.
/// All `rel_to_` vecs are lazy - computed on-demand from their sources.
#[derive(Clone, Traversable)]
pub struct RelativeMetrics {
    // === Supply Relative to Circulating Supply (lazy from global supply) ===
    pub supply_rel_to_circulating_supply:
        Option<LazyBinaryFromHeightLast<StoredF64, Sats, Sats>>,

    // === Supply in Profit/Loss Relative to Own Supply (lazy) ===
    pub supply_in_profit_rel_to_own_supply: LazyBinaryFromHeightLast<StoredF64, Sats, Sats>,
    pub supply_in_loss_rel_to_own_supply: LazyBinaryFromHeightLast<StoredF64, Sats, Sats>,

    // === Supply in Profit/Loss Relative to Circulating Supply (lazy from global supply) ===
    pub supply_in_profit_rel_to_circulating_supply:
        Option<LazyBinaryFromHeightLast<StoredF64, Sats, Sats>>,
    pub supply_in_loss_rel_to_circulating_supply:
        Option<LazyBinaryFromHeightLast<StoredF64, Sats, Sats>>,

    // === Unrealized vs Market Cap (lazy from global market cap) ===
    pub unrealized_profit_rel_to_market_cap:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub unrealized_loss_rel_to_market_cap:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub neg_unrealized_loss_rel_to_market_cap:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub net_unrealized_pnl_rel_to_market_cap:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,

    // === NUPL (Net Unrealized Profit/Loss) ===
    pub nupl: Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,

    // === Unrealized vs Own Market Cap (lazy) ===
    pub unrealized_profit_rel_to_own_market_cap:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub unrealized_loss_rel_to_own_market_cap:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub neg_unrealized_loss_rel_to_own_market_cap:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub net_unrealized_pnl_rel_to_own_market_cap:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,

    // === Unrealized vs Own Total Unrealized PnL (lazy) ===
    pub unrealized_profit_rel_to_own_total_unrealized_pnl:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,

    // === Invested Capital in Profit/Loss as % of Realized Cap ===
    pub invested_capital_in_profit_pct:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
    pub invested_capital_in_loss_pct:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,

    // === Unrealized Peak Regret Relative to Market Cap (lazy) ===
    pub unrealized_peak_regret_rel_to_market_cap:
        Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
}

impl RelativeMetrics {
    /// Import relative metrics from database.
    ///
    /// All `rel_to_` metrics are lazy - computed on-demand from their sources.
    /// `all_supply` provides global sources for `*_rel_to_market_cap` and `*_rel_to_circulating_supply`.
    /// `realized` provides realized_cap for invested capital percentage metrics.
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedMetrics,
        supply: &SupplyMetrics,
        all_supply: Option<&SupplyMetrics>,
        realized: Option<&RealizedMetrics>,
    ) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);
        let extended = cfg.extended();
        let compute_rel_to_all = cfg.compute_rel_to_all();

        // Global sources from "all" cohort
        let global_supply_sats = all_supply.map(|s| &s.total.sats);
        let global_market_cap = all_supply.map(|s| &s.total.usd);

        // Own market cap source
        let own_market_cap = &supply.total.usd;

        // For "all" cohort, own_market_cap IS the global market cap
        let market_cap = global_market_cap.or_else(|| {
            matches!(cfg.filter, Filter::All).then_some(own_market_cap)
        });

        Ok(Self {
            // === Supply Relative to Circulating Supply ===
            supply_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_sats.is_some())
            .then(|| {
                LazyBinaryFromHeightLast::from_computed_last::<PercentageSatsF64>(
                    &cfg.name("supply_rel_to_circulating_supply"),
                    cfg.version + v1,
                    &supply.total.sats,
                    global_supply_sats.unwrap(),
                )
            }),

            // === Supply in Profit/Loss Relative to Own Supply ===
            supply_in_profit_rel_to_own_supply:
                LazyBinaryFromHeightLast::from_computed_last::<PercentageSatsF64>(
                    &cfg.name("supply_in_profit_rel_to_own_supply"),
                    cfg.version + v1,
                    &unrealized.supply_in_profit.sats,
                    &supply.total.sats,
                ),
            supply_in_loss_rel_to_own_supply:
                LazyBinaryFromHeightLast::from_computed_last::<PercentageSatsF64>(
                    &cfg.name("supply_in_loss_rel_to_own_supply"),
                    cfg.version + v1,
                    &unrealized.supply_in_loss.sats,
                    &supply.total.sats,
                ),

            // === Supply in Profit/Loss Relative to Circulating Supply ===
            supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_sats.is_some())
            .then(|| {
                LazyBinaryFromHeightLast::from_computed_last::<PercentageSatsF64>(
                    &cfg.name("supply_in_profit_rel_to_circulating_supply"),
                    cfg.version + v1,
                    &unrealized.supply_in_profit.sats,
                    global_supply_sats.unwrap(),
                )
            }),
            supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_sats.is_some())
            .then(|| {
                LazyBinaryFromHeightLast::from_computed_last::<PercentageSatsF64>(
                    &cfg.name("supply_in_loss_rel_to_circulating_supply"),
                    cfg.version + v1,
                    &unrealized.supply_in_loss.sats,
                    global_supply_sats.unwrap(),
                )
            }),

            // === Unrealized vs Market Cap ===
            unrealized_profit_rel_to_market_cap: market_cap.map(|mc| {
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    PercentageDollarsF32, _, _,
                >(
                    &cfg.name("unrealized_profit_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_profit,
                    mc,
                )
            }),
            unrealized_loss_rel_to_market_cap: market_cap.map(|mc| {
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    PercentageDollarsF32, _, _,
                >(
                    &cfg.name("unrealized_loss_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_loss,
                    mc,
                )
            }),
            neg_unrealized_loss_rel_to_market_cap: market_cap.map(|mc| {
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    NegPercentageDollarsF32, _, _,
                >(
                    &cfg.name("neg_unrealized_loss_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_loss,
                    mc,
                )
            }),
            net_unrealized_pnl_rel_to_market_cap: market_cap.map(|mc| {
                LazyBinaryFromHeightLast::from_binary_block_and_lazy_binary_block_last::<
                    PercentageDollarsF32, _, _, _, _,
                >(
                    &cfg.name("net_unrealized_pnl_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.net_unrealized_pnl,
                    mc,
                )
            }),

            // NUPL is a proxy for net_unrealized_pnl_rel_to_market_cap
            nupl: market_cap.map(|mc| {
                LazyBinaryFromHeightLast::from_binary_block_and_lazy_binary_block_last::<
                    PercentageDollarsF32, _, _, _, _,
                >(
                    &cfg.name("nupl"),
                    cfg.version + v2,
                    &unrealized.net_unrealized_pnl,
                    mc,
                )
            }),

            // === Unrealized vs Own Market Cap (lazy, optional) ===
            unrealized_profit_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                        PercentageDollarsF32, _, _,
                    >(
                        &cfg.name("unrealized_profit_rel_to_own_market_cap"),
                        cfg.version + v2,
                        &unrealized.unrealized_profit,
                        own_market_cap,
                    )
                }),
            unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                        PercentageDollarsF32, _, _,
                    >(
                        &cfg.name("unrealized_loss_rel_to_own_market_cap"),
                        cfg.version + v2,
                        &unrealized.unrealized_loss,
                        own_market_cap,
                    )
                }),
            neg_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                        NegPercentageDollarsF32, _, _,
                    >(
                        &cfg.name("neg_unrealized_loss_rel_to_own_market_cap"),
                        cfg.version + v2,
                        &unrealized.unrealized_loss,
                        own_market_cap,
                    )
                }),
            net_unrealized_pnl_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    LazyBinaryFromHeightLast::from_binary_block_and_lazy_binary_block_last::<
                        PercentageDollarsF32, _, _, _, _,
                    >(
                        &cfg.name("net_unrealized_pnl_rel_to_own_market_cap"),
                        cfg.version + v2,
                        &unrealized.net_unrealized_pnl,
                        own_market_cap,
                    )
                }),

            // === Unrealized vs Own Total Unrealized PnL (lazy, optional) ===
            unrealized_profit_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyBinaryFromHeightLast::from_block_last_and_binary_block::<PercentageDollarsF32, _, _>(
                    &cfg.name("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.unrealized_profit,
                    &unrealized.total_unrealized_pnl,
                )
            }),
            unrealized_loss_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyBinaryFromHeightLast::from_block_last_and_binary_block::<PercentageDollarsF32, _, _>(
                    &cfg.name("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.unrealized_loss,
                    &unrealized.total_unrealized_pnl,
                )
            }),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyBinaryFromHeightLast::from_block_last_and_binary_block::<NegPercentageDollarsF32, _, _>(
                    &cfg.name("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.unrealized_loss,
                    &unrealized.total_unrealized_pnl,
                )
            }),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyBinaryFromHeightLast::from_both_binary_block::<PercentageDollarsF32, _, _, _, _>(
                    &cfg.name("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v2,
                    &unrealized.net_unrealized_pnl,
                    &unrealized.total_unrealized_pnl,
                )
            }),

            // === Invested Capital in Profit/Loss as % of Realized Cap ===
            invested_capital_in_profit_pct: realized.map(|r| {
                LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<
                    PercentageDollarsF32, _,
                >(
                    &cfg.name("invested_capital_in_profit_pct"),
                    cfg.version,
                    &unrealized.invested_capital_in_profit,
                    &r.realized_cap,
                )
            }),
            invested_capital_in_loss_pct: realized.map(|r| {
                LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<
                    PercentageDollarsF32, _,
                >(
                    &cfg.name("invested_capital_in_loss_pct"),
                    cfg.version,
                    &unrealized.invested_capital_in_loss,
                    &r.realized_cap,
                )
            }),

            // === Peak Regret Relative to Market Cap ===
            unrealized_peak_regret_rel_to_market_cap: unrealized
                .peak_regret
                .as_ref()
                .zip(market_cap)
                .map(|(pr, mc)| {
                    LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                        PercentageDollarsF32, _, _,
                    >(
                        &cfg.name("unrealized_peak_regret_rel_to_market_cap"),
                        cfg.version,
                        pr,
                        mc,
                    )
                }),
        })
    }
}
