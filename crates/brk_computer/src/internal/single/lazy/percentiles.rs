//! Lazy percentiles composite (pct10, pct25, median, pct75, pct90).

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{FromCoarserIndex, ReadableBoxedVec, VecIndex, VecValue};

use crate::internal::ComputedVecValue;

use super::LazyPercentile;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
pub struct LazyAggPercentiles<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1I: VecIndex,
    S2T: VecValue,
{
    pub pct10: LazyPercentile<I, T, S1I, S2T>,
    pub pct25: LazyPercentile<I, T, S1I, S2T>,
    pub median: LazyPercentile<I, T, S1I, S2T>,
    pub pct75: LazyPercentile<I, T, S1I, S2T>,
    pub pct90: LazyPercentile<I, T, S1I, S2T>,
}

impl<I, T, S1I, S2T> LazyAggPercentiles<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1I: VecIndex + 'static + FromCoarserIndex<I>,
    S2T: VecValue,
{
    pub(crate) fn from_source(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<S1I, T>,
        len_source: ReadableBoxedVec<I, S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            pct10: LazyPercentile::from_source::<10>(&format!("{name}_pct10"), v, source.clone(), len_source.clone()),
            pct25: LazyPercentile::from_source::<25>(&format!("{name}_pct25"), v, source.clone(), len_source.clone()),
            median: LazyPercentile::from_source::<50>(&format!("{name}_median"), v, source.clone(), len_source.clone()),
            pct75: LazyPercentile::from_source::<75>(&format!("{name}_pct75"), v, source.clone(), len_source.clone()),
            pct90: LazyPercentile::from_source::<90>(&format!("{name}_pct90"), v, source, len_source),
        }
    }
}

impl<I, T> LazyAggPercentiles<I, T, Height, Height>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
{
    pub(crate) fn from_height_source(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, T>,
        first_height: ReadableBoxedVec<I, Height>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            pct10: LazyPercentile::from_height_source::<10>(&format!("{name}_pct10"), v, source.clone(), first_height.clone()),
            pct25: LazyPercentile::from_height_source::<25>(&format!("{name}_pct25"), v, source.clone(), first_height.clone()),
            median: LazyPercentile::from_height_source::<50>(&format!("{name}_median"), v, source.clone(), first_height.clone()),
            pct75: LazyPercentile::from_height_source::<75>(&format!("{name}_pct75"), v, source.clone(), first_height.clone()),
            pct90: LazyPercentile::from_height_source::<90>(&format!("{name}_pct90"), v, source, first_height),
        }
    }
}
