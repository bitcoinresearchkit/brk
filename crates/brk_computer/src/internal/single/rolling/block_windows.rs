//! Block-count-based rolling window starts and distribution — 1h and 24h (actual time-based).
//!
//! Uses stored height-ago vecs (`height_1h_ago`, `height_24h_ago`) for accurate
//! time-based window starts.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Height;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, PcoVec, Rw, StorageMode, Version};

use crate::internal::{BlockWindows, ComputedVecValue, Distribution, NumericValue};

/// Rolling window start heights for tx-derived metrics (1h, 24h).
pub struct BlockWindowStarts<'a> {
    pub _1h: &'a EagerVec<PcoVec<Height, Height>>,
    pub _24h: &'a EagerVec<PcoVec<Height, Height>>,
}

/// 2 rolling window distributions (1h, 24h), each with 8 distribution stat vecs.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct BlockRollingDistribution<T, M: StorageMode = Rw>(
    pub BlockWindows<Distribution<Height, T, M>>,
)
where
    T: ComputedVecValue + PartialOrd + JsonSchema;

impl<T> BlockRollingDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self(BlockWindows {
            _1h: Distribution::forced_import(db, &format!("{name}_1h"), version)?,
            _24h: Distribution::forced_import(db, &format!("{name}_24h"), version)?,
        }))
    }
}
