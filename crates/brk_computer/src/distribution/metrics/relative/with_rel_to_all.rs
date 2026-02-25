use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};

use crate::distribution::metrics::{ImportConfig, RealizedBase, SupplyMetrics, UnrealizedBase};

use super::{RelativeBase, RelativeToAll};

/// Relative metrics with rel_to_all (no extended, no peak_regret).
/// Used by: epoch, year, type, amount, address cohorts.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct RelativeWithRelToAll {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeBase,
    #[traversable(flatten)]
    pub rel_to_all: RelativeToAll,
}

impl RelativeWithRelToAll {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedBase,
        supply: &SupplyMetrics,
        all_supply: &SupplyMetrics,
        realized_base: &RealizedBase,
    ) -> Self {
        let market_cap = &all_supply.total.usd;
        Self {
            base: RelativeBase::forced_import(
                cfg, unrealized, supply, market_cap, &realized_base.realized_cap,
            ),
            rel_to_all: RelativeToAll::forced_import(cfg, unrealized, supply, all_supply),
        }
    }
}
