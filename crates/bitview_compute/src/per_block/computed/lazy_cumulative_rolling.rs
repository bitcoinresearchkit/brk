//! Lazy counterpart to `PerBlockCumulativeRolling`.

use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{ColumnId, ReadableCloneableVec};

use crate::{
    Identity, IndexSources, LazyColumnPerBlock, LazyPerBlock, LazyPreviousDeltaVec,
    LazyRollingAvgsFromHeight, LazyRollingSumsFromHeight, NumericValue, Windows,
};

#[derive(Clone, Traversable)]
pub struct LazyPerBlockCumulativeRolling<T>
where
    T: NumericValue + JsonSchema,
{
    /// Value for the represented block. At time-period indexes, the value is
    /// taken from the period's final block.
    pub block: LazyPreviousDeltaVec<Height, T>,
    /// Cumulative value through the represented block. At time-period indexes,
    /// the value is taken at the period's final block.
    pub cumulative: LazyPerBlock<T>,
    pub sum: LazyRollingSumsFromHeight<T>,
    pub average: LazyRollingAvgsFromHeight<T>,
}

impl<T> LazyPerBlockCumulativeRolling<T>
where
    T: NumericValue + JsonSchema,
{
    fn from_cumulative(
        name: &str,
        version: Version,
        cumulative: LazyPerBlock<T>,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self {
        let source = &cumulative.height;
        let block = LazyPreviousDeltaVec::new(name, version, source.read_only_boxed_clone());
        let sum = LazyRollingSumsFromHeight::new(
            &format!("{name}_sum"),
            version,
            source,
            window_starts,
            indexes,
        );
        let average = LazyRollingAvgsFromHeight::new(
            &format!("{name}_average"),
            version,
            source,
            window_starts,
            indexes,
        );

        Self {
            block,
            cumulative,
            sum,
            average,
        }
    }

    pub fn from_cumulative_source<V>(
        name: &str,
        version: Version,
        source: &V,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self
    where
        V: ReadableCloneableVec<Height, T> + ?Sized,
    {
        let cumulative = LazyPerBlock::from_height_source::<Identity<T>>(
            &format!("{name}_cumulative"),
            version,
            source,
            indexes,
        );

        Self::from_cumulative(name, version, cumulative, window_starts, indexes)
    }

    pub fn from_lazy_source(
        name: &str,
        version: Version,
        source: &LazyPerBlock<T>,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self {
        let cumulative = LazyPerBlock::from_lazy::<Identity<T>, T>(
            &format!("{name}_cumulative"),
            version,
            source,
        );

        Self::from_cumulative(name, version, cumulative, window_starts, indexes)
    }

    pub fn from_column_source<C: ColumnId>(
        name: &str,
        version: Version,
        source: &LazyColumnPerBlock<T, C>,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self {
        let cumulative = LazyPerBlock::from_resolutions::<Identity<T>>(
            &format!("{name}_cumulative"),
            version,
            &source.resolutions,
        );

        Self::from_cumulative(name, version, cumulative, window_starts, indexes)
    }
}
