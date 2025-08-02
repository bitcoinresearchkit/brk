use std::collections::BTreeSet;

use brk_structs::TypeIndex;
use derive_deref::{Deref, DerefMut};

use super::ByAddressType;

#[derive(Debug, Deref, DerefMut)]
pub struct AddressTypeToTypeIndexSet(ByAddressType<BTreeSet<TypeIndex>>);

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
