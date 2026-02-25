use brk_types::Dollars;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};

use crate::internal::ComputedFromHeightLast;

use crate::distribution::metrics::{ImportConfig, RealizedBase, SupplyMetrics, UnrealizedBase};

use super::{RelativeBase, RelativeExtendedOwnPnl, RelativePeakRegret};

/// Relative metrics for the "all" cohort (base + own_pnl + peak_regret, NO rel_to_all).
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct RelativeForAll {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeBase,
    #[traversable(flatten)]
    pub extended_own_pnl: RelativeExtendedOwnPnl,
    #[traversable(flatten)]
    pub peak_regret: RelativePeakRegret,
}

impl RelativeForAll {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedBase,
        supply: &SupplyMetrics,
        realized_base: &RealizedBase,
        peak_regret: &ComputedFromHeightLast<Dollars>,
    ) -> Self {
        // For the "all" cohort, market_cap = own market cap
        let market_cap = &supply.total.usd;
        Self {
            base: RelativeBase::forced_import(
                cfg, unrealized, supply, market_cap, &realized_base.realized_cap,
            ),
            extended_own_pnl: RelativeExtendedOwnPnl::forced_import(cfg, unrealized),
            peak_regret: RelativePeakRegret::forced_import(cfg, peak_regret, market_cap),
        }
    }
}
