use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{PerBlock, WithAddrTypes},
};

use super::AddrTypeToAddrCount;

/// Per-block `StoredU64` counts with an aggregate `all` plus a per-address-type
/// breakdown. Shared primitive backing addr-count, empty-addr-count, and the
/// funded/total pairs used by exposed, reused, and respent.
#[derive(Deref, DerefMut, Traversable)]
pub struct AddrCountsVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub WithAddrTypes<PerBlock<StoredU64, M>>,
);

impl AddrCountsVecs {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(WithAddrTypes::<PerBlock<StoredU64>>::forced_import(
            db, name, version, indexes,
        )?))
    }

    #[inline(always)]
    pub(crate) fn push_counts(&mut self, counts: &AddrTypeToAddrCount) {
        self.push_height(counts.sum(), counts.values().copied());
    }
}
