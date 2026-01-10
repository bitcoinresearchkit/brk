use std::{collections::hash_map::Entry, mem};

use brk_cohort::ByAddressType;
use brk_types::{OutputType, TypeIndex};
use derive_more::{Deref, DerefMut};
use rustc_hash::FxHashMap;
use smallvec::{Array, SmallVec};

/// A hashmap for each address type, keyed by TypeIndex.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct AddressTypeToTypeIndexMap<T>(ByAddressType<FxHashMap<TypeIndex, T>>);

impl<T> Default for AddressTypeToTypeIndexMap<T> {
    fn default() -> Self {
        Self(ByAddressType {
            p2a: FxHashMap::default(),
            p2pk33: FxHashMap::default(),
            p2pk65: FxHashMap::default(),
            p2pkh: FxHashMap::default(),
            p2sh: FxHashMap::default(),
            p2tr: FxHashMap::default(),
            p2wpkh: FxHashMap::default(),
            p2wsh: FxHashMap::default(),
        })
    }
}

impl<T> AddressTypeToTypeIndexMap<T> {
    /// Create with pre-allocated capacity per address type.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(ByAddressType {
            p2a: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            p2pk33: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            p2pk65: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            p2pkh: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            p2sh: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            p2tr: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            p2wpkh: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
            p2wsh: FxHashMap::with_capacity_and_hasher(capacity, Default::default()),
        })
    }

    /// Merge two maps, consuming other and extending self.
    pub fn merge(mut self, mut other: Self) -> Self {
        Self::merge_single(&mut self.p2a, &mut other.p2a);
        Self::merge_single(&mut self.p2pk33, &mut other.p2pk33);
        Self::merge_single(&mut self.p2pk65, &mut other.p2pk65);
        Self::merge_single(&mut self.p2pkh, &mut other.p2pkh);
        Self::merge_single(&mut self.p2sh, &mut other.p2sh);
        Self::merge_single(&mut self.p2tr, &mut other.p2tr);
        Self::merge_single(&mut self.p2wpkh, &mut other.p2wpkh);
        Self::merge_single(&mut self.p2wsh, &mut other.p2wsh);
        self
    }

    fn merge_single(own: &mut FxHashMap<TypeIndex, T>, other: &mut FxHashMap<TypeIndex, T>) {
        if own.len() < other.len() {
            mem::swap(own, other);
        }
        own.extend(other.drain());
    }

    /// Merge another map into self, consuming other.
    pub fn merge_mut(&mut self, mut other: Self) {
        Self::merge_single(&mut self.p2a, &mut other.p2a);
        Self::merge_single(&mut self.p2pk33, &mut other.p2pk33);
        Self::merge_single(&mut self.p2pk65, &mut other.p2pk65);
        Self::merge_single(&mut self.p2pkh, &mut other.p2pkh);
        Self::merge_single(&mut self.p2sh, &mut other.p2sh);
        Self::merge_single(&mut self.p2tr, &mut other.p2tr);
        Self::merge_single(&mut self.p2wpkh, &mut other.p2wpkh);
        Self::merge_single(&mut self.p2wsh, &mut other.p2wsh);
    }

    /// Insert a value for a specific address type and typeindex.
    pub fn insert_for_type(&mut self, address_type: OutputType, typeindex: TypeIndex, value: T) {
        self.get_mut(address_type).unwrap().insert(typeindex, value);
    }

    /// Iterate over sorted entries by address type.
    pub fn into_sorted_iter(self) -> impl Iterator<Item = (OutputType, Vec<(TypeIndex, T)>)> {
        self.0.into_iter().map(|(output_type, map)| {
            let mut sorted: Vec<_> = map.into_iter().collect();
            sorted.sort_unstable_by_key(|(typeindex, _)| *typeindex);
            (output_type, sorted)
        })
    }

    /// Consume and iterate over entries by address type.
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> impl Iterator<Item = (OutputType, FxHashMap<TypeIndex, T>)> {
        self.0.into_iter()
    }

    /// Consume and return the inner ByAddressType.
    pub fn into_inner(self) -> ByAddressType<FxHashMap<TypeIndex, T>> {
        self.0
    }

    /// Iterate mutably over entries by address type.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (OutputType, &mut FxHashMap<TypeIndex, T>)> {
        self.0.iter_mut()
    }
}

impl<T> AddressTypeToTypeIndexMap<SmallVec<T>>
where
    T: Array,
{
    /// Merge two maps of SmallVec values, concatenating vectors.
    pub fn merge_vec(mut self, other: Self) -> Self {
        for (address_type, other_map) in other.0.into_iter() {
            let self_map = self.0.get_mut_unwrap(address_type);
            for (typeindex, mut other_vec) in other_map {
                match self_map.entry(typeindex) {
                    Entry::Occupied(mut entry) => {
                        let self_vec = entry.get_mut();
                        if other_vec.len() > self_vec.len() {
                            mem::swap(self_vec, &mut other_vec);
                        }
                        self_vec.extend(other_vec);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(other_vec);
                    }
                }
            }
        }
        self
    }
}
