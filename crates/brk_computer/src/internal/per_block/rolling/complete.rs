//! RollingComplete - Lazy rolling sums + stored rolling distribution per window.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        CachedWindowStarts, NumericValue, LazyRollingSumsFromHeight, RollingDistribution,
        WindowStarts,
    },
};

/// Lazy rolling sums + stored rolling distribution (8 stats × 4 windows).
#[derive(Traversable)]
pub struct RollingComplete<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub sum: LazyRollingSumsFromHeight<T>,
    #[traversable(flatten)]
    pub distribution: RollingDistribution<T, M>,
}

impl<T> RollingComplete<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cumulative: &(impl ReadableCloneableVec<Height, T> + 'static),
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let sum = LazyRollingSumsFromHeight::new(
            &format!("{name}_sum"),
            version,
            cumulative,
            cached_starts,
            indexes,
        );
        let distribution = RollingDistribution::forced_import(db, name, version, indexes)?;

        Ok(Self { sum, distribution })
    }

    /// Compute rolling distribution stats across all 4 windows.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        source: &impl ReadableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<f64> + Default + Copy + Ord,
        f64: From<T>,
    {
        self.distribution
            .compute_distribution(max_from, windows, source, exit)
    }
}
