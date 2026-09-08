use bitview_cohort::{ADDR_TYPE_IDS, AddrTypeId, WithAddrTypes};
use bitview_collections::Windows;
use bitview_compute::NumericValue;
use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{
    CacheBudget, Ident, PcoVec, PcoVecValue, ReadOnlyColumnarVec, ReadableCloneableVec,
    ReadableColumnarVec, UnaryTransform,
};

use crate::{IndexSources, LazyPreviousDeltaVec, LazyRollingAvgsFromHeight};

/// Lazy exact per-block values and rolling averages backed by one cumulative source.
#[derive(Traversable)]
pub struct LazyPerBlockCumulativeAverage<T, C = T, F = Ident>
where
    T: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
    F: UnaryTransform<C, T>,
{
    /// Value for the represented block. At time-period indexes, the value is
    /// taken from the period's final block.
    pub block: LazyPreviousDeltaVec<Height, C, T, F>,
    #[traversable(flatten)]
    pub average: LazyRollingAvgsFromHeight<C>,
}

impl<T, C, F> Clone for LazyPerBlockCumulativeAverage<T, C, F>
where
    T: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
    F: UnaryTransform<C, T>,
{
    fn clone(&self) -> Self {
        Self {
            block: self.block.clone(),
            average: self.average.clone(),
        }
    }
}

impl<T, C, F> LazyPerBlockCumulativeAverage<T, C, F>
where
    T: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
    F: UnaryTransform<C, T>,
{
    pub fn new(
        name: &str,
        version: Version,
        cumulative: &impl ReadableCloneableVec<Height, C>,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Self {
        let cumulative = cumulative.read_only_boxed_clone();
        Self {
            block: LazyPreviousDeltaVec::transformed(
                name,
                version,
                cumulative.read_only_boxed_clone(),
            ),
            average: LazyRollingAvgsFromHeight::new(
                &format!("{name}_average"),
                version + Version::TWO,
                &cumulative,
                window_starts,
                indexes,
            ),
        }
    }
}

impl<T, C, F> LazyPerBlockCumulativeAverage<T, C, F>
where
    T: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema + PcoVecValue,
    F: UnaryTransform<C, T>,
{
    pub fn with_addr_types(
        cache: &'static CacheBudget,
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, C>, AddrTypeId>,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> WithAddrTypes<Self> {
        let cumulative_name = format!("{name}_cumulative");
        let cumulative = cache.wrap(source.sum_columns(&cumulative_name, version, ADDR_TYPE_IDS));
        let all =
            LazyPerBlockCumulativeAverage::new(name, version, &cumulative, indexes, window_starts);
        let by_addr_type = AddrTypeId::series(|column, type_name| {
            let name = format!("{type_name}_{name}");
            let cumulative =
                cache.wrap(source.column(&format!("{name}_cumulative"), version, column));
            LazyPerBlockCumulativeAverage::new(&name, version, &cumulative, indexes, window_starts)
        });

        WithAddrTypes { all, by_addr_type }
    }
}
