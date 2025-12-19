use std::ops::{Add, AddAssign, SubAssign};

use brk_types::{CheckedSub, LoadedAddressData, Sats};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Bytes, Formattable};

/// Current supply state tracking UTXO count and total value
#[derive(Debug, Default, Clone, Serialize, JsonSchema)]
pub struct SupplyState {
    /// Number of unspent transaction outputs
    pub utxo_count: u64,
    /// Total value in satoshis
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

impl Formattable for SupplyState {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        true
    }
}

impl Bytes for SupplyState {
    type Array = [u8; size_of::<Self>()];

    fn to_bytes(&self) -> Self::Array {
        let mut arr = [0u8; size_of::<Self>()];
        arr[0..8].copy_from_slice(self.utxo_count.to_bytes().as_ref());
        arr[8..16].copy_from_slice(self.value.to_bytes().as_ref());
        arr
    }

    fn from_bytes(bytes: &[u8]) -> vecdb::Result<Self> {
        Ok(Self {
            utxo_count: u64::from_bytes(&bytes[0..8])?,
            value: Sats::from_bytes(&bytes[8..16])?,
        })
    }
}
