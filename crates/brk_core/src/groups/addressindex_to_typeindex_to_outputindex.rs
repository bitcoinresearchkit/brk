use std::mem;

use derive_deref::{Deref, DerefMut};

use crate::{OutputIndex, TypeIndex};

use super::GroupedByAddressType;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddressIndexToTypeIndedToOutputIndex(
    GroupedByAddressType<Vec<(TypeIndex, OutputIndex)>>,
);

impl AddressIndexToTypeIndedToOutputIndex {
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

    fn merge_(own: &mut Vec<(TypeIndex, OutputIndex)>, other: &mut Vec<(TypeIndex, OutputIndex)>) {
        if own.len() >= other.len() {
            own.append(other);
        } else {
            other.append(own);
            mem::swap(own, other);
        }
    }

    pub fn inner(self) -> GroupedByAddressType<Vec<(TypeIndex, OutputIndex)>> {
        self.0
    }
}
