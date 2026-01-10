use brk_traversable::Traversable;

use crate::internal::ValueFromHeightFull;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub opreturn: ValueFromHeightFull,
}
