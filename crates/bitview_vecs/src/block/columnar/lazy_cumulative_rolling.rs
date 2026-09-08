use crate::RollingTotals;
use bitview_cohort::{ADDR_TYPE_IDS, AddrTypeId, WithAddrTypes};
use bitview_collections::Windows;
use bitview_compute::NumericValue;
use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    CacheBudget, ColumnId, PcoVec, PcoVecValue, PinnedCachedVec, ReadOnlyColumnarVec,
    ReadableCloneableVec, ReadableColumnarVec,
};

use crate::{
    IndexSources, LazyColumnPerBlock, LazyPerBlockCumulativeRolling, LazyPreviousDeltaVec,
};

#[derive(Clone, Deref, DerefMut, Traversable)]
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
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rolling: RollingTotals<T>,
}

impl<T, C> LazyColumnPerBlockCumulativeRolling<T, C>
where
    T: NumericValue + JsonSchema,
    C: ColumnId,
{
    pub fn new(
        cache: &'static CacheBudget,
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, T>, C>,
        column: C,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Self {
        let cumulative = LazyColumnPerBlock::new(
            cache,
            &format!("{name}_cumulative"),
            version,
            source,
            column,
            indexes,
        );
        let source = cumulative.resolutions.height_source();
        let block = LazyPreviousDeltaVec::new(name, version, source.read_only_boxed_clone());
        let rolling = RollingTotals::new(name, version, source, window_starts, indexes);

        Self {
            block,
            cumulative,
            rolling,
        }
    }
}

impl<T> LazyColumnPerBlockCumulativeRolling<T, AddrTypeId>
where
    T: NumericValue + JsonSchema + PcoVecValue,
{
    pub fn with_addr_types(
        cache: &'static CacheBudget,
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, T>, AddrTypeId>,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> WithAddrTypes<Self, LazyPerBlockCumulativeRolling<T>> {
        let cumulative = PinnedCachedVec::wrap(source.sum_columns(
            &format!("{name}_cumulative"),
            version,
            ADDR_TYPE_IDS,
        ));
        let all = LazyPerBlockCumulativeRolling::from_cumulative_source(
            name,
            version,
            &cumulative,
            window_starts,
            indexes,
        );
        let by_addr_type = AddrTypeId::series(|column, type_name| {
            LazyColumnPerBlockCumulativeRolling::new(
                cache,
                &format!("{type_name}_{name}"),
                version,
                source,
                column,
                indexes,
                window_starts,
            )
        });

        WithAddrTypes { all, by_addr_type }
    }
}
