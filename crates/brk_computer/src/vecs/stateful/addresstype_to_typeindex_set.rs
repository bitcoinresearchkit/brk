use std::{collections::BTreeSet, mem};

use brk_core::TypeIndex;
use derive_deref::{Deref, DerefMut};

use super::ByAddressType;

#[derive(Debug, Deref, DerefMut)]
pub struct AddressTypeToTypeIndexSet(ByAddressType<BTreeSet<TypeIndex>>);

impl AddressTypeToTypeIndexSet {
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

    fn merge_(own: &mut BTreeSet<TypeIndex>, other: &mut BTreeSet<TypeIndex>) {
        if own.len() >= other.len() {
            own.append(other);
        } else {
            other.append(own);
            mem::swap(own, other);
        }
    }
}

impl Default for AddressTypeToTypeIndexSet {
    fn default() -> Self {
        Self(ByAddressType {
            p2pk65: BTreeSet::default(),
            p2pk33: BTreeSet::default(),
            p2pkh: BTreeSet::default(),
            p2sh: BTreeSet::default(),
            p2wpkh: BTreeSet::default(),
            p2wsh: BTreeSet::default(),
            p2tr: BTreeSet::default(),
            p2a: BTreeSet::default(),
        })
    }
}
