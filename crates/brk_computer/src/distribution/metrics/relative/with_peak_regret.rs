use brk_types::Dollars;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};

use crate::internal::ComputedFromHeightLast;

use crate::distribution::metrics::{ImportConfig, RealizedBase, SupplyMetrics, UnrealizedBase};

use super::{RelativeBase, RelativePeakRegret, RelativeToAll};

/// Relative metrics with rel_to_all + peak_regret (no extended).
/// Used by: max_age, min_age cohorts.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct RelativeWithPeakRegret {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeBase,
    #[traversable(flatten)]
    pub rel_to_all: RelativeToAll,
    #[traversable(flatten)]
    pub peak_regret: RelativePeakRegret,
}

impl RelativeWithPeakRegret {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedBase,
        supply: &SupplyMetrics,
        all_supply: &SupplyMetrics,
        realized_base: &RealizedBase,
        peak_regret: &ComputedFromHeightLast<Dollars>,
    ) -> Self {
        let market_cap = &all_supply.total.usd;
        Self {
            base: RelativeBase::forced_import(
                cfg, unrealized, supply, market_cap, &realized_base.realized_cap,
            ),
            rel_to_all: RelativeToAll::forced_import(cfg, unrealized, supply, all_supply),
            peak_regret: RelativePeakRegret::forced_import(cfg, peak_regret, market_cap),
        }
    }
}
