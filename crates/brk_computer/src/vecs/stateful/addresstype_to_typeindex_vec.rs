use std::mem;

use brk_core::{AddressData, Dollars, OutputIndex, Sats, TypeIndex};
use derive_deref::{Deref, DerefMut};

use crate::vecs::stateful::WithAddressDataSource;

use super::GroupedByAddressType;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddressTypeToTypeIndexVec<T>(GroupedByAddressType<Vec<(TypeIndex, T)>>);

impl<T> AddressTypeToTypeIndexVec<T> {
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

    fn merge_(own: &mut Vec<(TypeIndex, T)>, other: &mut Vec<(TypeIndex, T)>) {
        if own.len() >= other.len() {
            own.append(other);
        } else {
            other.append(own);
            mem::swap(own, other);
        }
    }

    pub fn unwrap(self) -> GroupedByAddressType<Vec<(TypeIndex, T)>> {
        self.0
    }
}

impl AddressTypeToTypeIndexVec<OutputIndex> {
    #[allow(clippy::type_complexity)]
    pub fn extend_from_sent(
        &mut self,
        other: &AddressTypeToTypeIndexVec<(
            OutputIndex,
            Sats,
            Option<WithAddressDataSource<AddressData>>,
            Option<Dollars>,
            usize,
            f64,
            bool,
        )>,
    ) {
        Self::extend_from_sent_(&mut self.p2pk33, &other.p2pk33);
        Self::extend_from_sent_(&mut self.p2pkh, &other.p2pkh);
        Self::extend_from_sent_(&mut self.p2sh, &other.p2sh);
        Self::extend_from_sent_(&mut self.p2wpkh, &other.p2wpkh);
        Self::extend_from_sent_(&mut self.p2wsh, &other.p2wsh);
        Self::extend_from_sent_(&mut self.p2tr, &other.p2tr);
        Self::extend_from_sent_(&mut self.p2a, &other.p2a);
    }

    #[allow(clippy::type_complexity)]
    fn extend_from_sent_(
        own: &mut Vec<(TypeIndex, OutputIndex)>,
        other: &[(
            TypeIndex,
            (
                OutputIndex,
                Sats,
                Option<WithAddressDataSource<AddressData>>,
                Option<Dollars>,
                usize,
                f64,
                bool,
            ),
        )],
    ) {
        own.extend(
            other
                .iter()
                .map(|(type_index, (output_index, ..))| (*type_index, *output_index)),
        );
    }

    pub fn extend_from_received(
        &mut self,
        other: &AddressTypeToTypeIndexVec<(
            OutputIndex,
            Sats,
            Option<WithAddressDataSource<AddressData>>,
        )>,
    ) {
        Self::extend_from_received_(&mut self.p2pk33, &other.p2pk33);
        Self::extend_from_received_(&mut self.p2pkh, &other.p2pkh);
        Self::extend_from_received_(&mut self.p2sh, &other.p2sh);
        Self::extend_from_received_(&mut self.p2wpkh, &other.p2wpkh);
        Self::extend_from_received_(&mut self.p2wsh, &other.p2wsh);
        Self::extend_from_received_(&mut self.p2tr, &other.p2tr);
        Self::extend_from_received_(&mut self.p2a, &other.p2a);
    }

    #[allow(clippy::type_complexity)]
    fn extend_from_received_(
        own: &mut Vec<(TypeIndex, OutputIndex)>,
        other: &[(
            TypeIndex,
            (
                OutputIndex,
                Sats,
                Option<WithAddressDataSource<AddressData>>,
            ),
        )],
    ) {
        own.extend(
            other
                .iter()
                .map(|(type_index, (output_index, ..))| (*type_index, *output_index)),
        );
    }
}
