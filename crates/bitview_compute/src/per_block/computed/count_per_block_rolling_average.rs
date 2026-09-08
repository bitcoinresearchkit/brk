use brk_error::Result;

use bitview_traversable::Traversable;
use brk_types::{Height, StoredU32, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, ReadableCloneableVec, Rw, StorageMode};

use crate::{IndexSources, StoredU64ToStoredU32, Windows};

use super::PerBlockCumulativeAverage;

#[derive(Deref, DerefMut, Traversable)]
pub struct CountPerBlockRollingAverage<M: StorageMode = Rw>(
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    PerBlockCumulativeAverage<StoredU32, StoredU64, M, StoredU64ToStoredU32>,
);

impl CountPerBlockRollingAverage {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Result<Self> {
        PerBlockCumulativeAverage::forced_import(db, name, version, indexes, window_starts)
            .map(Self)
    }
}
