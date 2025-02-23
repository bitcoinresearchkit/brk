use std::ops::Add;

use color_eyre::eyre::eyre;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Date;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct Dateindex(u16);

impl From<Dateindex> for usize {
    fn from(value: Dateindex) -> Self {
        value.0 as usize
    }
}

impl From<usize> for Dateindex {
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<Dateindex> for i64 {
    fn from(value: Dateindex) -> Self {
        value.0 as i64
    }
}

impl Add<usize> for Dateindex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u16)
    }
}

impl TryFrom<Date> for Dateindex {
    type Error = color_eyre::Report;
    fn try_from(value: Date) -> Result<Self, Self::Error> {
        let value_ = jiff::civil::Date::from(value);
        if value_ < Date::INDEX_ZERO_ {
            Err(eyre!("Date is too early"))
        } else if value == Date::INDEX_ZERO {
            Ok(Self(0))
        } else if value_ < Date::INDEX_ONE_ {
            Err(eyre!("Date is between first and second"))
        } else if value == Date::INDEX_ONE {
            Ok(Self(1))
        } else {
            Ok(Self(Date::INDEX_ONE_.until(value_)?.get_days() as u16 + 1))
        }
    }
}
