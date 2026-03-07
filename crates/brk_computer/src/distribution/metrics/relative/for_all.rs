use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::distribution::metrics::{ImportConfig, RealizedBase, UnrealizedBase};

use super::{RelativeFull, RelativeExtendedOwnPnl};

/// Relative metrics for the "all" cohort (base + own_pnl, NO rel_to_all).
#[derive(Deref, DerefMut, Traversable)]
pub struct RelativeForAll<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeFull<M>,
    #[traversable(flatten)]
    pub extended_own_pnl: RelativeExtendedOwnPnl<M>,
}

impl RelativeForAll {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            base: RelativeFull::forced_import(cfg)?,
            extended_own_pnl: RelativeExtendedOwnPnl::forced_import(cfg)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedBase,
        realized: &RealizedBase,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.base.compute(
            max_from,
            unrealized,
            realized,
            supply_total_sats,
            market_cap,
            exit,
        )?;
        self.extended_own_pnl.compute(max_from, unrealized, exit)?;
        Ok(())
    }
}
