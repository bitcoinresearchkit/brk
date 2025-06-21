use std::ops::{Add, AddAssign};

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable};

use super::Vout;

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
pub struct OutputIndex(u64);

impl OutputIndex {
    pub const COINBASE: Self = Self(u64::MAX);

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }

    pub fn is_coinbase(self) -> bool {
        self == Self::COINBASE
    }
}

impl Add<OutputIndex> for OutputIndex {
    type Output = Self;
    fn add(self, rhs: OutputIndex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Vout> for OutputIndex {
    type Output = Self;
    fn add(self, rhs: Vout) -> Self::Output {
        Self(self.0 + u64::from(rhs))
    }
}

impl Add<usize> for OutputIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u64)
    }
}

impl AddAssign<OutputIndex> for OutputIndex {
    fn add_assign(&mut self, rhs: OutputIndex) {
        self.0 += rhs.0
    }
}

impl CheckedSub<OutputIndex> for OutputIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self::from)
    }
}

impl From<OutputIndex> for u32 {
    fn from(value: OutputIndex) -> Self {
        if value.0 > u32::MAX as u64 {
            panic!()
        }
        value.0 as u32
    }
}

impl From<u64> for OutputIndex {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<OutputIndex> for u64 {
    fn from(value: OutputIndex) -> Self {
        value.0
    }
}

impl From<usize> for OutputIndex {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
impl From<OutputIndex> for usize {
    fn from(value: OutputIndex) -> Self {
        value.0 as usize
    }
}

impl Printable for OutputIndex {
    fn to_string() -> &'static str {
        "outputindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["out", "outputindex"]
    }
}
