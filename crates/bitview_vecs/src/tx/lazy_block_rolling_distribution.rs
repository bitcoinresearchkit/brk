use bitview_traversable::Traversable;
use brk_types::Height;
use schemars::JsonSchema;

use crate::LazyDistribution;
use bitview_compute::ComputedVecValue;

#[derive(Clone, Traversable)]
pub struct LazyBlockRollingDistribution<T, S1T>
where
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
{
    /// Uses the six-block window ending at the represented block.
    pub _6b: LazyDistribution<Height, T, S1T>,
}
