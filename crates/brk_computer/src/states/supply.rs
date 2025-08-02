use std::ops::{Add, AddAssign, SubAssign};

use brk_structs::{CheckedSub, LoadedAddressData, Sats};
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
pub struct SupplyState {
    pub utxos: u64,
    pub value: Sats,
}

impl Add<SupplyState> for SupplyState {
    type Output = Self;
    fn add(self, rhs: SupplyState) -> Self::Output {
        Self {
            utxos: self.utxos + rhs.utxos,
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign<SupplyState> for SupplyState {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl AddAssign<&SupplyState> for SupplyState {
    fn add_assign(&mut self, rhs: &Self) {
        self.utxos += rhs.utxos;
        self.value += rhs.value;
    }
}

impl SubAssign<&SupplyState> for SupplyState {
    fn sub_assign(&mut self, rhs: &Self) {
        self.utxos = self.utxos.checked_sub(rhs.utxos).unwrap();
        self.value = self.value.checked_sub(rhs.value).unwrap();
    }
}

impl From<&LoadedAddressData> for SupplyState {
    fn from(value: &LoadedAddressData) -> Self {
        Self {
            utxos: value.outputs_len as u64,
            value: value.amount(),
        }
    }
}
