use brk_traversable::Traversable;
use brk_types::Dollars;
use derive_more::{Deref, DerefMut};

use crate::internal::LazyBlockLast;

#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct Vecs(pub LazyBlockLast<Dollars>);
