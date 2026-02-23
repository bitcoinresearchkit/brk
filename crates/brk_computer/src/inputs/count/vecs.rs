use brk_traversable::Traversable;
use brk_types::StoredU64;
use derive_more::{Deref, DerefMut};
use vecdb::{Rw, StorageMode};

use crate::internal::TxDerivedFull;

#[derive(Deref, DerefMut, Traversable)]
pub struct Vecs<M: StorageMode = Rw>(pub TxDerivedFull<StoredU64, M>);
