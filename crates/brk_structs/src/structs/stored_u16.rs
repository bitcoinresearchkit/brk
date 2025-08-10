use std::ops::{Add, AddAssign, Div};

use vecdb::{CheckedSub, Printable, StoredCompressed};
use derive_deref::Deref;
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{
    EmptyOutputIndex, OpReturnIndex, P2AAddressIndex, P2MSOutputIndex, P2PK33AddressIndex,
    P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex,
    P2WSHAddressIndex, UnknownOutputIndex,
};

#[derive(
    Debug,
    Deref,
    Clone,
    Default,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
    StoredCompressed,
)]
pub struct StoredU16(u16);

impl StoredU16 {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: u16) -> Self {
        Self(counter)
    }
}

impl From<u16> for StoredU16 {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for StoredU16 {
    fn from(value: usize) -> Self {
        if value > u16::MAX as usize {
            panic!("usize too big (value = {value})")
        }
        Self(value as u16)
    }
}

impl CheckedSub<StoredU16> for StoredU16 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredU16 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u16)
    }
}

impl Add for StoredU16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for StoredU16 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl From<f64> for StoredU16 {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u16::MAX as f64 {
            panic!()
        }
        Self(value as u16)
    }
}

impl From<StoredU16> for f64 {
    fn from(value: StoredU16) -> Self {
        value.0 as f64
    }
}

impl From<StoredU16> for usize {
    fn from(value: StoredU16) -> Self {
        value.0 as usize
    }
}

impl From<P2PK65AddressIndex> for StoredU16 {
    fn from(value: P2PK65AddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PK33AddressIndex> for StoredU16 {
    fn from(value: P2PK33AddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PKHAddressIndex> for StoredU16 {
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<OpReturnIndex> for StoredU16 {
    fn from(value: OpReturnIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2MSOutputIndex> for StoredU16 {
    fn from(value: P2MSOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2SHAddressIndex> for StoredU16 {
    fn from(value: P2SHAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2WSHAddressIndex> for StoredU16 {
    fn from(value: P2WSHAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2WPKHAddressIndex> for StoredU16 {
    fn from(value: P2WPKHAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2TRAddressIndex> for StoredU16 {
    fn from(value: P2TRAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2AAddressIndex> for StoredU16 {
    fn from(value: P2AAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<UnknownOutputIndex> for StoredU16 {
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<EmptyOutputIndex> for StoredU16 {
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl Printable for StoredU16 {
    fn to_string() -> &'static str {
        "u16"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["u16"]
    }
}
