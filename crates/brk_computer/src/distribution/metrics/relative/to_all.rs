use brk_traversable::Traversable;
use brk_types::{Sats, StoredF64, Version};

use crate::internal::{LazyBinaryFromHeightLast, PercentageSatsF64};

use crate::distribution::metrics::{ImportConfig, SupplyMetrics, UnrealizedBase};

/// Relative-to-all metrics (not present for the "all" cohort itself).
#[derive(Clone, Traversable)]
pub struct RelativeToAll {
    pub supply_rel_to_circulating_supply:
        LazyBinaryFromHeightLast<StoredF64, Sats, Sats>,
    pub supply_in_profit_rel_to_circulating_supply:
        LazyBinaryFromHeightLast<StoredF64, Sats, Sats>,
    pub supply_in_loss_rel_to_circulating_supply:
        LazyBinaryFromHeightLast<StoredF64, Sats, Sats>,
}

impl RelativeToAll {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedBase,
        supply: &SupplyMetrics,
        all_supply: &SupplyMetrics,
    ) -> Self {
        let v1 = Version::ONE;
        let gs = &all_supply.total.sats;

        Self {
            supply_rel_to_circulating_supply:
                LazyBinaryFromHeightLast::from_computed_last::<PercentageSatsF64>(
                    &cfg.name("supply_rel_to_circulating_supply"),
                    cfg.version + v1,
                    &supply.total.sats,
                    gs,
                ),
            supply_in_profit_rel_to_circulating_supply:
                LazyBinaryFromHeightLast::from_computed_last::<PercentageSatsF64>(
                    &cfg.name("supply_in_profit_rel_to_circulating_supply"),
                    cfg.version + v1,
                    &unrealized.supply_in_profit.sats,
                    gs,
                ),
            supply_in_loss_rel_to_circulating_supply:
                LazyBinaryFromHeightLast::from_computed_last::<PercentageSatsF64>(
                    &cfg.name("supply_in_loss_rel_to_circulating_supply"),
                    cfg.version + v1,
                    &unrealized.supply_in_loss.sats,
                    gs,
                ),
        }
    }
}
