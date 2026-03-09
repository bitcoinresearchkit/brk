use derive_more::{Deref, DerefMut};

use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedPerBlockAggregated;

#[derive(Deref, DerefMut, Traversable)]
pub struct Vecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub ComputedPerBlockAggregated<StoredU64, M>,
);
