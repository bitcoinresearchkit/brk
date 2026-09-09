use std::iter;

use crate::ByAddrType;
#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

/// `all` aggregate plus a per-address-type breakdown.
#[derive(Clone)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct WithAddrTypes<T, A = T> {
    /// Across all address types.
    pub all: A,
    #[cfg_attr(feature = "storage", traversable(flatten))]
    pub by_addr_type: ByAddrType<T>,
}

impl<T> WithAddrTypes<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        iter::once(&self.all).chain(self.by_addr_type.iter().map(|(_, value)| value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        iter::once(&mut self.all).chain(self.by_addr_type.iter_mut().map(|(_, value)| value))
    }
}
