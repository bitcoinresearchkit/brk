use derive_deref::{Deref, DerefMut};

use super::ByAddressType;

#[derive(Debug, Deref, DerefMut)]
pub struct AddressTypeToVec<T>(ByAddressType<Vec<T>>);

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
