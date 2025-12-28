use brk_traversable::Traversable;
use brk_types::Version;
use schemars::JsonSchema;
use vecdb::{IterableCloneableVec, LazyVecFrom1, UnaryTransform, VecIndex};

use super::{ComputedVecValue, EagerVecsBuilder, LazyVecsBuilder};

const VERSION: Version = Version::ZERO;

/// Lazy transform version of `EagerVecsBuilder`.
/// Each group is a `LazyVecFrom1` that transforms from the corresponding stored group.
/// S1T is the source type, T is the output type (can be the same for transforms like negation).
#[derive(Clone, Traversable)]
pub struct LazyTransformBuilder<I, T, S1T = T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
{
    pub first: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub average: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub sum: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub max: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub pct90: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub pct75: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub median: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub pct25: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub pct10: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub min: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub last: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
    pub cumulative: Option<Box<LazyVecFrom1<I, T, I, S1T>>>,
}

impl<I, T, S1T> LazyTransformBuilder<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    /// Create a lazy transform from a stored `EagerVecsBuilder`.
    /// F is the transform type (e.g., `Negate`, `Halve`).
    pub fn from_eager<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &EagerVecsBuilder<I, S1T>,
    ) -> Self {
        let v = version + VERSION;
        let suffix = |s: &str| format!("{name}_{s}");
        Self {
            first: source.first.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("first"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            average: source.average.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("avg"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            sum: source.sum.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("sum"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            max: source.max.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("max"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            pct90: source.pct90.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("pct90"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            pct75: source.pct75.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("pct75"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            median: source.median.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("median"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            pct25: source.pct25.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("pct25"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            pct10: source.pct10.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("pct10"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            min: source.min.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("min"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            last: source
                .last
                .as_ref()
                .map(|s| Box::new(LazyVecFrom1::transformed::<F>(name, v, s.boxed_clone()))),
            cumulative: source.cumulative.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("cumulative"),
                    v,
                    s.boxed_clone(),
                ))
            }),
        }
    }
}

impl<I, T, S1T> LazyTransformBuilder<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
{
    pub fn unwrap_sum(&self) -> &LazyVecFrom1<I, T, I, S1T> {
        self.sum.as_ref().unwrap()
    }

    pub fn unwrap_cumulative(&self) -> &LazyVecFrom1<I, T, I, S1T> {
        self.cumulative.as_ref().unwrap()
    }
}

impl<I, T, S1T> LazyTransformBuilder<I, T, S1T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    /// Create a lazy transform from a `LazyVecsBuilder`.
    /// Note: LazyVecsBuilder doesn't have percentiles, so those will be None.
    pub fn from_lazy<F: UnaryTransform<S1T, T>, S1I: VecIndex, S2T: ComputedVecValue>(
        name: &str,
        version: Version,
        source: &LazyVecsBuilder<I, S1T, S1I, S2T>,
    ) -> Self {
        let v = version + VERSION;
        // Use same suffix pattern as EagerVecsBuilder
        let suffix = |s: &str| format!("{name}_{s}");
        Self {
            first: source.first.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("first"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            average: source.average.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("avg"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            sum: source.sum.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("sum"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            max: source.max.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("max"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            pct90: None,
            pct75: None,
            median: None,
            pct25: None,
            pct10: None,
            min: source.min.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("min"),
                    v,
                    s.boxed_clone(),
                ))
            }),
            last: source
                .last
                .as_ref()
                .map(|s| Box::new(LazyVecFrom1::transformed::<F>(name, v, s.boxed_clone()))),
            cumulative: source.cumulative.as_ref().map(|s| {
                Box::new(LazyVecFrom1::transformed::<F>(
                    &suffix("cumulative"),
                    v,
                    s.boxed_clone(),
                ))
            }),
        }
    }
}
