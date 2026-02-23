//! Lazy unary transform for Percentiles.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{ReadableBoxedVec, LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::ComputedVecValue;

#[derive(Clone, Traversable)]
pub struct LazyPercentiles<I, T, S1T = T>
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub pct10: LazyVecFrom1<I, T, I, S1T>,
    pub pct25: LazyVecFrom1<I, T, I, S1T>,
    pub median: LazyVecFrom1<I, T, I, S1T>,
    pub pct75: LazyVecFrom1<I, T, I, S1T>,
    pub pct90: LazyVecFrom1<I, T, I, S1T>,
}

impl<I, T, S1T> LazyPercentiles<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_boxed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        pct10: ReadableBoxedVec<I, S1T>,
        pct25: ReadableBoxedVec<I, S1T>,
        median: ReadableBoxedVec<I, S1T>,
        pct75: ReadableBoxedVec<I, S1T>,
        pct90: ReadableBoxedVec<I, S1T>,
    ) -> Self {
        Self {
            pct10: LazyVecFrom1::transformed::<F>(&format!("{name}_pct10"), version, pct10),
            pct25: LazyVecFrom1::transformed::<F>(&format!("{name}_pct25"), version, pct25),
            median: LazyVecFrom1::transformed::<F>(&format!("{name}_median"), version, median),
            pct75: LazyVecFrom1::transformed::<F>(&format!("{name}_pct75"), version, pct75),
            pct90: LazyVecFrom1::transformed::<F>(&format!("{name}_pct90"), version, pct90),
        }
    }
}
