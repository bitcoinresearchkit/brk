use brk_traversable::Traversable;

use crate::internal::ValueBlockLast;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vaulted_supply: ValueBlockLast,
    pub active_supply: ValueBlockLast,
}
