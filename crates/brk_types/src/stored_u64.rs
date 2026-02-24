use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use super::{
    Day1, EmptyOutputIndex, Height, Month1, OpReturnIndex, P2AAddressIndex,
    P2MSOutputIndex, P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex,
    P2TRAddressIndex, P2WPKHAddressIndex, P2WSHAddressIndex, TxInIndex, TxIndex, TxOutIndex,
    UnknownOutputIndex, Year1,
};

/// Fixed-size 64-bit unsigned integer optimized for on-disk storage
#[derive(
    Debug,
    Default,
    Deref,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct StoredU64(u64);

impl StoredU64 {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: u64) -> Self {
        Self(counter)
    }
}

impl From<u64> for StoredU64 {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<StoredU64> for u64 {
    #[inline]
    fn from(value: StoredU64) -> Self {
        value.0
    }
}

impl From<StoredU64> for usize {
    #[inline]
    fn from(value: StoredU64) -> Self {
        value.0 as usize
    }
}

impl From<usize> for StoredU64 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl CheckedSub<StoredU64> for StoredU64 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredU64 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}

impl Add for StoredU64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for StoredU64 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for StoredU64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for StoredU64 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl From<f64> for StoredU64 {
    #[inline]
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as u64)
    }
}

impl From<StoredU64> for f64 {
    #[inline]
    fn from(value: StoredU64) -> Self {
        value.0 as f64
    }
}

impl From<TxIndex> for StoredU64 {
    #[inline]
    fn from(value: TxIndex) -> Self {
        Self(*value as u64)
    }
}

impl From<TxInIndex> for StoredU64 {
    #[inline]
    fn from(value: TxInIndex) -> Self {
        Self(*value)
    }
}

impl From<Height> for StoredU64 {
    #[inline]
    fn from(value: Height) -> Self {
        Self(*value as u64)
    }
}

impl From<TxOutIndex> for StoredU64 {
    #[inline]
    fn from(value: TxOutIndex) -> Self {
        Self(*value)
    }
}

impl From<Day1> for StoredU64 {
    #[inline]
    fn from(value: Day1) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<Month1> for StoredU64 {
    #[inline]
    fn from(value: Month1) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<Year1> for StoredU64 {
    #[inline]
    fn from(value: Year1) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<P2PK65AddressIndex> for StoredU64 {
    #[inline]
    fn from(value: P2PK65AddressIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<P2PK33AddressIndex> for StoredU64 {
    #[inline]
    fn from(value: P2PK33AddressIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<P2PKHAddressIndex> for StoredU64 {
    #[inline]
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<OpReturnIndex> for StoredU64 {
    #[inline]
    fn from(value: OpReturnIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<P2MSOutputIndex> for StoredU64 {
    #[inline]
    fn from(value: P2MSOutputIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<P2SHAddressIndex> for StoredU64 {
    #[inline]
    fn from(value: P2SHAddressIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<P2WSHAddressIndex> for StoredU64 {
    #[inline]
    fn from(value: P2WSHAddressIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<P2WPKHAddressIndex> for StoredU64 {
    #[inline]
    fn from(value: P2WPKHAddressIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<P2TRAddressIndex> for StoredU64 {
    #[inline]
    fn from(value: P2TRAddressIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<P2AAddressIndex> for StoredU64 {
    #[inline]
    fn from(value: P2AAddressIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<UnknownOutputIndex> for StoredU64 {
    #[inline]
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<EmptyOutputIndex> for StoredU64 {
    #[inline]
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(u64::from(value))
    }
}

impl PrintableIndex for StoredU64 {
    fn to_string() -> &'static str {
        "u64"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["u64"]
    }
}

impl std::fmt::Display for StoredU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for StoredU64 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
