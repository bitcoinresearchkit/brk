//! Height to AddressTypeToVec hashmap.

use brk_types::Height;
use derive_deref::{Deref, DerefMut};
use rustc_hash::FxHashMap;

use super::type_vec::AddressTypeToVec;

/// Hashmap from Height to AddressTypeToVec.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct HeightToAddressTypeToVec<T>(FxHashMap<Height, AddressTypeToVec<T>>);

impl<T> HeightToAddressTypeToVec<T> {
    /// Merge another map into this one.
    pub fn merge_mut(&mut self, other: Self) {
        for (height, vec) in other.0 {
            self.entry(height).or_default().merge_mut(vec);
        }
    }

    /// Consume and iterate over (Height, AddressTypeToVec) pairs.
    pub fn into_iter(self) -> impl Iterator<Item = (Height, AddressTypeToVec<T>)> {
        self.0.into_iter()
    }
}
