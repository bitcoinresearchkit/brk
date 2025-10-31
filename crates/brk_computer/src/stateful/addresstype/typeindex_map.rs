use std::mem;

use brk_grouper::ByAddressType;
use brk_types::{OutputType, TypeIndex};
use derive_deref::{Deref, DerefMut};
use rustc_hash::FxHashMap;

#[derive(Debug, Deref, DerefMut)]
pub struct AddressTypeToTypeIndexMap<T>(ByAddressType<FxHashMap<TypeIndex, T>>);

impl<T> AddressTypeToTypeIndexMap<T> {
    pub fn merge(mut self, mut other: Self) -> Self {
        Self::merge_(&mut self.p2pk65, &mut other.p2pk65);
        Self::merge_(&mut self.p2pk33, &mut other.p2pk33);
        Self::merge_(&mut self.p2pkh, &mut other.p2pkh);
        Self::merge_(&mut self.p2sh, &mut other.p2sh);
        Self::merge_(&mut self.p2wpkh, &mut other.p2wpkh);
        Self::merge_(&mut self.p2wsh, &mut other.p2wsh);
        Self::merge_(&mut self.p2tr, &mut other.p2tr);
        Self::merge_(&mut self.p2a, &mut other.p2a);
        self
    }

    fn merge_(own: &mut FxHashMap<TypeIndex, T>, other: &mut FxHashMap<TypeIndex, T>) {
        if own.len() < other.len() {
            mem::swap(own, other);
        }
        own.extend(other.drain());
    }

    // pub fn get_for_type(&self, address_type: OutputType, typeindex: &TypeIndex) -> Option<&T> {
    //     self.get(address_type).unwrap().get(typeindex)
    // }

    pub fn insert_for_type(&mut self, address_type: OutputType, typeindex: TypeIndex, value: T) {
        self.get_mut(address_type).unwrap().insert(typeindex, value);
    }

    pub fn remove_for_type(&mut self, address_type: OutputType, typeindex: &TypeIndex) -> T {
        self.get_mut(address_type)
            .unwrap()
            .remove(typeindex)
            .unwrap()
    }

    pub fn into_sorted_iter(self) -> impl Iterator<Item = (OutputType, Vec<(TypeIndex, T)>)> {
        self.0.into_iter_typed().map(|(output_type, map)| {
            let mut sorted: Vec<_> = map.into_iter().collect();
            sorted.sort_unstable_by_key(|(typeindex, _)| *typeindex);
            (output_type, sorted)
        })
    }
}

impl<T> Default for AddressTypeToTypeIndexMap<T> {
    fn default() -> Self {
        Self(ByAddressType {
            p2pk65: FxHashMap::default(),
            p2pk33: FxHashMap::default(),
            p2pkh: FxHashMap::default(),
            p2sh: FxHashMap::default(),
            p2wpkh: FxHashMap::default(),
            p2wsh: FxHashMap::default(),
            p2tr: FxHashMap::default(),
            p2a: FxHashMap::default(),
        })
    }
}

impl<T> AddressTypeToTypeIndexMap<Vec<T>>
where
    T: Copy,
{
    pub fn merge_vec(mut self, other: Self) -> Self {
        for (address_type, other_map) in other.0.into_iter_typed() {
            let self_map = self.0.get_mut_unwrap(address_type);
            for (typeindex, mut other_vec) in other_map {
                self_map
                    .entry(typeindex)
                    .and_modify(|self_vec| {
                        if other_vec.len() > self_vec.len() {
                            mem::swap(self_vec, &mut other_vec);
                        }
                        self_vec.extend(other_vec.iter().copied());
                    })
                    .or_insert(other_vec);
            }
        }
        self
    }
}
