use brk_traversable::Traversable;

use crate::internal::ValueFromHeightLast;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vaulted_supply: ValueFromHeightLast,
    pub active_supply: ValueFromHeightLast,
}
