use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{ColumnId, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec};

use super::LazyColumnPerBlock;
use crate::{
    IndexSources, LazyPreviousDeltaVec, LazyRollingAvgsFromHeight, LazyRollingSumsFromHeight,
    NumericValue, Windows,
};

#[derive(Clone, Traversable)]
pub struct LazyColumnPerBlockCumulativeRolling<T, C>
where
    T: NumericValue + JsonSchema,
    C: ColumnId,
{
    /// Value for the represented block. At time-period indexes, the value is
    /// taken from the period's final block.
    pub block: LazyPreviousDeltaVec<Height, T>,
    /// Cumulative value through the represented block. At time-period indexes,
    /// the value is taken at the period's final block.
    pub cumulative: LazyColumnPerBlock<T, C>,
    pub sum: LazyRollingSumsFromHeight<T>,
    pub average: LazyRollingAvgsFromHeight<T>,
}

impl<T, C> LazyColumnPerBlockCumulativeRolling<T, C>
where
    T: NumericValue + JsonSchema,
    C: ColumnId,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, T>, C>,
        column: C,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Self {
        let cumulative = LazyColumnPerBlock::new(
            &format!("{name}_cumulative"),
            version,
            source,
            column,
            indexes,
        );
        let source = cumulative.resolutions.height_source();
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
}
