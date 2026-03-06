use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::distribution::metrics::{ImportConfig, UnrealizedComplete};

use super::{RelativeComplete, RelativeToAll};

/// Complete relative metrics with rel_to_all.
/// Used by CompleteCohortMetrics (epoch, class, min_age, max_age).
#[derive(Deref, DerefMut, Traversable)]
pub struct RelativeCompleteWithRelToAll<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeComplete<M>,
    #[traversable(flatten)]
    pub rel_to_all: RelativeToAll<M>,
}

impl RelativeCompleteWithRelToAll {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            base: RelativeComplete::forced_import(cfg)?,
            rel_to_all: RelativeToAll::forced_import(cfg)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedComplete,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        market_cap: &impl ReadableVec<Height, Dollars>,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.base.compute(
            max_from,
            unrealized,
            supply_total_sats,
            market_cap,
            exit,
        )?;
        self.rel_to_all.compute(
            max_from,
            &unrealized.supply_in_profit.sats.height,
            &unrealized.supply_in_loss.sats.height,
            supply_total_sats,
            all_supply_sats,
            exit,
        )?;
        Ok(())
    }
}
