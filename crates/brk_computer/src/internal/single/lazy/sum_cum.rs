//! Lazy sum + cumulative aggregation.

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{FromCoarserIndex, ReadableBoxedVec, VecIndex, VecValue};

use crate::internal::{ComputedVecValue, LazyCumulative, LazySum};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
pub struct LazySumCum<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1I: VecIndex,
    S2T: VecValue,
{
    #[traversable(flatten)]
    pub sum: LazySum<I, T, S1I, S2T>,
    #[traversable(flatten)]
    pub cumulative: LazyCumulative<I, T, S1I, S2T>,
}

impl<I, T, S1I, S2T> LazySumCum<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1I: VecIndex + 'static + FromCoarserIndex<I>,
    S2T: VecValue,
{
    /// Create from sources without adding _sum suffix to sum vec.
    pub(crate) fn from_sources_sum_raw(
        name: &str,
        version: Version,
        sum_source: ReadableBoxedVec<S1I, T>,
        cumulative_source: ReadableBoxedVec<S1I, T>,
        len_source: ReadableBoxedVec<I, S2T>,
    ) -> Self {
        Self {
            sum: LazySum::from_source_raw(name, version + VERSION, sum_source, len_source.clone()),
            cumulative: LazyCumulative::from_source(
                name,
                version + VERSION,
                cumulative_source,
                len_source,
            ),
        }
    }
}

impl<I, T> LazySumCum<I, T, Height, Height>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
{
    pub(crate) fn from_height_sources_sum_raw(
        name: &str,
        version: Version,
        sum_source: ReadableBoxedVec<Height, T>,
        cumulative_source: ReadableBoxedVec<Height, T>,
        first_height: ReadableBoxedVec<I, Height>,
    ) -> Self {
        Self {
            sum: LazySum::from_height_source_raw(
                name,
                version + VERSION,
                sum_source,
                first_height.clone(),
            ),
            cumulative: LazyCumulative::from_height_source(
                name,
                version + VERSION,
                cumulative_source,
                first_height,
            ),
        }
    }
}
