//! Lazy binary transform for Percentiles.

use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{BinaryTransform, ReadableBoxedVec, LazyVecFrom2, VecIndex};

use crate::internal::{ComputedVecValue, Percentiles};

#[derive(Clone, Traversable)]
pub struct LazyBinaryPercentiles<I, T, S1T = T, S2T = T>
where
    I: VecIndex,
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub pct10: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    pub pct25: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    pub median: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    pub pct75: LazyVecFrom2<I, T, I, S1T, I, S2T>,
    pub pct90: LazyVecFrom2<I, T, I, S1T, I, S2T>,
}

impl<I, T, S1T, S2T> LazyBinaryPercentiles<I, T, S1T, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_percentiles<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source: &Percentiles<I, S1T>,
        source2: ReadableBoxedVec<I, S2T>,
    ) -> Self {
        Self {
            pct10: LazyVecFrom2::transformed::<F>(
                &format!("{name}_pct10"),
                version,
                source.boxed_pct10(),
                source2.clone(),
            ),
            pct25: LazyVecFrom2::transformed::<F>(
                &format!("{name}_pct25"),
                version,
                source.boxed_pct25(),
                source2.clone(),
            ),
            median: LazyVecFrom2::transformed::<F>(
                &format!("{name}_median"),
                version,
                source.boxed_median(),
                source2.clone(),
            ),
            pct75: LazyVecFrom2::transformed::<F>(
                &format!("{name}_pct75"),
                version,
                source.boxed_pct75(),
                source2.clone(),
            ),
            pct90: LazyVecFrom2::transformed::<F>(
                &format!("{name}_pct90"),
                version,
                source.boxed_pct90(),
                source2,
            ),
        }
    }
}
