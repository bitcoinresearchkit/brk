use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::distribution::metrics::{ImportConfig, RealizedBase, UnrealizedBase};

use super::{RelativeBase, RelativeToAll};

/// Relative metrics with rel_to_all (no extended, no peak_regret).
/// Used by: epoch, year, type, amount, address cohorts.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct RelativeWithRelToAll<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeBase<M>,
    #[traversable(flatten)]
    pub rel_to_all: RelativeToAll<M>,
}

impl RelativeWithRelToAll {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            base: RelativeBase::forced_import(cfg)?,
            rel_to_all: RelativeToAll::forced_import(cfg)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedBase,
        realized: &RealizedBase,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        market_cap: &impl ReadableVec<Height, Dollars>,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
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
        self.rel_to_all.compute(
            max_from,
            unrealized,
            supply_total_sats,
            all_supply_sats,
            exit,
        )?;
        Ok(())
    }
}
