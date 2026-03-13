use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{ReadableCloneableVec, UnaryTransform};

use crate::{
    indexes,
    internal::{
        CachedWindowStarts, ComputedPerBlockFull, ComputedVecValue, LazyPerBlock, LazyRollingFull,
        NumericValue,
    },
};

/// Lazy analog of `ResolutionsFull<T>`: lazy cumulative + lazy rolling full.
/// Derived by transforming a `ComputedPerBlockFull<S1T>`. Zero stored vecs.
#[derive(Clone, Traversable)]
pub struct LazyResolutionsFull<T, S1T>
where
    T: NumericValue + JsonSchema,
    S1T: ComputedVecValue + JsonSchema,
{
    pub cumulative: LazyPerBlock<T, S1T>,
    #[traversable(flatten)]
    pub rolling: LazyRollingFull<T, S1T>,
}

impl<T, S1T> LazyResolutionsFull<T, S1T>
where
    T: NumericValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
{
    pub(crate) fn from_computed_per_block_full<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &ComputedPerBlockFull<S1T>,
        cached_starts: &CachedWindowStarts,
        indexes: &indexes::Vecs,
    ) -> Self {
        let cumulative = LazyPerBlock::from_computed::<F>(
            &format!("{name}_cumulative"),
            version,
            source.cumulative.height.read_only_boxed_clone(),
            &source.cumulative,
        );

        let rolling = LazyRollingFull::from_rolling_full::<F>(
            name,
            version,
            &cumulative.height,
            &source.rolling,
            cached_starts,
            indexes,
        );

        Self {
            cumulative,
            rolling,
        }
    }
}
