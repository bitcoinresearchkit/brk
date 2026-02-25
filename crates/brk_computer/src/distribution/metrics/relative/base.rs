use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Sats, StoredF32, StoredF64, Version};

use crate::internal::{
    LazyBinaryComputedFromHeightLast, LazyBinaryFromHeightLast, LazyFromHeightLast,
    NegPercentageDollarsF32, PercentageDollarsF32, PercentageSatsF64,
};

use crate::distribution::metrics::{ImportConfig, SupplyMetrics, UnrealizedBase};

/// Base relative metrics (always computed when relative is enabled).
/// All fields are non-Optional - market_cap and realized_cap are always
/// available when relative metrics are enabled.
#[derive(Clone, Traversable)]
pub struct RelativeBase {
    // === Supply in Profit/Loss Relative to Own Supply ===
    pub supply_in_profit_rel_to_own_supply: LazyBinaryFromHeightLast<StoredF64, Sats, Sats>,
    pub supply_in_loss_rel_to_own_supply: LazyBinaryFromHeightLast<StoredF64, Sats, Sats>,

    // === Unrealized vs Market Cap ===
    pub unrealized_profit_rel_to_market_cap: LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub unrealized_loss_rel_to_market_cap: LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub neg_unrealized_loss_rel_to_market_cap:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub net_unrealized_pnl_rel_to_market_cap:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub nupl: LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,

    // === Invested Capital in Profit/Loss as % of Realized Cap ===
    pub invested_capital_in_profit_pct: LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub invested_capital_in_loss_pct: LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
}

impl RelativeBase {
    /// Import base relative metrics.
    ///
    /// `market_cap` is either `all_supply.total.usd` (for non-"all" cohorts)
    /// or `supply.total.usd` (for the "all" cohort itself).
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedBase,
        supply: &SupplyMetrics,
        market_cap: &LazyBinaryComputedFromHeightLast<Dollars, Sats, Dollars>,
        realized_cap: &LazyFromHeightLast<Dollars, Cents>,
    ) -> Self {
        let v1 = Version::ONE;
        let v2 = Version::new(2);

        Self {
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

            unrealized_profit_rel_to_market_cap:
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    PercentageDollarsF32, _, _,
                >(
                    &cfg.name("unrealized_profit_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_profit,
                    market_cap,
                ),
            unrealized_loss_rel_to_market_cap:
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    PercentageDollarsF32, _, _,
                >(
                    &cfg.name("unrealized_loss_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_loss,
                    market_cap,
                ),
            neg_unrealized_loss_rel_to_market_cap:
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    NegPercentageDollarsF32, _, _,
                >(
                    &cfg.name("neg_unrealized_loss_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_loss,
                    market_cap,
                ),
            net_unrealized_pnl_rel_to_market_cap:
                LazyBinaryFromHeightLast::from_binary_block_and_lazy_binary_block_last::<
                    PercentageDollarsF32, _, _, _, _,
                >(
                    &cfg.name("net_unrealized_pnl_rel_to_market_cap"),
                    cfg.version + v2,
                    &unrealized.net_unrealized_pnl,
                    market_cap,
                ),
            nupl:
                LazyBinaryFromHeightLast::from_binary_block_and_lazy_binary_block_last::<
                    PercentageDollarsF32, _, _, _, _,
                >(
                    &cfg.name("nupl"),
                    cfg.version + v2,
                    &unrealized.net_unrealized_pnl,
                    market_cap,
                ),

            invested_capital_in_profit_pct:
                LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<
                    PercentageDollarsF32, _,
                >(
                    &cfg.name("invested_capital_in_profit_pct"),
                    cfg.version,
                    &unrealized.invested_capital_in_profit,
                    realized_cap,
                ),
            invested_capital_in_loss_pct:
                LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<
                    PercentageDollarsF32, _,
                >(
                    &cfg.name("invested_capital_in_loss_pct"),
                    cfg.version,
                    &unrealized.invested_capital_in_loss,
                    realized_cap,
                ),
        }
    }
}
