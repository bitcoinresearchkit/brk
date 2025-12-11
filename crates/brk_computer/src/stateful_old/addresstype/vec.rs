use std::mem;

use brk_grouper::ByAddressType;
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Deref, DerefMut)]
pub struct AddressTypeToVec<T>(ByAddressType<Vec<T>>);

impl<T> AddressTypeToVec<T> {
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

    pub fn merge_mut(&mut self, mut other: Self) {
        Self::merge_(&mut self.p2pk65, &mut other.p2pk65);
        Self::merge_(&mut self.p2pk33, &mut other.p2pk33);
        Self::merge_(&mut self.p2pkh, &mut other.p2pkh);
        Self::merge_(&mut self.p2sh, &mut other.p2sh);
        Self::merge_(&mut self.p2wpkh, &mut other.p2wpkh);
        Self::merge_(&mut self.p2wsh, &mut other.p2wsh);
        Self::merge_(&mut self.p2tr, &mut other.p2tr);
        Self::merge_(&mut self.p2a, &mut other.p2a);
    }

    fn merge_(own: &mut Vec<T>, other: &mut Vec<T>) {
        if own.len() >= other.len() {
            own.append(other);
        } else {
            other.append(own);
            mem::swap(own, other);
        }
    }

    pub fn unwrap(self) -> ByAddressType<Vec<T>> {
        self.0
    }
}

impl<T> Default for AddressTypeToVec<T> {
    fn default() -> Self {
        Self(ByAddressType {
            p2pk65: vec![],
            p2pk33: vec![],
            p2pkh: vec![],
            p2sh: vec![],
            p2wpkh: vec![],
            p2wsh: vec![],
            p2tr: vec![],
            p2a: vec![],
        })
    }
}
