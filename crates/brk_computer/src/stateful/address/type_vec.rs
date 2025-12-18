//! Per-address-type vector.

use std::mem;

use brk_grouper::ByAddressType;
use derive_deref::{Deref, DerefMut};

/// A vector for each address type.
#[derive(Debug, Deref, DerefMut)]
pub struct AddressTypeToVec<T>(ByAddressType<Vec<T>>);

impl<T> Default for AddressTypeToVec<T> {
    fn default() -> Self {
        Self(ByAddressType {
            p2a: vec![],
            p2pk33: vec![],
            p2pk65: vec![],
            p2pkh: vec![],
            p2sh: vec![],
            p2tr: vec![],
            p2wpkh: vec![],
            p2wsh: vec![],
        })
    }
}

impl<T> AddressTypeToVec<T> {
    /// Create with pre-allocated capacity per address type.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(ByAddressType {
            p2a: Vec::with_capacity(capacity),
            p2pk33: Vec::with_capacity(capacity),
            p2pk65: Vec::with_capacity(capacity),
            p2pkh: Vec::with_capacity(capacity),
            p2sh: Vec::with_capacity(capacity),
            p2tr: Vec::with_capacity(capacity),
            p2wpkh: Vec::with_capacity(capacity),
            p2wsh: Vec::with_capacity(capacity),
        })
    }
}

impl<T> AddressTypeToVec<T> {
    /// Merge two AddressTypeToVec, consuming other.
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

    /// Merge in place.
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

    fn merge_single(own: &mut Vec<T>, other: &mut Vec<T>) {
        if own.len() >= other.len() {
            own.append(other);
        } else {
            other.append(own);
            mem::swap(own, other);
        }
    }

    /// Unwrap the inner ByAddressType.
    pub fn unwrap(self) -> ByAddressType<Vec<T>> {
        self.0
    }
}
