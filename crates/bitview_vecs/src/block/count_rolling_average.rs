use bitview_collections::Windows;
use bitview_transforms::StoredU64ToStoredU32;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, StoredU32, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, Database, ReadableCloneableVec, Rw, StorageMode};

use crate::{IndexSources, PerBlockCumulativeAverage};

#[derive(Deref, DerefMut, Traversable)]
pub struct CountPerBlockRollingAverage<M: StorageMode = Rw>(
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    PerBlockCumulativeAverage<StoredU32, StoredU64, M, StoredU64ToStoredU32>,
);

impl CountPerBlockRollingAverage {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Result<Self> {
        PerBlockCumulativeAverage::forced_import(cache, db, name, version, indexes, window_starts)
            .map(Self)
    }
}
