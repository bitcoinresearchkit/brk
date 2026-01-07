use brk_traversable::Traversable;

use crate::internal::ValueBlockLast;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_vaulted_supply: ValueBlockLast,
    pub indexes_to_active_supply: ValueBlockLast,
}
