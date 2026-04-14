//! Address activity tracking - per-block counts of address behaviors.
//!
//! Tracks global and per-address-type activity metrics:
//!
//! | Metric | Description |
//! |--------|-------------|
//! | `receiving` | Unique addresses that received this block |
//! | `sending` | Unique addresses that sent this block |
//! | `reactivated` | Addresses that were empty and now have funds |
//! | `bidirectional` | Addresses that both sent AND received in same block |
//! | `active` | Distinct addresses involved (sent ∪ received) |

use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU32, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, Rw, StorageMode, WritableVec};

use crate::{
    indexes,
    internal::{PerBlockRollingAverage, WindowStartVec, Windows},
};

/// Per-block activity counts - reset each block.
#[derive(Debug, Default, Clone)]
pub struct BlockActivityCounts {
    pub reactivated: u32,
    pub sending: u32,
    pub receiving: u32,
    pub bidirectional: u32,
}

impl BlockActivityCounts {
    /// Reset all counts to zero.
    #[inline]
    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Per-address-type activity counts - aggregated during block processing.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddrTypeToActivityCounts(pub ByAddrType<BlockActivityCounts>);

impl AddrTypeToActivityCounts {
    /// Reset all per-type counts.
    pub(crate) fn reset(&mut self) {
        self.0.values_mut().for_each(|v| v.reset());
    }

    /// Sum all types to get totals.
    pub(crate) fn totals(&self) -> BlockActivityCounts {
        let mut total = BlockActivityCounts::default();
        for counts in self.0.values() {
            total.reactivated += counts.reactivated;
            total.sending += counts.sending;
            total.receiving += counts.receiving;
            total.bidirectional += counts.bidirectional;
        }
        total
    }
}

/// Activity count vectors for a single category (e.g., one address type or "all").
#[derive(Traversable)]
pub struct ActivityCountVecs<M: StorageMode = Rw> {
    pub reactivated: PerBlockRollingAverage<StoredU32, StoredU64, M>,
    pub sending: PerBlockRollingAverage<StoredU32, StoredU64, M>,
    pub receiving: PerBlockRollingAverage<StoredU32, StoredU64, M>,
    pub bidirectional: PerBlockRollingAverage<StoredU32, StoredU64, M>,
    /// Distinct addresses involved in this block (sent ∪ received),
    /// computed at push time as `sending + receiving - bidirectional`
    /// via inclusion-exclusion. For per-type instances this is
    /// per-type. For the `all` aggregate it's the cross-type total.
    pub active: PerBlockRollingAverage<StoredU32, StoredU64, M>,
}

impl ActivityCountVecs {
    /// `prefix` is prepended to each field's disk name. Use `""` for the
    /// "all" aggregate and `"{type}_"` for per-address-type instances.
    /// Field names are suffixed with `_addrs` so the final disk series
    /// are e.g. `active_addrs`, `p2tr_bidirectional_addrs`.
    pub(crate) fn forced_import(
        db: &Database,
        prefix: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        Ok(Self {
            reactivated: PerBlockRollingAverage::forced_import(
                db,
                &format!("{prefix}reactivated_addrs"),
                version,
                indexes,
                cached_starts,
            )?,
            sending: PerBlockRollingAverage::forced_import(
                db,
                &format!("{prefix}sending_addrs"),
                version,
                indexes,
                cached_starts,
            )?,
            receiving: PerBlockRollingAverage::forced_import(
                db,
                &format!("{prefix}receiving_addrs"),
                version,
                indexes,
                cached_starts,
            )?,
            bidirectional: PerBlockRollingAverage::forced_import(
                db,
                &format!("{prefix}bidirectional_addrs"),
                version,
                indexes,
                cached_starts,
            )?,
            active: PerBlockRollingAverage::forced_import(
                db,
                &format!("{prefix}active_addrs"),
                version,
                indexes,
                cached_starts,
            )?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.reactivated
            .block
            .len()
            .min(self.sending.block.len())
            .min(self.receiving.block.len())
            .min(self.bidirectional.block.len())
            .min(self.active.block.len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.reactivated.block as &mut dyn AnyStoredVec,
            &mut self.sending.block,
            &mut self.receiving.block,
            &mut self.bidirectional.block,
            &mut self.active.block,
        ]
        .into_par_iter()
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.reactivated.block.reset()?;
        self.sending.block.reset()?;
        self.receiving.block.reset()?;
        self.bidirectional.block.reset()?;
        self.active.block.reset()?;
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn push_height(&mut self, counts: &BlockActivityCounts) {
        self.reactivated.block.push(counts.reactivated.into());
        self.sending.block.push(counts.sending.into());
        self.receiving.block.push(counts.receiving.into());
        self.bidirectional
            .block
            .push(counts.bidirectional.into());
        let active = counts.sending + counts.receiving - counts.bidirectional;
        self.active.block.push(active.into());
    }

    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()> {
        self.reactivated.compute_rest(max_from, exit)?;
        self.sending.compute_rest(max_from, exit)?;
        self.receiving.compute_rest(max_from, exit)?;
        self.bidirectional.compute_rest(max_from, exit)?;
        self.active.compute_rest(max_from, exit)?;
        Ok(())
    }
}

/// Per-address-type activity count vecs.
#[derive(Deref, DerefMut, Traversable)]
pub struct AddrTypeToActivityCountVecs<M: StorageMode = Rw>(ByAddrType<ActivityCountVecs<M>>);

impl From<ByAddrType<ActivityCountVecs>> for AddrTypeToActivityCountVecs {
    #[inline]
    fn from(value: ByAddrType<ActivityCountVecs>) -> Self {
        Self(value)
    }
}

impl AddrTypeToActivityCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        Ok(Self::from(ByAddrType::<ActivityCountVecs>::new_with_name(
            |type_name| {
                ActivityCountVecs::forced_import(
                    db,
                    &format!("{type_name}_"),
                    version,
                    indexes,
                    cached_starts,
                )
            },
        )?))
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.0
            .values()
            .map(|v| v.min_stateful_len())
            .min()
            .unwrap_or(0)
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        for type_vecs in self.0.values_mut() {
            vecs.push(&mut type_vecs.reactivated.block);
            vecs.push(&mut type_vecs.sending.block);
            vecs.push(&mut type_vecs.receiving.block);
            vecs.push(&mut type_vecs.bidirectional.block);
            vecs.push(&mut type_vecs.active.block);
        }
        vecs.into_par_iter()
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        for v in self.0.values_mut() {
            v.reset_height()?;
        }
        Ok(())
    }

    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()> {
        for type_vecs in self.0.values_mut() {
            type_vecs.compute_rest(max_from, exit)?;
        }
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn push_height(&mut self, counts: &AddrTypeToActivityCounts) {
        for (vecs, c) in self.0.values_mut().zip(counts.0.values()) {
            vecs.push_height(c);
        }
    }
}

/// Storage for activity metrics (global + per type).
#[derive(Traversable)]
pub struct AddrActivityVecs<M: StorageMode = Rw> {
    pub all: ActivityCountVecs<M>,
    #[traversable(flatten)]
    pub by_addr_type: AddrTypeToActivityCountVecs<M>,
}

impl AddrActivityVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        Ok(Self {
            all: ActivityCountVecs::forced_import(db, "", version, indexes, cached_starts)?,
            by_addr_type: AddrTypeToActivityCountVecs::forced_import(
                db,
                version,
                indexes,
                cached_starts,
            )?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.all
            .min_stateful_len()
            .min(self.by_addr_type.min_stateful_len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.all
            .par_iter_height_mut()
            .chain(self.by_addr_type.par_iter_height_mut())
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.all.reset_height()?;
        self.by_addr_type.reset_height()?;
        Ok(())
    }

    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()> {
        self.all.compute_rest(max_from, exit)?;
        self.by_addr_type.compute_rest(max_from, exit)?;
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn push_height(&mut self, counts: &AddrTypeToActivityCounts) {
        let totals = counts.totals();
        self.all.push_height(&totals);
        self.by_addr_type.push_height(counts);
    }
}
