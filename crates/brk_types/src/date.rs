use jiff::{Span, Zoned, civil::Date as Date_, tz::TimeZone};
use serde::{Serialize, Serializer};
use vecdb::{Formattable, Pco};

use crate::ONE_DAY_IN_SEC_F64;

use super::{DateIndex, Timestamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Pco)]
pub struct Date(u32);

impl Date {
    pub const INDEX_ZERO: Self = Self(20090103);
    pub const INDEX_ZERO_: Date_ = Date_::constant(2009, 1, 3);
    pub const INDEX_ONE: Self = Self(20090109);
    pub const INDEX_ONE_: Date_ = Date_::constant(2009, 1, 9);
    pub const MIN_RATIO: Self = Self(20120101);

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

    pub fn into_jiff(self) -> Date_ {
        self.into()
    }

    pub fn today() -> Self {
        Self::from(Timestamp::now())
    }

    pub fn completion(&self) -> f64 {
        let date = Date_::from(*self);
        let now = Zoned::now().with_time_zone(TimeZone::UTC);
        let today = now.date();

        if date < today {
            1.0
        } else if date == today {
            let rounded = jiff::Timestamp::from(*self);
            now.timestamp().duration_since(rounded).as_secs_f64() / ONE_DAY_IN_SEC_F64
        } else {
            0.0
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::INDEX_ZERO
    }
}

impl From<Date_> for Date {
    #[inline]
    fn from(value: Date_) -> Self {
        Self::new(value.year() as u16, value.month() as u8, value.day() as u8)
    }
}

impl From<Date> for Date_ {
    #[inline]
    fn from(value: Date) -> Self {
        Self::new(value.year() as i16, value.month() as i8, value.day() as i8).unwrap()
    }
}

impl From<Date> for jiff::Timestamp {
    #[inline]
    fn from(value: Date) -> Self {
        Self::from(Timestamp::from(value))
    }
}

impl From<Timestamp> for Date {
    #[inline]
    fn from(value: Timestamp) -> Self {
        Self::from(Date_::from(
            jiff::Timestamp::from(value).to_zoned(TimeZone::UTC),
        ))
    }
}

impl From<DateIndex> for Date {
    #[inline]
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

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();

        f.write_str(buf.format(self.year()))?;
        f.write_str("-")?;

        let month = self.month();
        if month < 10 {
            f.write_str("0")?;
        }
        f.write_str(buf.format(month))?;
        f.write_str("-")?;

        let day = self.day();
        if day < 10 {
            f.write_str("0")?;
        }

        f.write_str(buf.format(day))
    }
}

impl Formattable for Date {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
