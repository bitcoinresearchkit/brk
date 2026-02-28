//! Block-count-based rolling distribution — 6-block window.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Height;
use schemars::JsonSchema;
use vecdb::{Database, Rw, StorageMode, Version};

use crate::internal::{ComputedVecValue, Distribution, NumericValue};

/// Single 6-block rolling window distribution with 8 distribution stat vecs.
#[derive(Traversable)]
pub struct BlockRollingDistribution<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "6b")]
    pub _6b: Distribution<Height, T, M>,
}

impl<T> BlockRollingDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            _6b: Distribution::forced_import(db, &format!("{name}_6b"), version)?,
        })
    }
}
