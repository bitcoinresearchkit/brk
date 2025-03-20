use std::{fmt::Debug, ops::Add};

use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::{Date, Dateindex, Monthindex};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
)]
pub struct Yearindex(u8);

impl From<u8> for Yearindex {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for Yearindex {
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<Yearindex> for usize {
    fn from(value: Yearindex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Yearindex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}

impl From<Dateindex> for Yearindex {
    fn from(value: Dateindex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for Yearindex {
    fn from(value: Date) -> Self {
        Self((value.year() - 2009) as u8)
    }
}

impl From<Yearindex> for u16 {
    fn from(value: Yearindex) -> Self {
        value.0 as u16
    }
}

impl CheckedSub for Yearindex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl From<Monthindex> for Yearindex {
    fn from(value: Monthindex) -> Self {
        Self((usize::from(value) / 12) as u8)
    }
}
