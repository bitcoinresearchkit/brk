use brk_traversable::Traversable;
use brk_types::StoredF32;
use derive_more::{Deref, DerefMut};

use crate::internal::ComputedVecsDateAverage;

/// Inflation rate metrics
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct Vecs(pub ComputedVecsDateAverage<StoredF32>);
