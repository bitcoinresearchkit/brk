//! Lazy unary transform for Percentiles.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::{ComputedVecValue, Percentiles};

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
    pub fn from_percentiles<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &Percentiles<I, S1T>,
    ) -> Self {
        Self {
            pct10: LazyVecFrom1::transformed::<F>(
                &format!("{name}_pct10"),
                version,
                source.pct10.0.boxed_clone(),
            ),
            pct25: LazyVecFrom1::transformed::<F>(
                &format!("{name}_pct25"),
                version,
                source.pct25.0.boxed_clone(),
            ),
            median: LazyVecFrom1::transformed::<F>(
                &format!("{name}_median"),
                version,
                source.median.0.boxed_clone(),
            ),
            pct75: LazyVecFrom1::transformed::<F>(
                &format!("{name}_pct75"),
                version,
                source.pct75.0.boxed_clone(),
            ),
            pct90: LazyVecFrom1::transformed::<F>(
                &format!("{name}_pct90"),
                version,
                source.pct90.0.boxed_clone(),
            ),
        }
    }
}
