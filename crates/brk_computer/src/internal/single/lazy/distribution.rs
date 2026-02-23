//! Lazy distribution pattern (average, min, max + percentiles).

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{FromCoarserIndex, ReadableBoxedVec, VecIndex, VecValue};

use super::{LazyAggPercentiles, LazyAverage, LazyMax, LazyMin};
use crate::internal::ComputedVecValue;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
pub struct LazyDistribution<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1I: VecIndex,
    S2T: VecValue,
{
    #[traversable(flatten)]
    pub average: LazyAverage<I, T, S1I, S2T>,
    #[traversable(flatten)]
    pub min: LazyMin<I, T, S1I, S2T>,
    #[traversable(flatten)]
    pub max: LazyMax<I, T, S1I, S2T>,
    #[traversable(flatten)]
    pub percentiles: LazyAggPercentiles<I, T, S1I, S2T>,
}

impl<I, T, S1I, S2T> LazyDistribution<I, T, S1I, S2T>
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
            average: LazyAverage::from_source(name, v, source.clone(), len_source.clone()),
            min: LazyMin::from_source(name, v, source.clone(), len_source.clone()),
            max: LazyMax::from_source(name, v, source.clone(), len_source.clone()),
            percentiles: LazyAggPercentiles::from_source(name, v, source, len_source),
        }
    }
}

impl<I, T> LazyDistribution<I, T, Height, Height>
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
            average: LazyAverage::from_height_source(name, v, source.clone(), first_height.clone()),
            min: LazyMin::from_height_source(name, v, source.clone(), first_height.clone()),
            max: LazyMax::from_height_source(name, v, source.clone(), first_height.clone()),
            percentiles: LazyAggPercentiles::from_height_source(name, v, source, first_height),
        }
    }
}
