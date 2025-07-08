use std::{collections::BTreeMap, mem};

use brk_core::TypeIndex;
use derive_deref::{Deref, DerefMut};

use super::ByAddressType;

#[derive(Debug, Deref, DerefMut)]
pub struct AddressTypeToTypeIndexTree<T>(ByAddressType<BTreeMap<TypeIndex, T>>);

impl<T> AddressTypeToTypeIndexTree<T> {
    pub fn merge(&mut self, mut other: Self) {
        Self::merge_(&mut self.p2pk65, &mut other.p2pk65);
        Self::merge_(&mut self.p2pk33, &mut other.p2pk33);
        Self::merge_(&mut self.p2pkh, &mut other.p2pkh);
        Self::merge_(&mut self.p2sh, &mut other.p2sh);
        Self::merge_(&mut self.p2wpkh, &mut other.p2wpkh);
        Self::merge_(&mut self.p2wsh, &mut other.p2wsh);
        Self::merge_(&mut self.p2tr, &mut other.p2tr);
        Self::merge_(&mut self.p2a, &mut other.p2a);
    }

    fn merge_(own: &mut BTreeMap<TypeIndex, T>, other: &mut BTreeMap<TypeIndex, T>) {
        if own.len() >= other.len() {
            own.append(other);
        } else {
            other.append(own);
            mem::swap(own, other);
        }
    }

    pub fn unwrap(self) -> ByAddressType<BTreeMap<TypeIndex, T>> {
        self.0
    }
}

impl<T> Default for AddressTypeToTypeIndexTree<T> {
    fn default() -> Self {
        Self(ByAddressType {
            p2pk65: BTreeMap::default(),
            p2pk33: BTreeMap::default(),
            p2pkh: BTreeMap::default(),
            p2sh: BTreeMap::default(),
            p2wpkh: BTreeMap::default(),
            p2wsh: BTreeMap::default(),
            p2tr: BTreeMap::default(),
            p2a: BTreeMap::default(),
        })
    }
}
