use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{ReadableCloneableVec, UnaryTransform};

use brk_types::Height;

use crate::{
    indexes,
    internal::{
        CachedWindowStarts, ComputedVecValue, LazyRollingDistribution, LazyRollingSumsFromHeight,
        NumericValue, RollingFull,
    },
};

/// Lazy analog of `RollingFull<T>`: lazy rolling sums + lazy rolling distribution.
/// Zero stored vecs.
#[derive(Clone, Traversable)]
pub struct LazyRollingFull<T, S1T>
where
    T: NumericValue + JsonSchema,
    S1T: ComputedVecValue + JsonSchema,
{
    pub sum: LazyRollingSumsFromHeight<T>,
    #[traversable(flatten)]
    pub distribution: LazyRollingDistribution<T, S1T>,
}

impl<T, S1T> LazyRollingFull<T, S1T>
where
    T: NumericValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
{
    pub(crate) fn from_rolling_full<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        cumulative: &(impl ReadableCloneableVec<Height, T> + 'static),
        source: &RollingFull<S1T>,
        cached_starts: &CachedWindowStarts,
        indexes: &indexes::Vecs,
    ) -> Self {
        let sum = LazyRollingSumsFromHeight::new(
            &format!("{name}_sum"),
            version,
            cumulative,
            cached_starts,
            indexes,
        );
        let distribution = LazyRollingDistribution::from_rolling_distribution::<F>(
            name,
            version,
            &source.distribution,
        );
        Self { sum, distribution }
    }
}
