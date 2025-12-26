use brk_types::Height;
use derive_deref::{Deref, DerefMut};
use rustc_hash::FxHashMap;

use super::type_vec::AddressTypeToVec;

/// Hashmap from Height to AddressTypeToVec.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct HeightToAddressTypeToVec<T>(FxHashMap<Height, AddressTypeToVec<T>>);

impl<T> HeightToAddressTypeToVec<T> {
    /// Create with pre-allocated capacity for unique heights.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(FxHashMap::with_capacity_and_hasher(
            capacity,
            Default::default(),
        ))
    }
}

impl<T> HeightToAddressTypeToVec<T> {
    /// Consume and iterate over (Height, AddressTypeToVec) pairs.
    pub fn into_iter(self) -> impl Iterator<Item = (Height, AddressTypeToVec<T>)> {
        self.0.into_iter()
    }
}
