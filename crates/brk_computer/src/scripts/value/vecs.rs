use brk_traversable::Traversable;

use crate::internal::ValueBlockFull;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub opreturn: ValueBlockFull,
}
