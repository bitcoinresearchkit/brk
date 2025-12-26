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
    /// Unwrap the inner ByAddressType.
    pub fn unwrap(self) -> ByAddressType<Vec<T>> {
        self.0
    }
}
