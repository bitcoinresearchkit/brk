//! Lazy stats aggregate pattern (average, min, max, sum, cumulative).

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{FromCoarserIndex, IterableBoxedVec, VecIndex, VecValue};

use super::{LazyAverage, LazyCumulative, LazyMax, LazyMin, LazySum};
use crate::internal::ComputedVecValue;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
pub struct LazyFull<I, T, S1I, S2T>
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
    pub sum: LazySum<I, T, S1I, S2T>,
    #[traversable(flatten)]
    pub cumulative: LazyCumulative<I, T, S1I, S2T>,
}

impl<I, T, S1I, S2T> LazyFull<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1I: VecIndex + 'static + FromCoarserIndex<I>,
    S2T: VecValue,
{
    #[allow(clippy::too_many_arguments)]
    pub fn from_stats_aggregate(
        name: &str,
        version: Version,
        source_average: IterableBoxedVec<S1I, T>,
        source_min: IterableBoxedVec<S1I, T>,
        source_max: IterableBoxedVec<S1I, T>,
        source_sum: IterableBoxedVec<S1I, T>,
        source_cumulative: IterableBoxedVec<S1I, T>,
        len_source: IterableBoxedVec<I, S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            average: LazyAverage::from_source(name, v, source_average, len_source.clone()),
            min: LazyMin::from_source(name, v, source_min, len_source.clone()),
            max: LazyMax::from_source(name, v, source_max, len_source.clone()),
            sum: LazySum::from_source(name, v, source_sum, len_source.clone()),
            cumulative: LazyCumulative::from_source(name, v, source_cumulative, len_source),
        }
    }
}
