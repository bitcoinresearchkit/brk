use derive_more::{Deref, DerefMut};

use brk_traversable::Traversable;
use brk_types::Timestamp;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlockRollingAverage;

#[derive(Deref, DerefMut, Traversable)]
pub struct Vecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub PerBlockRollingAverage<Timestamp, Timestamp, M>,
);
