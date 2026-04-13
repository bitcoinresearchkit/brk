use brk_traversable::Traversable;
use brk_types::StoredF32;
use vecdb::{Rw, StorageMode};

use crate::internal::{PerBlock, Windows};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw>(#[traversable(flatten)] pub Windows<PerBlock<StoredF32, M>>);
