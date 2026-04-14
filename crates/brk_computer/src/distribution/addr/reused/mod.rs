//! Reused address tracking.
//!
//! An address is "reused" if its lifetime `funded_txo_count > 1`, i.e.
//! it has received more than one output across its lifetime. This is
//! the simplest output-multiplicity proxy for address linkability.
//!
//! Two facets are tracked here:
//! - [`count`]: how many distinct addresses are currently reused
//!   (funded) and how many have *ever* been reused (total). Per address
//!   type plus an aggregated `all`.
//! - [`events`]: per-block address-reuse event counts on both sides.
//!   Output-side (`output_to_reused_addr_count`, outputs landing on
//!   addresses that had already received ≥ 1 prior output) and
//!   input-side (`input_from_reused_addr_count`, inputs spending from
//!   addresses with lifetime `funded_txo_count > 1`). Each count is
//!   paired with a percent over the matching block-level output/input
//!   total.

mod count;
mod events;

pub use count::{AddrTypeToReusedAddrCount, ReusedAddrCountsVecs};
pub use events::{AddrTypeToReusedAddrEventCount, ReusedAddrEventsVecs};

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, Rw, StorageMode};

use crate::{
    indexes, inputs,
    internal::{WindowStartVec, Windows},
    outputs,
};

/// Top-level container for all reused address tracking: counts (funded +
/// total) plus per-block reuse events (output-side + input-side).
#[derive(Traversable)]
pub struct ReusedAddrVecs<M: StorageMode = Rw> {
    pub count: ReusedAddrCountsVecs<M>,
    pub events: ReusedAddrEventsVecs<M>,
}

impl ReusedAddrVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        Ok(Self {
            count: ReusedAddrCountsVecs::forced_import(db, version, indexes)?,
            events: ReusedAddrEventsVecs::forced_import(db, version, indexes, cached_starts)?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.count
            .min_stateful_len()
            .min(self.events.min_stateful_len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.count
            .par_iter_height_mut()
            .chain(self.events.par_iter_height_mut())
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.count.reset_height()?;
        self.events.reset_height()?;
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        outputs_by_type: &outputs::ByTypeVecs,
        inputs_by_type: &inputs::ByTypeVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.count.compute_rest(starting_indexes, exit)?;
        self.events.compute_rest(
            starting_indexes,
            outputs_by_type,
            inputs_by_type,
            exit,
        )?;
        Ok(())
    }
}
