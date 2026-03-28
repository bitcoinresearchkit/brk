use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::distribution::metrics::{ImportConfig, SupplyCore, UnrealizedFull};

use super::{RelativeExtendedOwnMarketCap, RelativeExtendedOwnPnl, RelativeFull, RelativeToAll};

/// Full extended relative metrics (base + rel_to_all + own_market_cap + own_pnl).
/// Used by: sth, lth cohorts.
#[derive(Deref, DerefMut, Traversable)]
pub struct RelativeWithExtended<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeFull<M>,
    #[traversable(flatten)]
    pub rel_to_all: RelativeToAll<M>,
    #[traversable(flatten)]
    pub extended_own_market_cap: RelativeExtendedOwnMarketCap<M>,
    #[traversable(flatten)]
    pub extended_own_pnl: RelativeExtendedOwnPnl<M>,
}

impl RelativeWithExtended {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            base: RelativeFull::forced_import(cfg)?,
            rel_to_all: RelativeToAll::forced_import(cfg)?,
            extended_own_market_cap: RelativeExtendedOwnMarketCap::forced_import(cfg)?,
            extended_own_pnl: RelativeExtendedOwnPnl::forced_import(cfg)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        supply: &SupplyCore,
        unrealized: &UnrealizedFull,
        market_cap: &impl ReadableVec<Height, Dollars>,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        own_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.base
            .compute(max_from, supply, &unrealized.inner.basic, market_cap, exit)?;
        self.rel_to_all
            .compute(max_from, supply, all_supply_sats, exit)?;
        self.extended_own_market_cap
            .compute(max_from, &unrealized.inner, own_market_cap, exit)?;
        self.extended_own_pnl.compute(
            max_from,
            &unrealized.inner,
            &unrealized.gross_pnl.usd.height,
            exit,
        )?;
        Ok(())
    }
}
