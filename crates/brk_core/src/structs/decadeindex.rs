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
pub struct Decadeindex(u8);

impl From<u8> for Decadeindex {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for Decadeindex {
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<Decadeindex> for usize {
    fn from(value: Decadeindex) -> Self {
        value.0 as usize
    }
}

impl Add<usize> for Decadeindex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u8)
    }
}

impl From<Dateindex> for Decadeindex {
    fn from(value: Dateindex) -> Self {
        Self::from(Date::from(value))
    }
}

impl From<Date> for Decadeindex {
    fn from(value: Date) -> Self {
        let year = value.year();
        if year < 2000 {
            panic!("unsupported")
        }
        Self(((year - 2000) / 10) as u8)
    }
}

impl CheckedSub for Decadeindex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl From<Yearindex> for Decadeindex {
    fn from(value: Yearindex) -> Self {
        let v = usize::from(value);
        if v == 0 {
            Self(0)
        } else {
            Self((((v - 1) / 10) + 1) as u8)
        }
    }
}
