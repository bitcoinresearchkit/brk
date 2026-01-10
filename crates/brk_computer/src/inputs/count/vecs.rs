use brk_traversable::Traversable;
use brk_types::StoredU64;
use derive_more::{Deref, DerefMut};

use crate::internal::TxDerivedFull;

#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct Vecs(pub TxDerivedFull<StoredU64>);
