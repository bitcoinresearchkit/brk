use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Sats, StoredF32, StoredF64, Version};
use vecdb::IterableCloneableVec;

use crate::internal::{
    LazyBinaryFromHeightLast, LazyBinaryFromDateLast, NegPercentageDollarsF32, NegRatio32,
    PercentageDollarsF32, PercentageSatsF64, Ratio32,
};

use super::{ImportConfig, SupplyMetrics, UnrealizedMetrics};

/// Relative metrics comparing cohort values to global values.
/// All `rel_to_` vecs are lazy - computed on-demand from their sources.
#[derive(Clone, Traversable)]
pub struct RelativeMetrics {
    // === Supply Relative to Circulating Supply (lazy from global supply) ===
    pub supply_rel_to_circulating_supply: Option<LazyBinaryFromDateLast<StoredF64, Sats, Sats>>,

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
    pub unrealized_loss_rel_to_market_cap: Option<LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>>,
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
}

impl RelativeMetrics {
    /// Import relative metrics from database.
    ///
    /// All `rel_to_` metrics are lazy - computed on-demand from their sources.
    /// `all_supply` provides global sources for `*_rel_to_market_cap` and `*_rel_to_circulating_supply`.
    pub fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedMetrics,
        supply: &SupplyMetrics,
        all_supply: Option<&SupplyMetrics>,
    ) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);
        let extended = cfg.extended();
        let compute_rel_to_all = cfg.compute_rel_to_all();

        // Global sources from "all" cohort
        let global_supply_sats_height = all_supply.map(|s| &s.total.sats.height);
        let global_supply_sats_difficultyepoch = all_supply.map(|s| &s.total.sats.difficultyepoch);
        let global_supply_sats_dates = all_supply.map(|s| &s.total.sats.rest.dates);
        let global_supply_sats_dateindex = all_supply.map(|s| &s.total.sats.rest.dateindex);
        let global_market_cap = all_supply.and_then(|s| s.total.dollars.as_ref());

        // Own market cap source
        let own_market_cap = supply.total.dollars.as_ref();

        Ok(Self {
            // === Supply Relative to Circulating Supply (lazy from global supply) ===
            supply_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_sats_dates.is_some())
            .then(|| {
                LazyBinaryFromDateLast::from_both_derived_last::<PercentageSatsF64>(
                    &cfg.name("supply_rel_to_circulating_supply"),
                    cfg.version + v1,
                    supply.total.sats.rest.dateindex.boxed_clone(),
                    &supply.total.sats.rest.dates,
                    global_supply_sats_dateindex.unwrap().boxed_clone(),
                    global_supply_sats_dates.unwrap(),
                )
            }),

            // === Supply in Profit/Loss Relative to Own Supply (lazy) ===
            supply_in_profit_rel_to_own_supply:
                LazyBinaryFromHeightLast::from_height_difficultyepoch_dates::<PercentageSatsF64>(
                    &cfg.name("supply_in_profit_rel_to_own_supply"),
                    cfg.version + v1,
                    unrealized.supply_in_profit.height.boxed_clone(),
                    supply.total.sats.height.boxed_clone(),
                    unrealized.supply_in_profit.difficultyepoch.sats.boxed_clone(),
                    supply.total.sats.difficultyepoch.boxed_clone(),
                    unrealized
                        .supply_in_profit
                        .indexes
                        .sats_dateindex
                        .boxed_clone(),
                    &unrealized.supply_in_profit.indexes.sats,
                    supply.total.sats.rest.dateindex.boxed_clone(),
                    &supply.total.sats.rest.dates,
                ),
            supply_in_loss_rel_to_own_supply:
                LazyBinaryFromHeightLast::from_height_difficultyepoch_dates::<PercentageSatsF64>(
                    &cfg.name("supply_in_loss_rel_to_own_supply"),
                    cfg.version + v1,
                    unrealized.supply_in_loss.height.boxed_clone(),
                    supply.total.sats.height.boxed_clone(),
                    unrealized.supply_in_loss.difficultyepoch.sats.boxed_clone(),
                    supply.total.sats.difficultyepoch.boxed_clone(),
                    unrealized
                        .supply_in_loss
                        .indexes
                        .sats_dateindex
                        .boxed_clone(),
                    &unrealized.supply_in_loss.indexes.sats,
                    supply.total.sats.rest.dateindex.boxed_clone(),
                    &supply.total.sats.rest.dates,
                ),

            // === Supply in Profit/Loss Relative to Circulating Supply (lazy from global supply) ===
            supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_sats_height.is_some())
            .then(|| {
                LazyBinaryFromHeightLast::from_height_difficultyepoch_dates::<PercentageSatsF64>(
                    &cfg.name("supply_in_profit_rel_to_circulating_supply"),
                    cfg.version + v1,
                    unrealized.supply_in_profit.height.boxed_clone(),
                    global_supply_sats_height.unwrap().boxed_clone(),
                    unrealized.supply_in_profit.difficultyepoch.sats.boxed_clone(),
                    global_supply_sats_difficultyepoch.unwrap().boxed_clone(),
                    unrealized
                        .supply_in_profit
                        .indexes
                        .sats_dateindex
                        .boxed_clone(),
                    &unrealized.supply_in_profit.indexes.sats,
                    global_supply_sats_dateindex.unwrap().boxed_clone(),
                    global_supply_sats_dates.unwrap(),
                )
            }),
            supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_sats_height.is_some())
            .then(|| {
                LazyBinaryFromHeightLast::from_height_difficultyepoch_dates::<PercentageSatsF64>(
                    &cfg.name("supply_in_loss_rel_to_circulating_supply"),
                    cfg.version + v1,
                    unrealized.supply_in_loss.height.boxed_clone(),
                    global_supply_sats_height.unwrap().boxed_clone(),
                    unrealized.supply_in_loss.difficultyepoch.sats.boxed_clone(),
                    global_supply_sats_difficultyepoch.unwrap().boxed_clone(),
                    unrealized
                        .supply_in_loss
                        .indexes
                        .sats_dateindex
                        .boxed_clone(),
                    &unrealized.supply_in_loss.indexes.sats,
                    global_supply_sats_dateindex.unwrap().boxed_clone(),
                    global_supply_sats_dates.unwrap(),
                )
            }),

            // === Unrealized vs Market Cap (lazy from global market cap) ===
            unrealized_profit_rel_to_market_cap:
                global_market_cap.map(|mc| {
                    LazyBinaryFromHeightLast::from_computed_height_date_and_lazy_binary_block_last::<
                        PercentageDollarsF32,
                        _,
                        _,
                    >(
                        &cfg.name("unrealized_profit_rel_to_market_cap"),
                        cfg.version + v2,
                        &unrealized.unrealized_profit,
                        mc,
                    )
                }),
            unrealized_loss_rel_to_market_cap:
                global_market_cap.map(|mc| {
                    LazyBinaryFromHeightLast::from_computed_height_date_and_lazy_binary_block_last::<
                        PercentageDollarsF32,
                        _,
                        _,
                    >(
                        &cfg.name("unrealized_loss_rel_to_market_cap"),
                        cfg.version + v2,
                        &unrealized.unrealized_loss,
                        mc,
                    )
                }),
            neg_unrealized_loss_rel_to_market_cap: global_market_cap.map(|mc| {
                LazyBinaryFromHeightLast::from_computed_height_date_and_lazy_binary_block_last::<
                    NegPercentageDollarsF32,
                    _,
                    _,
                >(
                    &cfg.name("neg_unrealized_loss_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_loss,
                    mc,
                )
            }),
            net_unrealized_pnl_rel_to_market_cap: global_market_cap.map(|mc| {
                LazyBinaryFromHeightLast::from_binary_block_and_lazy_binary_block_last::<
                    PercentageDollarsF32,
                    _,
                    _,
                    _,
                    _,
                >(
                    &cfg.name("net_unrealized_pnl_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.net_unrealized_pnl,
                    mc,
                )
            }),

            // NUPL is a proxy for net_unrealized_pnl_rel_to_market_cap
            nupl: global_market_cap.map(|mc| {
                LazyBinaryFromHeightLast::from_binary_block_and_lazy_binary_block_last::<
                    PercentageDollarsF32,
                    _,
                    _,
                    _,
                    _,
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
                    own_market_cap.map(|mc| {
                        LazyBinaryFromHeightLast::from_computed_height_date_and_lazy_binary_block_last::<
                            PercentageDollarsF32,
                            _,
                            _,
                        >(
                            &cfg.name("unrealized_profit_rel_to_own_market_cap"),
                            cfg.version + v2,
                            &unrealized.unrealized_profit,
                            mc,
                        )
                    })
                })
                .flatten(),
            unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap.map(|mc| {
                        LazyBinaryFromHeightLast::from_computed_height_date_and_lazy_binary_block_last::<
                            PercentageDollarsF32,
                            _,
                            _,
                        >(
                            &cfg.name("unrealized_loss_rel_to_own_market_cap"),
                            cfg.version + v2,
                            &unrealized.unrealized_loss,
                            mc,
                        )
                    })
                })
                .flatten(),
            neg_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap.map(|mc| {
                        LazyBinaryFromHeightLast::from_computed_height_date_and_lazy_binary_block_last::<
                            NegPercentageDollarsF32,
                            _,
                            _,
                        >(
                            &cfg.name("neg_unrealized_loss_rel_to_own_market_cap"),
                            cfg.version + v2,
                            &unrealized.unrealized_loss,
                            mc,
                        )
                    })
                })
                .flatten(),
            net_unrealized_pnl_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap.map(|mc| {
                        LazyBinaryFromHeightLast::from_binary_block_and_lazy_binary_block_last::<
                            PercentageDollarsF32,
                            _,
                            _,
                            _,
                            _,
                        >(
                            &cfg.name("net_unrealized_pnl_rel_to_own_market_cap"),
                            cfg.version + v2,
                            &unrealized.net_unrealized_pnl,
                            mc,
                        )
                    })
                })
                .flatten(),

            // === Unrealized vs Own Total Unrealized PnL (lazy, optional) ===
            unrealized_profit_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyBinaryFromHeightLast::from_computed_height_date_and_binary_block::<Ratio32, _, _>(
                    &cfg.name("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                    cfg.version,
                    &unrealized.unrealized_profit,
                    &unrealized.total_unrealized_pnl,
                )
            }),
            unrealized_loss_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyBinaryFromHeightLast::from_computed_height_date_and_binary_block::<Ratio32, _, _>(
                    &cfg.name("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version,
                    &unrealized.unrealized_loss,
                    &unrealized.total_unrealized_pnl,
                )
            }),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyBinaryFromHeightLast::from_computed_height_date_and_binary_block::<NegRatio32, _, _>(
                    &cfg.name("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version,
                    &unrealized.unrealized_loss,
                    &unrealized.total_unrealized_pnl,
                )
            }),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyBinaryFromHeightLast::from_both_binary_block::<Ratio32, _, _, _, _>(
                    &cfg.name("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.net_unrealized_pnl,
                    &unrealized.total_unrealized_pnl,
                )
            }),
        })
    }

    /// Add relative-to-circulating-supply metrics in a second step.
    /// Called after the "all" cohort is available to provide global supply sources.
    pub fn add_rel_to_circulating(
        &mut self,
        filter: &Filter,
        full_name: &str,
        version: Version,
        unrealized: &UnrealizedMetrics,
        supply: &SupplyMetrics,
        all_supply: &SupplyMetrics,
    ) {
        let v1 = Version::ONE;

        // Skip if this is the "all" cohort itself
        if !filter.compute_rel_to_all() {
            return;
        }

        let name = |suffix: &str| {
            if full_name.is_empty() {
                suffix.to_string()
            } else {
                format!("{full_name}_{suffix}")
            }
        };

        // Global sources from "all" cohort
        let global_supply_sats_height = &all_supply.total.sats.height;
        let global_supply_sats_difficultyepoch = &all_supply.total.sats.difficultyepoch;
        let global_supply_sats_dates = &all_supply.total.sats.rest.dates;
        let global_supply_sats_dateindex = &all_supply.total.sats.rest.dateindex;

        // Fill in supply_rel_to_circulating_supply if not already set
        if self.supply_rel_to_circulating_supply.is_none() {
            self.supply_rel_to_circulating_supply = Some(
                LazyBinaryFromDateLast::from_both_derived_last::<PercentageSatsF64>(
                    &name("supply_rel_to_circulating_supply"),
                    version + v1,
                    supply.total.sats.rest.dateindex.boxed_clone(),
                    &supply.total.sats.rest.dates,
                    global_supply_sats_dateindex.boxed_clone(),
                    global_supply_sats_dates,
                ),
            );
        }

        // Fill in supply_in_profit_rel_to_circulating_supply if not already set
        if self.supply_in_profit_rel_to_circulating_supply.is_none() {
            self.supply_in_profit_rel_to_circulating_supply = Some(
                LazyBinaryFromHeightLast::from_height_difficultyepoch_dates::<PercentageSatsF64>(
                    &name("supply_in_profit_rel_to_circulating_supply"),
                    version + v1,
                    unrealized.supply_in_profit.height.boxed_clone(),
                    global_supply_sats_height.boxed_clone(),
                    unrealized.supply_in_profit.difficultyepoch.sats.boxed_clone(),
                    global_supply_sats_difficultyepoch.boxed_clone(),
                    unrealized
                        .supply_in_profit
                        .indexes
                        .sats_dateindex
                        .boxed_clone(),
                    &unrealized.supply_in_profit.indexes.sats,
                    global_supply_sats_dateindex.boxed_clone(),
                    global_supply_sats_dates,
                ),
            );
        }

        // Fill in supply_in_loss_rel_to_circulating_supply if not already set
        if self.supply_in_loss_rel_to_circulating_supply.is_none() {
            self.supply_in_loss_rel_to_circulating_supply = Some(
                LazyBinaryFromHeightLast::from_height_difficultyepoch_dates::<PercentageSatsF64>(
                    &name("supply_in_loss_rel_to_circulating_supply"),
                    version + v1,
                    unrealized.supply_in_loss.height.boxed_clone(),
                    global_supply_sats_height.boxed_clone(),
                    unrealized.supply_in_loss.difficultyepoch.sats.boxed_clone(),
                    global_supply_sats_difficultyepoch.boxed_clone(),
                    unrealized
                        .supply_in_loss
                        .indexes
                        .sats_dateindex
                        .boxed_clone(),
                    &unrealized.supply_in_loss.indexes.sats,
                    global_supply_sats_dateindex.boxed_clone(),
                    global_supply_sats_dates,
                ),
            );
        }
    }
}
