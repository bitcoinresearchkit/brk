use jiff::{Span, civil::Date as Date_, tz::TimeZone};
use serde::{Serialize, Serializer};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{DateIndex, Timestamp};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, Immutable, IntoBytes, KnownLayout,
)]
pub struct Date(u32);

impl Date {
    pub const INDEX_ZERO: Self = Self(20090103);
    pub const INDEX_ZERO_: Date_ = Date_::constant(2009, 1, 3);
    pub const INDEX_ONE: Self = Self(20090109);
    pub const INDEX_ONE_: Date_ = Date_::constant(2009, 1, 9);

    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self(year as u32 * 1_00_00 + month as u32 * 1_00 + day as u32)
    }

    pub fn year(&self) -> u16 {
        (self.0 / 1_00_00) as u16
    }

    pub fn month(&self) -> u8 {
        ((self.0 % 1_00_00) / 1_00) as u8
    }

    pub fn day(&self) -> u8 {
        (self.0 % 1_00) as u8
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::INDEX_ZERO
    }
}

impl From<Date_> for Date {
    fn from(value: Date_) -> Self {
        Self::new(value.year() as u16, value.month() as u8, value.day() as u8)
    }
}

impl From<Date> for Date_ {
    fn from(value: Date) -> Self {
        Self::new(value.year() as i16, value.month() as i8, value.day() as i8).unwrap()
    }
}

impl From<Timestamp> for Date {
    fn from(value: Timestamp) -> Self {
        Self::from(Date_::from(
            jiff::Timestamp::from(value).to_zoned(TimeZone::UTC),
        ))
    }
}

impl From<DateIndex> for Date {
    fn from(value: DateIndex) -> Self {
        if value == DateIndex::default() {
            Date::INDEX_ZERO
        } else {
            Self::from(
                Self::INDEX_ONE_
                    .checked_add(Span::new().days(i64::from(value) - 1))
                    .unwrap(),
            )
        }
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}-{:0>2}-{:0>2}",
            self.year(),
            self.month(),
            self.day()
        ))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
