use std::fmt;

use jiff::{Span, Zoned, civil::Date as Date_, tz::TimeZone};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};
use vecdb::{Formattable, Pco};

use crate::ONE_DAY_IN_SEC_F64;

use super::{DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Timestamp, WeekIndex, YearIndex};

/// Date in YYYYMMDD format stored as u32
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Pco, JsonSchema)]
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

impl From<WeekIndex> for Date {
    #[inline]
    fn from(value: WeekIndex) -> Self {
        // Week 0 starts at genesis (2009-01-03), add i weeks
        Self::from(
            Self::INDEX_ZERO_
                .checked_add(Span::new().weeks(i64::from(u16::from(value))))
                .unwrap(),
        )
    }
}

impl From<MonthIndex> for Date {
    #[inline]
    fn from(value: MonthIndex) -> Self {
        // Month 0 is January 2009, add i months
        Self::from(
            Date_::constant(2009, 1, 1)
                .checked_add(Span::new().months(i64::from(u16::from(value))))
                .unwrap(),
        )
    }
}

impl From<YearIndex> for Date {
    #[inline]
    fn from(value: YearIndex) -> Self {
        // Year 0 is 2009
        let year = 2009i16 + usize::from(value) as i16;
        Self::from(Date_::constant(year, 1, 1))
    }
}

impl From<QuarterIndex> for Date {
    #[inline]
    fn from(value: QuarterIndex) -> Self {
        // Quarter 0 is Q1 2009, add i*3 months
        Self::from(
            Date_::constant(2009, 1, 1)
                .checked_add(Span::new().months(usize::from(value) as i64 * 3))
                .unwrap(),
        )
    }
}

impl From<SemesterIndex> for Date {
    #[inline]
    fn from(value: SemesterIndex) -> Self {
        // Semester 0 is H1 2009, add i*6 months
        Self::from(
            Date_::constant(2009, 1, 1)
                .checked_add(Span::new().months(usize::from(value) as i64 * 6))
                .unwrap(),
        )
    }
}

impl From<DecadeIndex> for Date {
    #[inline]
    fn from(value: DecadeIndex) -> Self {
        // Decade 0 is 2009, add i*10 years
        let year = 2009i16 + usize::from(value) as i16 * 10;
        Self::from(Date_::constant(year, 1, 1))
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

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateVisitor;

        impl Visitor<'_> for DateVisitor {
            type Value = Date;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a date string in YYYY-MM-DD format")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Parse YYYY-MM-DD format
                if v.len() != 10 {
                    return Err(E::invalid_length(v.len(), &self));
                }

                let year: u16 = v[0..4]
                    .parse()
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &self))?;
                let month: u8 = v[5..7]
                    .parse()
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &self))?;
                let day: u8 = v[8..10]
                    .parse()
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &self))?;

                Ok(Date::new(year, month, day))
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_from_dateindex_zero() {
        // DateIndex 0 is genesis: Jan 3, 2009
        let date = Date::from(DateIndex::from(0_usize));
        assert_eq!(date, Date::INDEX_ZERO);
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 3);
    }

    #[test]
    fn test_date_from_dateindex_one() {
        // DateIndex 1 is Jan 9, 2009 (6 day gap after genesis)
        let date = Date::from(DateIndex::from(1_usize));
        assert_eq!(date, Date::INDEX_ONE);
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 9);
    }

    #[test]
    fn test_date_from_dateindex_two() {
        // DateIndex 2 is Jan 10, 2009
        let date = Date::from(DateIndex::from(2_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 10);
    }

    #[test]
    fn test_date_from_weekindex_zero() {
        // WeekIndex 0 starts at genesis: Jan 3, 2009
        let date = Date::from(WeekIndex::from(0_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 3);
    }

    #[test]
    fn test_date_from_weekindex_one() {
        // WeekIndex 1 is Jan 10, 2009 (one week after genesis)
        let date = Date::from(WeekIndex::from(1_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 10);
    }

    #[test]
    fn test_date_from_monthindex_zero() {
        // MonthIndex 0 is Jan 1, 2009
        let date = Date::from(MonthIndex::from(0_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_monthindex_one() {
        // MonthIndex 1 is Feb 1, 2009
        let date = Date::from(MonthIndex::from(1_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 2);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_monthindex_twelve() {
        // MonthIndex 12 is Jan 1, 2010
        let date = Date::from(MonthIndex::from(12_usize));
        assert_eq!(date.year(), 2010);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_yearindex_zero() {
        // YearIndex 0 is Jan 1, 2009
        let date = Date::from(YearIndex::from(0_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_yearindex_one() {
        // YearIndex 1 is Jan 1, 2010
        let date = Date::from(YearIndex::from(1_usize));
        assert_eq!(date.year(), 2010);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_quarterindex_zero() {
        // QuarterIndex 0 is Q1 2009: Jan 1, 2009
        let date = Date::from(QuarterIndex::from(0_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_quarterindex_one() {
        // QuarterIndex 1 is Q2 2009: Apr 1, 2009
        let date = Date::from(QuarterIndex::from(1_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 4);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_quarterindex_four() {
        // QuarterIndex 4 is Q1 2010: Jan 1, 2010
        let date = Date::from(QuarterIndex::from(4_usize));
        assert_eq!(date.year(), 2010);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_semesterindex_zero() {
        // SemesterIndex 0 is H1 2009: Jan 1, 2009
        let date = Date::from(SemesterIndex::from(0_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_semesterindex_one() {
        // SemesterIndex 1 is H2 2009: Jul 1, 2009
        let date = Date::from(SemesterIndex::from(1_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 7);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_semesterindex_two() {
        // SemesterIndex 2 is H1 2010: Jan 1, 2010
        let date = Date::from(SemesterIndex::from(2_usize));
        assert_eq!(date.year(), 2010);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_decadeindex_zero() {
        // DecadeIndex 0 is 2009: Jan 1, 2009
        let date = Date::from(DecadeIndex::from(0_usize));
        assert_eq!(date.year(), 2009);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }

    #[test]
    fn test_date_from_decadeindex_one() {
        // DecadeIndex 1 is 2019: Jan 1, 2019
        let date = Date::from(DecadeIndex::from(1_usize));
        assert_eq!(date.year(), 2019);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 1);
    }
}
