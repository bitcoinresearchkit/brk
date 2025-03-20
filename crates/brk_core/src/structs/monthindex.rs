use std::{fmt::Debug, ops::Add};

use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::{Date, Dateindex, Yearindex};

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
pub struct Monthindex(u16);

impl From<u16> for Monthindex {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<usize> for Monthindex {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<Monthindex> for usize {
    fn from(value: Monthindex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Monthindex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u16)
    }
}

impl From<Dateindex> for Monthindex {
    fn from(value: Dateindex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for Monthindex {
    fn from(value: Date) -> Self {
        Self(u16::from(Yearindex::from(value)) * 12 + value.month() as u16 - 1)
    }
}

impl CheckedSub for Monthindex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}
