use std::ops::{Add, AddAssign};

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable};

use super::Vin;

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
pub struct InputIndex(u64);

impl InputIndex {
    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }
}

impl Add<InputIndex> for InputIndex {
    type Output = Self;
    fn add(self, rhs: InputIndex) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Vin> for InputIndex {
    type Output = Self;
    fn add(self, rhs: Vin) -> Self::Output {
        Self(self.0 + u64::from(rhs))
    }
}

impl Add<usize> for InputIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u64)
    }
}

impl AddAssign<InputIndex> for InputIndex {
    fn add_assign(&mut self, rhs: InputIndex) {
        self.0 += rhs.0
    }
}

impl CheckedSub<InputIndex> for InputIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self::from)
    }
}

impl From<InputIndex> for u32 {
    fn from(value: InputIndex) -> Self {
        if value.0 > u32::MAX as u64 {
            panic!()
        }
        value.0 as u32
    }
}

impl From<u64> for InputIndex {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<InputIndex> for u64 {
    fn from(value: InputIndex) -> Self {
        value.0
    }
}

impl From<usize> for InputIndex {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
impl From<InputIndex> for usize {
    fn from(value: InputIndex) -> Self {
        value.0 as usize
    }
}

impl Printable for InputIndex {
    fn to_string() -> &'static str {
        "inputindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["in", "inputindex"]
    }
}
