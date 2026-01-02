use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec, LazyVecFrom2, VecIndex};

use super::{
    super::ComputedVecValue,
    eager::EagerVecsBuilder,
    lazy::LazyVecsBuilder,
};

const VERSION: Version = Version::ZERO;

/// Lazy binary transform builder.
/// Each group is a `LazyVecFrom2` that transforms from two corresponding stored groups.
#[derive(Clone, Traversable)]
#[allow(clippy::type_complexity)]
pub struct LazyTransform2Builder<I, T, S1T, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub first: Option<Box<LazyVecFrom2<I, T, I, S1T, I, S2T>>>,
    pub average: Option<Box<LazyVecFrom2<I, T, I, S1T, I, S2T>>>,
    pub sum: Option<Box<LazyVecFrom2<I, T, I, S1T, I, S2T>>>,
    pub max: Option<Box<LazyVecFrom2<I, T, I, S1T, I, S2T>>>,
    pub min: Option<Box<LazyVecFrom2<I, T, I, S1T, I, S2T>>>,
    pub last: Option<Box<LazyVecFrom2<I, T, I, S1T, I, S2T>>>,
    pub cumulative: Option<Box<LazyVecFrom2<I, T, I, S1T, I, S2T>>>,
}

impl<I, T, S1T, S2T> LazyTransform2Builder<I, T, S1T, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    /// Create a lazy binary transform from two stored `EagerVecsBuilder`.
    pub fn from_eager<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &EagerVecsBuilder<I, S1T>,
        source2: &EagerVecsBuilder<I, S2T>,
    ) -> Self {
        let v = version + VERSION;
        let suffix = |s: &str| format!("{name}_{s}");
        Self {
            first: source1
                .first
                .as_ref()
                .zip(source2.first.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("first"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            average: source1
                .average
                .as_ref()
                .zip(source2.average.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("avg"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            sum: source1
                .sum
                .as_ref()
                .zip(source2.sum.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("sum"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            max: source1
                .max
                .as_ref()
                .zip(source2.max.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("max"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            min: source1
                .min
                .as_ref()
                .zip(source2.min.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("min"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            last: source1
                .last
                .as_ref()
                .zip(source2.last.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        name,
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            cumulative: source1
                .cumulative
                .as_ref()
                .zip(source2.cumulative.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("cumulative"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
        }
    }

    /// Create a lazy binary transform from two `LazyVecsBuilder`.
    pub fn from_lazy<
        F: BinaryTransform<S1T, S2T, T>,
        S1I: VecIndex,
        S1E: ComputedVecValue,
        S2I: VecIndex,
        S2E: ComputedVecValue,
    >(
        name: &str,
        version: Version,
        source1: &LazyVecsBuilder<I, S1T, S1I, S1E>,
        source2: &LazyVecsBuilder<I, S2T, S2I, S2E>,
    ) -> Self {
        let v = version + VERSION;
        let suffix = |s: &str| format!("{name}_{s}");
        Self {
            first: source1
                .first
                .as_ref()
                .zip(source2.first.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("first"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            average: source1
                .average
                .as_ref()
                .zip(source2.average.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("avg"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            sum: source1
                .sum
                .as_ref()
                .zip(source2.sum.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("sum"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            max: source1
                .max
                .as_ref()
                .zip(source2.max.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("max"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            min: source1
                .min
                .as_ref()
                .zip(source2.min.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("min"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            last: source1
                .last
                .as_ref()
                .zip(source2.last.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        name,
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
            cumulative: source1
                .cumulative
                .as_ref()
                .zip(source2.cumulative.as_ref())
                .map(|(s1, s2)| {
                    Box::new(LazyVecFrom2::transformed::<F>(
                        &suffix("cumulative"),
                        v,
                        s1.boxed_clone(),
                        s2.boxed_clone(),
                    ))
                }),
        }
    }
}
