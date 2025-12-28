use std::{
    fmt,
    ops::{Add, AddAssign, SubAssign},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Bytes, Formattable};

use crate::{CheckedSub, LoadedAddressData, Sats};

/// Current supply state tracking UTXO count and total value
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
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
        self.utxo_count = self
            .utxo_count
            .checked_sub(rhs.utxo_count)
            .unwrap_or_else(|| {
                panic!(
                    "SupplyState underflow: cohort utxo_count {} < address utxo_count {}. \
                This indicates a desync between cohort state and address data. \
                Try deleting the compute cache and restarting fresh.",
                    self.utxo_count, rhs.utxo_count
                )
            });
        self.value = self.value.checked_sub(rhs.value).unwrap_or_else(|| {
            panic!(
                "SupplyState underflow: cohort value {} < address value {}. \
                This indicates a desync between cohort state and address data. \
                Try deleting the compute cache and restarting fresh.",
                self.value, rhs.value
            )
        });
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

impl fmt::Display for SupplyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
