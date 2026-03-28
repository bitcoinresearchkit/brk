use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::distribution::metrics::{ImportConfig, SupplyCore, UnrealizedFull};

use super::{RelativeExtendedOwnPnl, RelativeFull};

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
        supply: &SupplyCore,
        unrealized: &UnrealizedFull,
        market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.base
            .compute(max_from, supply, &unrealized.inner.basic, market_cap, exit)?;
        self.extended_own_pnl.compute(
            max_from,
            &unrealized.inner,
            &unrealized.gross_pnl.usd.height,
            exit,
        )?;
        Ok(())
    }
}
