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
