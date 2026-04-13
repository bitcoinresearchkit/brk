//! Reused address count tracking — running counters of how many addresses
//! are currently in (or have ever been in) the reused set, per address type
//! plus an aggregated `all`. See the parent [`super`] module for the
//! definition of "reused".
//!
//! Two counters are exposed:
//! - `funded`: addresses currently funded AND with `funded_txo_count > 1`
//! - `total`: addresses that have ever satisfied `funded_txo_count > 1` (monotonic)

mod state;
mod vecs;

pub use state::AddrTypeToReusedAddrCount;
pub use vecs::ReusedAddrCountAllVecs;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, Rw, StorageMode};

use crate::indexes;

/// Reused address counts: funded (currently with balance) and total (ever reused).
#[derive(Traversable)]
pub struct ReusedAddrCountsVecs<M: StorageMode = Rw> {
    pub funded: ReusedAddrCountAllVecs<M>,
    pub total: ReusedAddrCountAllVecs<M>,
}

impl ReusedAddrCountsVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            funded: ReusedAddrCountAllVecs::forced_import(
                db,
                "reused_addr_count",
                version,
                indexes,
            )?,
            total: ReusedAddrCountAllVecs::forced_import(
                db,
                "total_reused_addr_count",
                version,
                indexes,
            )?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.funded
            .min_stateful_len()
            .min(self.total.min_stateful_len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.funded
            .par_iter_height_mut()
            .chain(self.total.par_iter_height_mut())
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.funded.reset_height()?;
        self.total.reset_height()?;
        Ok(())
    }

    pub(crate) fn compute_rest(&mut self, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.funded.compute_rest(starting_indexes, exit)?;
        self.total.compute_rest(starting_indexes, exit)?;
        Ok(())
    }
}
