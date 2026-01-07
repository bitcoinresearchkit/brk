//! Lazy sum + cumulative aggregation.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{FromCoarserIndex, IterableBoxedVec, VecIndex, VecValue};

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
    pub fn from_sources(
        name: &str,
        version: Version,
        sum_source: IterableBoxedVec<S1I, T>,
        cumulative_source: IterableBoxedVec<S1I, T>,
        len_source: IterableBoxedVec<I, S2T>,
    ) -> Self {
        Self {
            sum: LazySum::from_source(name, version + VERSION, sum_source, len_source.clone()),
            cumulative: LazyCumulative::from_source(
                name,
                version + VERSION,
                cumulative_source,
                len_source,
            ),
        }
    }
}

