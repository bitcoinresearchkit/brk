use std::ops::Add;

use bindex::Timestamp;
use color_eyre::eyre::eyre;
use jiff::{civil::Date as Date_, tz::TimeZone, Span};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    const INDEX_ZERO: Self = Self {
        year: 2009,
        month: 1,
        day: 3,
    };
    const INDEX_ZERO_: Date_ = Date_::constant(2009, 1, 3);
    const INDEX_ONE: Self = Self {
        year: 2009,
        month: 1,
        day: 9,
    };
    const INDEX_ONE_: Date_ = Date_::constant(2009, 1, 9);
}

impl Default for Date {
    fn default() -> Self {
        Self::INDEX_ZERO
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Date_::from(*self).cmp(&Date_::from(*other))
    }
}

impl From<Date_> for Date {
    fn from(value: Date_) -> Self {
        Self {
            year: value.year() as u16,
            month: value.month() as u8,
            day: value.day() as u8,
        }
    }
}

impl From<Date> for Date_ {
    fn from(value: Date) -> Self {
        Self::new(value.year as i16, value.month as i8, value.day as i8).unwrap()
    }
}

impl From<Timestamp> for Date {
    fn from(value: Timestamp) -> Self {
        Self::from(Date_::from(jiff::Timestamp::from(value).to_zoned(TimeZone::UTC)))
    }
}

impl TryFrom<Date> for usize {
    type Error = color_eyre::Report;
    fn try_from(value: Date) -> Result<Self, Self::Error> {
        let value_ = Date_::from(value);
        if value_ < Date::INDEX_ZERO_ {
            Err(eyre!("Date is too early"))
        } else if value == Date::INDEX_ZERO {
            Ok(0)
        } else if value_ < Date::INDEX_ONE_ {
            Err(eyre!("Date is between first and second"))
        } else if value == Date::INDEX_ONE {
            Ok(1)
        } else {
            Ok(Date_::from(Date::INDEX_ONE).until(value_)?.get_days() as usize + 1)
        }
    }
}

impl From<usize> for Date {
    fn from(value: usize) -> Self {
        Self::from(Self::INDEX_ZERO_.checked_add(Span::new().days(value as i64)).unwrap())
    }
}

impl Add<usize> for Date {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self::from(Date_::from(self).checked_add(Span::new().days(rhs as i64)).unwrap())
    }
}
