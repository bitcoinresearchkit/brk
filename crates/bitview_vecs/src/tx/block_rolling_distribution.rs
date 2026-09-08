use bitview_compute::{ComputedVecValue, NumericValue};
use bitview_traversable::Traversable;
use brk_error::Result;
use schemars::JsonSchema;
use vecdb::{CacheBudget, Database, Rw, StorageMode, Version};

use crate::{IndexSources, PerBlockDistribution};

#[derive(Traversable)]
pub struct BlockRollingDistribution<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    /// Uses the six-block window ending at the represented block.
    pub _6b: PerBlockDistribution<T, M>,
}

impl<T> BlockRollingDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        Ok(Self {
            _6b: PerBlockDistribution::forced_import(
                cache,
                db,
                &format!("{name}_6b"),
                version,
                indexes,
            )?,
        })
    }
}
