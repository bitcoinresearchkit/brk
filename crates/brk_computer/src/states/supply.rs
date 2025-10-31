use std::ops::{Add, AddAssign, SubAssign};

use brk_types::{CheckedSub, LoadedAddressData, Sats};
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
pub struct SupplyState {
    pub utxo_count: u64,
    pub value: Sats,
}

impl Add<SupplyState> for SupplyState {
    type Output = Self;
    fn add(self, rhs: SupplyState) -> Self::Output {
        Self {
            utxo_count: self.utxo_count + rhs.utxo_count,
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
        self.utxo_count += rhs.utxo_count;
        self.value += rhs.value;
    }
}

impl SubAssign<&SupplyState> for SupplyState {
    fn sub_assign(&mut self, rhs: &Self) {
        self.utxo_count = self.utxo_count.checked_sub(rhs.utxo_count).unwrap();
        self.value = self.value.checked_sub(rhs.value).unwrap();
    }
}

impl From<&LoadedAddressData> for SupplyState {
    #[inline]
    fn from(value: &LoadedAddressData) -> Self {
        Self {
            utxo_count: value.utxo_count() as u64,
            value: value.balance(),
        }
    }
}

impl std::fmt::Display for SupplyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "utxos: {}, value: {}", self.utxo_count, self.value)
    }
}
