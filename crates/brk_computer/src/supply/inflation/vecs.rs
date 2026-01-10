use brk_traversable::Traversable;
use brk_types::StoredF32;
use derive_more::{Deref, DerefMut};

use crate::internal::ComputedDateAverage;

/// Inflation rate metrics
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct Vecs(pub ComputedDateAverage<StoredF32>);
