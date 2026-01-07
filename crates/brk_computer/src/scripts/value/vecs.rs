use brk_traversable::Traversable;

use crate::internal::ValueBlockFull;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_opreturn_value: ValueBlockFull,
}
