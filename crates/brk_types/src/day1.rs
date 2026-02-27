use std::{
    fmt,
    ops::{Add, Rem},
};

use brk_error::Error;
use jiff::Span;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, Pco, PrintableIndex};

use crate::{FromCoarserIndex, Month1, Month3, Month6, Week1, Year1, Year10};

use super::Date;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
pub struct Day1(u16);

impl Day1 {
    pub const BYTES: usize = size_of::<Self>();
}

impl From<Day1> for usize {
    #[inline]
    fn from(value: Day1) -> Self {
        value.0 as usize
    }
}

impl From<Day1> for u64 {
    #[inline]
    fn from(value: Day1) -> Self {
        value.0 as u64
    }
}

impl From<usize> for Day1 {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<Day1> for i64 {
    #[inline]
    fn from(value: Day1) -> Self {
        value.0 as i64
    }
}

impl Add<usize> for Day1 {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u16)
    }
}

impl TryFrom<Date> for Day1 {
    type Error = Error;
    fn try_from(value: Date) -> Result<Self, Self::Error> {
        let value_ = jiff::civil::Date::from(value);
        if value_ < Date::INDEX_ZERO_ {
            Err(Error::UnindexableDate)
        } else if value == Date::INDEX_ZERO {
            Ok(Self(0))
        } else if value_ < Date::INDEX_ONE_ {
            Err(Error::UnindexableDate)
        } else if value == Date::INDEX_ONE {
            Ok(Self(1))
        } else {
            Ok(Self(Date::INDEX_ONE_.until(value_)?.get_days() as u16 + 1))
        }
    }
}

impl CheckedSub for Day1 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Rem<usize> for Day1 {
    type Output = Self;
    fn rem(self, rhs: usize) -> Self::Output {
        Self(self.0 % rhs as u16)
    }
}

impl FromCoarserIndex<Week1> for Day1 {
    fn min_from(coarser: Week1) -> usize {
        let coarser = usize::from(coarser);
        if coarser == 0 {
            0
        } else if coarser == 1 {
            1
        } else {
            4 + (coarser - 2) * 7
        }
    }

    fn max_from_(coarser: Week1) -> usize {
        let coarser = usize::from(coarser);
        if coarser == 0 {
            0
        } else if coarser == 1 {
            3
        } else {
            3 + (coarser - 1) * 7
        }
    }
}

impl FromCoarserIndex<Month1> for Day1 {
    fn min_from(coarser: Month1) -> usize {
        let coarser = u16::from(coarser);
        if coarser == 0 {
            0
        } else {
            let d = Date::new(2009, 1, 1)
                .into_jiff()
                .checked_add(Span::new().months(coarser))
                .unwrap();
            Day1::try_from(Date::from(d)).unwrap().into()
        }
    }

    fn max_from_(coarser: Month1) -> usize {
        let d = Date::new(2009, 1, 31)
            .into_jiff()
            .checked_add(Span::new().months(u16::from(coarser)))
            .unwrap();
        Day1::try_from(Date::from(d)).unwrap().into()
    }
}

impl FromCoarserIndex<Month3> for Day1 {
    fn min_from(coarser: Month3) -> usize {
        let coarser = u8::from(coarser);
        if coarser == 0 {
            0
        } else {
            let d = Date::new(2009, 1, 1)
                .into_jiff()
                .checked_add(Span::new().months(3 * coarser))
                .unwrap();
            Day1::try_from(Date::from(d)).unwrap().into()
        }
    }

    fn max_from_(coarser: Month3) -> usize {
        let d = Date::new(2009, 3, 31)
            .into_jiff()
            .checked_add(Span::new().months(3 * u8::from(coarser)))
            .unwrap();
        Day1::try_from(Date::from(d)).unwrap().into()
    }
}

impl FromCoarserIndex<Month6> for Day1 {
    fn min_from(coarser: Month6) -> usize {
        let coarser = u8::from(coarser);
        if coarser == 0 {
            0
        } else {
            let d = Date::new(2009, 1, 1)
                .into_jiff()
                .checked_add(Span::new().months(6 * coarser))
                .unwrap();
            Day1::try_from(Date::from(d)).unwrap().into()
        }
    }

    fn max_from_(coarser: Month6) -> usize {
        let d = Date::new(2009, 5, 31)
            .into_jiff()
            .checked_add(Span::new().months(1 + 6 * u8::from(coarser)))
            .unwrap();
        Day1::try_from(Date::from(d)).unwrap().into()
    }
}

impl FromCoarserIndex<Year1> for Day1 {
    fn min_from(coarser: Year1) -> usize {
        let coarser = u8::from(coarser);
        if coarser == 0 {
            0
        } else {
            Self::try_from(Date::new(2009 + coarser as u16, 1, 1))
                .unwrap()
                .into()
        }
    }

    fn max_from_(coarser: Year1) -> usize {
        Self::try_from(Date::new(2009 + u8::from(coarser) as u16, 12, 31))
            .unwrap()
            .into()
    }
}

impl FromCoarserIndex<Year10> for Day1 {
    fn min_from(coarser: Year10) -> usize {
        let coarser = u8::from(coarser);
        if coarser == 0 {
            0
        } else {
            Self::try_from(Date::new(2000 + 10 * coarser as u16, 1, 1))
                .unwrap()
                .into()
        }
    }

    fn max_from_(coarser: Year10) -> usize {
        let coarser = u8::from(coarser);
        Self::try_from(Date::new(2009 + 10 * coarser as u16, 12, 31))
            .unwrap()
            .into()
    }
}

impl PrintableIndex for Day1 {
    fn to_string() -> &'static str {
        "day1"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["1d", "d", "date", "daily", "day1", "dateindex"]
    }
}

impl fmt::Display for Day1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for Day1 {
    #[inline(always)]
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}
