//! Reused address tracking.
//!
//! An address is "reused" if its lifetime `funded_txo_count > 1` — i.e. it
//! has received more than one output across its lifetime. This is the
//! simplest output-multiplicity proxy for address linkability.
//!
//! Two facets are tracked here:
//! - [`count`] — how many distinct addresses are currently reused (funded)
//!   and how many have *ever* been reused (total). Per address type plus
//!   an aggregated `all`.
//! - [`uses`] — per-block count of outputs going to addresses that were
//!   already reused, plus the derived percent over total address-output
//!   count (denominator from `outputs::by_type`).

mod count;
mod uses;

pub use count::{AddrTypeToReusedAddrCount, ReusedAddrCountsVecs};
pub use uses::{AddrTypeToReusedAddrUseCount, ReusedAddrUsesVecs};

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{WindowStartVec, Windows},
    outputs,
};

/// Top-level container for all reused address tracking: counts (funded +
/// total) plus per-block uses (count + percent).
#[derive(Traversable)]
pub struct ReusedAddrVecs<M: StorageMode = Rw> {
    pub count: ReusedAddrCountsVecs<M>,
    pub uses: ReusedAddrUsesVecs<M>,
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
            uses: ReusedAddrUsesVecs::forced_import(db, version, indexes, cached_starts)?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.count
            .min_stateful_len()
            .min(self.uses.min_stateful_len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.count
            .par_iter_height_mut()
            .chain(self.uses.par_iter_height_mut())
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.count.reset_height()?;
        self.uses.reset_height()?;
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        outputs_by_type: &outputs::ByTypeVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.count.compute_rest(starting_indexes, exit)?;
        self.uses
            .compute_rest(starting_indexes, outputs_by_type, exit)?;
        Ok(())
    }
}
