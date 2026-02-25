use brk_types::Dollars;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};

use crate::internal::ComputedFromHeightLast;

use crate::distribution::metrics::{ImportConfig, RealizedBase, SupplyMetrics, UnrealizedBase};

use super::{
    RelativeBase, RelativeExtendedOwnMarketCap, RelativeExtendedOwnPnl,
    RelativePeakRegret, RelativeToAll,
};

/// Full extended relative metrics (base + rel_to_all + own_market_cap + own_pnl + peak_regret).
/// Used by: sth, lth, age_range cohorts.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct RelativeWithExtended {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeBase,
    #[traversable(flatten)]
    pub rel_to_all: RelativeToAll,
    #[traversable(flatten)]
    pub extended_own_market_cap: RelativeExtendedOwnMarketCap,
    #[traversable(flatten)]
    pub extended_own_pnl: RelativeExtendedOwnPnl,
    #[traversable(flatten)]
    pub peak_regret: RelativePeakRegret,
}

impl RelativeWithExtended {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedBase,
        supply: &SupplyMetrics,
        all_supply: &SupplyMetrics,
        realized_base: &RealizedBase,
        peak_regret: &ComputedFromHeightLast<Dollars>,
    ) -> Self {
        let market_cap = &all_supply.total.usd;
        let own_market_cap = &supply.total.usd;
        Self {
            base: RelativeBase::forced_import(
                cfg, unrealized, supply, market_cap, &realized_base.realized_cap,
            ),
            rel_to_all: RelativeToAll::forced_import(cfg, unrealized, supply, all_supply),
            extended_own_market_cap: RelativeExtendedOwnMarketCap::forced_import(
                cfg, unrealized, own_market_cap,
            ),
            extended_own_pnl: RelativeExtendedOwnPnl::forced_import(cfg, unrealized),
            peak_regret: RelativePeakRegret::forced_import(cfg, peak_regret, market_cap),
        }
    }
}
