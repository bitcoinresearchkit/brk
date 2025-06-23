use std::ops::{Add, Div};

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable};

use super::{
    DateIndex, EmptyOutputIndex, Height, InputIndex, MonthIndex, OpReturnIndex, OutputIndex,
    P2AAddressIndex, P2MSOutputIndex, P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex,
    P2SHAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex, P2WSHAddressIndex, TxIndex,
    UnknownOutputIndex, YearIndex,
};

#[derive(
    Debug,
    Deref,
    DerefMut,
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
)]
pub struct StoredUsize(usize);

impl StoredUsize {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: usize) -> Self {
        Self(counter)
    }
}

impl From<usize> for StoredUsize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl CheckedSub<StoredUsize> for StoredUsize {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredUsize {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Add for StoredUsize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<f64> for StoredUsize {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as usize)
    }
}

impl From<StoredUsize> for f64 {
    fn from(value: StoredUsize) -> Self {
        value.0 as f64
    }
}

impl From<Height> for StoredUsize {
    fn from(value: Height) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<DateIndex> for StoredUsize {
    fn from(value: DateIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<MonthIndex> for StoredUsize {
    fn from(value: MonthIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<YearIndex> for StoredUsize {
    fn from(value: YearIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<OutputIndex> for StoredUsize {
    fn from(value: OutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<InputIndex> for StoredUsize {
    fn from(value: InputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<TxIndex> for StoredUsize {
    fn from(value: TxIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PK65AddressIndex> for StoredUsize {
    fn from(value: P2PK65AddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PK33AddressIndex> for StoredUsize {
    fn from(value: P2PK33AddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PKHAddressIndex> for StoredUsize {
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<OpReturnIndex> for StoredUsize {
    fn from(value: OpReturnIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2MSOutputIndex> for StoredUsize {
    fn from(value: P2MSOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2SHAddressIndex> for StoredUsize {
    fn from(value: P2SHAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2WSHAddressIndex> for StoredUsize {
    fn from(value: P2WSHAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2WPKHAddressIndex> for StoredUsize {
    fn from(value: P2WPKHAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2TRAddressIndex> for StoredUsize {
    fn from(value: P2TRAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2AAddressIndex> for StoredUsize {
    fn from(value: P2AAddressIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<UnknownOutputIndex> for StoredUsize {
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<EmptyOutputIndex> for StoredUsize {
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl Printable for StoredUsize {
    fn to_string() -> &'static str {
        "usize"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["usize"]
    }
}
