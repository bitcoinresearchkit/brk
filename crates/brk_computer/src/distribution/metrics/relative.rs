use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, StoredF32, StoredF64, Version};
use vecdb::{IterableCloneableVec, LazyVecFrom2};

use crate::internal::{
    LazyVecsFrom2FromDateIndex, NegPercentageDollarsF32, NegRatio32, PercentageBtcF64,
    PercentageDollarsF32, PercentageSatsF64, Ratio32,
};

use super::{ImportConfig, SupplyMetrics, UnrealizedMetrics};

/// Relative metrics comparing cohort values to global values.
/// All `rel_to_` vecs are lazy - computed on-demand from their sources.
#[derive(Clone, Traversable)]
pub struct RelativeMetrics {
    // === Supply Relative to Circulating Supply (lazy from global supply) ===
    pub indexes_to_supply_rel_to_circulating_supply:
        Option<LazyVecsFrom2FromDateIndex<StoredF64, Sats, Sats>>,

    // === Supply in Profit/Loss Relative to Own Supply (lazy) ===
    pub height_to_supply_in_profit_rel_to_own_supply:
        LazyVecFrom2<Height, StoredF64, Height, Bitcoin, Height, Bitcoin>,
    pub height_to_supply_in_loss_rel_to_own_supply:
        LazyVecFrom2<Height, StoredF64, Height, Bitcoin, Height, Bitcoin>,
    pub indexes_to_supply_in_profit_rel_to_own_supply:
        LazyVecsFrom2FromDateIndex<StoredF64, Sats, Sats>,
    pub indexes_to_supply_in_loss_rel_to_own_supply:
        LazyVecsFrom2FromDateIndex<StoredF64, Sats, Sats>,

    // === Supply in Profit/Loss Relative to Circulating Supply (lazy from global supply) ===
    pub height_to_supply_in_profit_rel_to_circulating_supply:
        Option<LazyVecFrom2<Height, StoredF64, Height, Bitcoin, Height, Bitcoin>>,
    pub height_to_supply_in_loss_rel_to_circulating_supply:
        Option<LazyVecFrom2<Height, StoredF64, Height, Bitcoin, Height, Bitcoin>>,
    pub indexes_to_supply_in_profit_rel_to_circulating_supply:
        Option<LazyVecsFrom2FromDateIndex<StoredF64, Sats, Sats>>,
    pub indexes_to_supply_in_loss_rel_to_circulating_supply:
        Option<LazyVecsFrom2FromDateIndex<StoredF64, Sats, Sats>>,

    // === Unrealized vs Market Cap (lazy from global market cap) ===
    pub height_to_unrealized_profit_rel_to_market_cap:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub height_to_unrealized_loss_rel_to_market_cap:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub height_to_neg_unrealized_loss_rel_to_market_cap:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub height_to_net_unrealized_pnl_rel_to_market_cap:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub indexes_to_unrealized_profit_rel_to_market_cap:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
    pub indexes_to_unrealized_loss_rel_to_market_cap:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
    pub indexes_to_neg_unrealized_loss_rel_to_market_cap:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
    pub indexes_to_net_unrealized_pnl_rel_to_market_cap:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,

    // === NUPL (Net Unrealized Profit/Loss) ===
    // Proxy for indexes_to_net_unrealized_pnl_rel_to_market_cap
    pub indexes_to_nupl: Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,

    // === Unrealized vs Own Market Cap (lazy) ===
    pub height_to_unrealized_profit_rel_to_own_market_cap:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub height_to_unrealized_loss_rel_to_own_market_cap:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub height_to_neg_unrealized_loss_rel_to_own_market_cap:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub height_to_net_unrealized_pnl_rel_to_own_market_cap:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub indexes_to_unrealized_profit_rel_to_own_market_cap:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
    pub indexes_to_unrealized_loss_rel_to_own_market_cap:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
    pub indexes_to_neg_unrealized_loss_rel_to_own_market_cap:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
    pub indexes_to_net_unrealized_pnl_rel_to_own_market_cap:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,

    // === Unrealized vs Own Total Unrealized PnL (lazy) ===
    pub height_to_unrealized_profit_rel_to_own_total_unrealized_pnl:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub height_to_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
        Option<LazyVecFrom2<Height, StoredF32, Height, Dollars, Height, Dollars>>,
    pub indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
    pub indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
    pub indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
    pub indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
        Option<LazyVecsFrom2FromDateIndex<StoredF32, Dollars, Dollars>>,
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
        let global_supply_sats = all_supply.map(|s| &s.indexes_to_supply.sats);
        let global_supply_btc = all_supply.map(|s| &s.height_to_supply_value.bitcoin);
        let global_market_cap = all_supply.and_then(|s| s.indexes_to_supply.dollars.as_ref());
        let global_market_cap_height =
            all_supply.and_then(|s| s.height_to_supply_value.dollars.as_ref());

        // Own market cap source
        let own_market_cap = supply.indexes_to_supply.dollars.as_ref();
        let own_market_cap_height = supply.height_to_supply_value.dollars.as_ref();

        Ok(Self {
            // === Supply Relative to Circulating Supply (lazy from global supply) ===
            indexes_to_supply_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_sats.is_some())
            .then(|| {
                LazyVecsFrom2FromDateIndex::from_computed::<PercentageSatsF64>(
                    &cfg.name("supply_rel_to_circulating_supply"),
                    cfg.version + v1,
                    &supply.indexes_to_supply.sats,
                    global_supply_sats.unwrap(),
                )
            }),

            // === Supply in Profit/Loss Relative to Own Supply (lazy) ===
            height_to_supply_in_profit_rel_to_own_supply: LazyVecFrom2::transformed::<
                PercentageBtcF64,
            >(
                &cfg.name("supply_in_profit_rel_to_own_supply"),
                cfg.version + v1,
                unrealized
                    .height_to_supply_in_profit_value
                    .bitcoin
                    .boxed_clone(),
                supply.height_to_supply_value.bitcoin.boxed_clone(),
            ),
            height_to_supply_in_loss_rel_to_own_supply: LazyVecFrom2::transformed::<PercentageBtcF64>(
                &cfg.name("supply_in_loss_rel_to_own_supply"),
                cfg.version + v1,
                unrealized
                    .height_to_supply_in_loss_value
                    .bitcoin
                    .boxed_clone(),
                supply.height_to_supply_value.bitcoin.boxed_clone(),
            ),
            indexes_to_supply_in_profit_rel_to_own_supply:
                LazyVecsFrom2FromDateIndex::from_computed::<PercentageSatsF64>(
                    &cfg.name("supply_in_profit_rel_to_own_supply"),
                    cfg.version + v1,
                    &unrealized.indexes_to_supply_in_profit.sats,
                    &supply.indexes_to_supply.sats,
                ),
            indexes_to_supply_in_loss_rel_to_own_supply: LazyVecsFrom2FromDateIndex::from_computed::<
                PercentageSatsF64,
            >(
                &cfg.name("supply_in_loss_rel_to_own_supply"),
                cfg.version + v1,
                &unrealized.indexes_to_supply_in_loss.sats,
                &supply.indexes_to_supply.sats,
            ),

            // === Supply in Profit/Loss Relative to Circulating Supply (lazy from global supply) ===
            height_to_supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_btc.is_some())
            .then(|| {
                LazyVecFrom2::transformed::<PercentageBtcF64>(
                    &cfg.name("supply_in_profit_rel_to_circulating_supply"),
                    cfg.version + v1,
                    unrealized
                        .height_to_supply_in_profit_value
                        .bitcoin
                        .boxed_clone(),
                    global_supply_btc.unwrap().boxed_clone(),
                )
            }),
            height_to_supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_btc.is_some())
            .then(|| {
                LazyVecFrom2::transformed::<PercentageBtcF64>(
                    &cfg.name("supply_in_loss_rel_to_circulating_supply"),
                    cfg.version + v1,
                    unrealized
                        .height_to_supply_in_loss_value
                        .bitcoin
                        .boxed_clone(),
                    global_supply_btc.unwrap().boxed_clone(),
                )
            }),
            indexes_to_supply_in_profit_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_sats.is_some())
            .then(|| {
                LazyVecsFrom2FromDateIndex::from_computed::<PercentageSatsF64>(
                    &cfg.name("supply_in_profit_rel_to_circulating_supply"),
                    cfg.version + v1,
                    &unrealized.indexes_to_supply_in_profit.sats,
                    global_supply_sats.unwrap(),
                )
            }),
            indexes_to_supply_in_loss_rel_to_circulating_supply: (compute_rel_to_all
                && global_supply_sats.is_some())
            .then(|| {
                LazyVecsFrom2FromDateIndex::from_computed::<PercentageSatsF64>(
                    &cfg.name("supply_in_loss_rel_to_circulating_supply"),
                    cfg.version + v1,
                    &unrealized.indexes_to_supply_in_loss.sats,
                    global_supply_sats.unwrap(),
                )
            }),

            // === Unrealized vs Market Cap (lazy from global market cap) ===
            height_to_unrealized_profit_rel_to_market_cap: global_market_cap_height.map(|mc| {
                LazyVecFrom2::transformed::<PercentageDollarsF32>(
                    &cfg.name("unrealized_profit_rel_to_market_cap"),
                    cfg.version,
                    unrealized.height_to_unrealized_profit.boxed_clone(),
                    mc.boxed_clone(),
                )
            }),
            height_to_unrealized_loss_rel_to_market_cap: global_market_cap_height.map(|mc| {
                LazyVecFrom2::transformed::<PercentageDollarsF32>(
                    &cfg.name("unrealized_loss_rel_to_market_cap"),
                    cfg.version,
                    unrealized.height_to_unrealized_loss.boxed_clone(),
                    mc.boxed_clone(),
                )
            }),
            height_to_neg_unrealized_loss_rel_to_market_cap: global_market_cap_height.map(|mc| {
                LazyVecFrom2::transformed::<NegPercentageDollarsF32>(
                    &cfg.name("neg_unrealized_loss_rel_to_market_cap"),
                    cfg.version,
                    unrealized.height_to_unrealized_loss.boxed_clone(),
                    mc.boxed_clone(),
                )
            }),
            height_to_net_unrealized_pnl_rel_to_market_cap: global_market_cap_height.map(|mc| {
                LazyVecFrom2::transformed::<PercentageDollarsF32>(
                    &cfg.name("net_unrealized_pnl_rel_to_market_cap"),
                    cfg.version + v1,
                    unrealized.height_to_net_unrealized_pnl.boxed_clone(),
                    mc.boxed_clone(),
                )
            }),
            indexes_to_unrealized_profit_rel_to_market_cap: global_market_cap.map(|mc| {
                LazyVecsFrom2FromDateIndex::from_computed::<PercentageDollarsF32>(
                    &cfg.name("unrealized_profit_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.indexes_to_unrealized_profit,
                    mc,
                )
            }),
            indexes_to_unrealized_loss_rel_to_market_cap: global_market_cap.map(|mc| {
                LazyVecsFrom2FromDateIndex::from_computed::<PercentageDollarsF32>(
                    &cfg.name("unrealized_loss_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.indexes_to_unrealized_loss,
                    mc,
                )
            }),
            indexes_to_neg_unrealized_loss_rel_to_market_cap: global_market_cap.map(|mc| {
                LazyVecsFrom2FromDateIndex::from_computed::<NegPercentageDollarsF32>(
                    &cfg.name("neg_unrealized_loss_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.indexes_to_unrealized_loss,
                    mc,
                )
            }),
            indexes_to_net_unrealized_pnl_rel_to_market_cap: global_market_cap.map(|mc| {
                LazyVecsFrom2FromDateIndex::from_computed::<PercentageDollarsF32>(
                    &cfg.name("net_unrealized_pnl_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.indexes_to_net_unrealized_pnl,
                    mc,
                )
            }),

            // NUPL is a proxy for net_unrealized_pnl_rel_to_market_cap
            indexes_to_nupl: global_market_cap.map(|mc| {
                LazyVecsFrom2FromDateIndex::from_computed::<PercentageDollarsF32>(
                    &cfg.name("nupl"),
                    cfg.version + v2,
                    &unrealized.indexes_to_net_unrealized_pnl,
                    mc,
                )
            }),

            // === Unrealized vs Own Market Cap (lazy, optional) ===
            height_to_unrealized_profit_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap_height.map(|mc| {
                        LazyVecFrom2::transformed::<PercentageDollarsF32>(
                            &cfg.name("unrealized_profit_rel_to_own_market_cap"),
                            cfg.version + v1,
                            unrealized.height_to_unrealized_profit.boxed_clone(),
                            mc.boxed_clone(),
                        )
                    })
                })
                .flatten(),
            height_to_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap_height.map(|mc| {
                        LazyVecFrom2::transformed::<PercentageDollarsF32>(
                            &cfg.name("unrealized_loss_rel_to_own_market_cap"),
                            cfg.version + v1,
                            unrealized.height_to_unrealized_loss.boxed_clone(),
                            mc.boxed_clone(),
                        )
                    })
                })
                .flatten(),
            height_to_neg_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap_height.map(|mc| {
                        LazyVecFrom2::transformed::<NegPercentageDollarsF32>(
                            &cfg.name("neg_unrealized_loss_rel_to_own_market_cap"),
                            cfg.version + v1,
                            unrealized.height_to_unrealized_loss.boxed_clone(),
                            mc.boxed_clone(),
                        )
                    })
                })
                .flatten(),
            height_to_net_unrealized_pnl_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap_height.map(|mc| {
                        LazyVecFrom2::transformed::<PercentageDollarsF32>(
                            &cfg.name("net_unrealized_pnl_rel_to_own_market_cap"),
                            cfg.version + v2,
                            unrealized.height_to_net_unrealized_pnl.boxed_clone(),
                            mc.boxed_clone(),
                        )
                    })
                })
                .flatten(),
            indexes_to_unrealized_profit_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap.map(|mc| {
                        LazyVecsFrom2FromDateIndex::from_computed::<PercentageDollarsF32>(
                            &cfg.name("unrealized_profit_rel_to_own_market_cap"),
                            cfg.version + v2,
                            &unrealized.indexes_to_unrealized_profit,
                            mc,
                        )
                    })
                })
                .flatten(),
            indexes_to_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap.map(|mc| {
                        LazyVecsFrom2FromDateIndex::from_computed::<PercentageDollarsF32>(
                            &cfg.name("unrealized_loss_rel_to_own_market_cap"),
                            cfg.version + v2,
                            &unrealized.indexes_to_unrealized_loss,
                            mc,
                        )
                    })
                })
                .flatten(),
            indexes_to_neg_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap.map(|mc| {
                        LazyVecsFrom2FromDateIndex::from_computed::<NegPercentageDollarsF32>(
                            &cfg.name("neg_unrealized_loss_rel_to_own_market_cap"),
                            cfg.version + v2,
                            &unrealized.indexes_to_unrealized_loss,
                            mc,
                        )
                    })
                })
                .flatten(),
            indexes_to_net_unrealized_pnl_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    own_market_cap.map(|mc| {
                        LazyVecsFrom2FromDateIndex::from_computed::<PercentageDollarsF32>(
                            &cfg.name("net_unrealized_pnl_rel_to_own_market_cap"),
                            cfg.version + v2,
                            &unrealized.indexes_to_net_unrealized_pnl,
                            mc,
                        )
                    })
                })
                .flatten(),

            // === Unrealized vs Own Total Unrealized PnL (lazy, optional) ===
            height_to_unrealized_profit_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyVecFrom2::transformed::<Ratio32>(
                    &cfg.name("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                    cfg.version,
                    unrealized.height_to_unrealized_profit.boxed_clone(),
                    unrealized.height_to_total_unrealized_pnl.boxed_clone(),
                )
            }),
            height_to_unrealized_loss_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyVecFrom2::transformed::<Ratio32>(
                    &cfg.name("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version,
                    unrealized.height_to_unrealized_loss.boxed_clone(),
                    unrealized.height_to_total_unrealized_pnl.boxed_clone(),
                )
            }),
            height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyVecFrom2::transformed::<NegRatio32>(
                    &cfg.name("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version,
                    unrealized.height_to_unrealized_loss.boxed_clone(),
                    unrealized.height_to_total_unrealized_pnl.boxed_clone(),
                )
            }),
            height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyVecFrom2::transformed::<Ratio32>(
                    &cfg.name("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    unrealized.height_to_net_unrealized_pnl.boxed_clone(),
                    unrealized.height_to_total_unrealized_pnl.boxed_clone(),
                )
            }),
            indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyVecsFrom2FromDateIndex::from_computed::<Ratio32>(
                    &cfg.name("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.indexes_to_unrealized_profit,
                    &unrealized.indexes_to_total_unrealized_pnl,
                )
            }),
            indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyVecsFrom2FromDateIndex::from_computed::<Ratio32>(
                    &cfg.name("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.indexes_to_unrealized_loss,
                    &unrealized.indexes_to_total_unrealized_pnl,
                )
            }),
            indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyVecsFrom2FromDateIndex::from_computed::<NegRatio32>(
                    &cfg.name("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.indexes_to_unrealized_loss,
                    &unrealized.indexes_to_total_unrealized_pnl,
                )
            }),
            indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: extended.then(|| {
                LazyVecsFrom2FromDateIndex::from_computed::<Ratio32>(
                    &cfg.name("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.indexes_to_net_unrealized_pnl,
                    &unrealized.indexes_to_total_unrealized_pnl,
                )
            }),
        })
    }
}
