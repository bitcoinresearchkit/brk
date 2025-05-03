use std::ops::{Add, AddAssign};

use byteview::ByteView;
use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Error};

use super::StoredU32;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    DerefMut,
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct TxIndex(u32);

impl TxIndex {
    pub const ZERO: Self = Self(0);

    pub fn new(txindex: u32) -> Self {
        Self(txindex)
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }
}

impl Add<TxIndex> for TxIndex {
    type Output = Self;
    fn add(self, rhs: TxIndex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<usize> for TxIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl AddAssign<TxIndex> for TxIndex {
    fn add_assign(&mut self, rhs: TxIndex) {
        self.0 += rhs.0
    }
}

impl CheckedSub<TxIndex> for TxIndex {
    fn checked_sub(self, rhs: TxIndex) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(TxIndex::from)
    }
}

impl From<u32> for TxIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for TxIndex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<TxIndex> for u64 {
    fn from(value: TxIndex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for TxIndex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<TxIndex> for usize {
    fn from(value: TxIndex) -> Self {
        value.0 as usize
    }
}

impl TryFrom<ByteView> for TxIndex {
    type Error = Error;
    fn try_from(value: ByteView) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<TxIndex> for ByteView {
    fn from(value: TxIndex) -> Self {
        Self::new(value.as_bytes())
    }
}

impl From<TxIndex> for StoredU32 {
    fn from(value: TxIndex) -> Self {
        Self::from(value.0)
    }
}
