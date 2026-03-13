use brk_traversable::Traversable;
use brk_types::Height;
use schemars::JsonSchema;
use vecdb::{DeltaAvg, LazyDeltaVec};

use crate::internal::{NumericValue, Resolutions};

/// A single lazy rolling-average slot from height: the lazy delta vec + its resolution views.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyRollingAvgFromHeight<T>
where
    T: NumericValue + JsonSchema,
{
    pub height: LazyDeltaVec<Height, f64, T, DeltaAvg>,
    #[traversable(flatten)]
    pub resolutions: Box<Resolutions<T>>,
}
