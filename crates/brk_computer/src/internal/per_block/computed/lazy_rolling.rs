use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{ReadableCloneableVec, UnaryTransform};

use crate::{
    indexes,
    internal::{
        ComputedVecValue, LazyPerBlock, LazyRollingComplete, NumericValue, PerBlockFull,
        WindowStartVec, Windows,
    },
};

/// Lazy analog of `PerBlockRolling<T>`: lazy cumulative + lazy rolling complete.
/// Derived by transforming a `PerBlockFull<S1T>`. Zero stored vecs.
#[derive(Clone, Traversable)]
pub struct LazyPerBlockRolling<T, S1T>
where
    T: NumericValue + JsonSchema,
    S1T: ComputedVecValue + JsonSchema,
{
    pub cumulative: LazyPerBlock<T, S1T>,
    #[traversable(flatten)]
    pub rolling: LazyRollingComplete<T, S1T>,
}

impl<T, S1T> LazyPerBlockRolling<T, S1T>
where
    T: NumericValue + JsonSchema + 'static,
    S1T: NumericValue + JsonSchema,
{
    pub(crate) fn from_per_block_full<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &PerBlockFull<S1T>,
        cached_starts: &Windows<&WindowStartVec>,
        indexes: &indexes::Vecs,
    ) -> Self {
        let cumulative = LazyPerBlock::from_computed::<F>(
            &format!("{name}_cumulative"),
            version,
            source.cumulative.height.read_only_boxed_clone(),
            &source.cumulative,
        );

        let rolling = LazyRollingComplete::from_rolling_complete::<F>(
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
