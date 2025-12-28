use std::{
    fmt,
    ops::{Add, Rem},
};

use brk_error::Error;
use jiff::Span;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{CheckedSub, Formattable, FromCoarserIndex, Pco, PrintableIndex};

use crate::{DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, WeekIndex, YearIndex};

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
pub struct DateIndex(u16);

impl DateIndex {
    pub const BYTES: usize = size_of::<Self>();
}

impl From<DateIndex> for usize {
    #[inline]
    fn from(value: DateIndex) -> Self {
        value.0 as usize
    }
}

impl From<DateIndex> for u64 {
    #[inline]
    fn from(value: DateIndex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for DateIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u16)
    }
}

impl From<DateIndex> for i64 {
    #[inline]
    fn from(value: DateIndex) -> Self {
        value.0 as i64
    }
}

impl Add<usize> for DateIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u16)
    }
}

impl TryFrom<Date> for DateIndex {
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

impl CheckedSub for DateIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Rem<usize> for DateIndex {
    type Output = Self;
    fn rem(self, rhs: usize) -> Self::Output {
        Self(self.0 % rhs as u16)
    }
}

impl FromCoarserIndex<WeekIndex> for DateIndex {
    fn min_from(coarser: WeekIndex) -> usize {
        let coarser = usize::from(coarser);
        if coarser == 0 {
            0
        } else if coarser == 1 {
            1
        } else {
            4 + (coarser - 2) * 7
        }
    }

    fn max_from_(coarser: WeekIndex) -> usize {
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

impl FromCoarserIndex<MonthIndex> for DateIndex {
    fn min_from(coarser: MonthIndex) -> usize {
        let coarser = u16::from(coarser);
        if coarser == 0 {
            0
        } else {
            let d = Date::new(2009, 1, 1)
                .into_jiff()
                .checked_add(Span::new().months(coarser))
                .unwrap();
            DateIndex::try_from(Date::from(d)).unwrap().into()
        }
    }

    fn max_from_(coarser: MonthIndex) -> usize {
        let d = Date::new(2009, 1, 31)
            .into_jiff()
            .checked_add(Span::new().months(u16::from(coarser)))
            .unwrap();
        DateIndex::try_from(Date::from(d)).unwrap().into()
    }
}

impl FromCoarserIndex<QuarterIndex> for DateIndex {
    fn min_from(coarser: QuarterIndex) -> usize {
        let coarser = u16::from(coarser);
        if coarser == 0 {
            0
        } else {
            let d = Date::new(2009, 1, 1)
                .into_jiff()
                .checked_add(Span::new().months(3 * coarser))
                .unwrap();
            DateIndex::try_from(Date::from(d)).unwrap().into()
        }
    }

    fn max_from_(coarser: QuarterIndex) -> usize {
        let d = Date::new(2009, 3, 31)
            .into_jiff()
            .checked_add(Span::new().months(3 * u16::from(coarser)))
            .unwrap();
        DateIndex::try_from(Date::from(d)).unwrap().into()
    }
}

impl FromCoarserIndex<SemesterIndex> for DateIndex {
    fn min_from(coarser: SemesterIndex) -> usize {
        let coarser = u16::from(coarser);
        if coarser == 0 {
            0
        } else {
            let d = Date::new(2009, 1, 1)
                .into_jiff()
                .checked_add(Span::new().months(6 * coarser))
                .unwrap();
            DateIndex::try_from(Date::from(d)).unwrap().into()
        }
    }

    fn max_from_(coarser: SemesterIndex) -> usize {
        let d = Date::new(2009, 5, 31)
            .into_jiff()
            .checked_add(Span::new().months(1 + 6 * u16::from(coarser)))
            .unwrap();
        DateIndex::try_from(Date::from(d)).unwrap().into()
    }
}

impl FromCoarserIndex<YearIndex> for DateIndex {
    fn min_from(coarser: YearIndex) -> usize {
        let coarser = u16::from(coarser);
        if coarser == 0 {
            0
        } else {
            Self::try_from(Date::new(2009 + coarser, 1, 1))
                .unwrap()
                .into()
        }
    }

    fn max_from_(coarser: YearIndex) -> usize {
        Self::try_from(Date::new(2009 + u16::from(coarser), 12, 31))
            .unwrap()
            .into()
    }
}

impl FromCoarserIndex<DecadeIndex> for DateIndex {
    fn min_from(coarser: DecadeIndex) -> usize {
        let coarser = u16::from(coarser);
        if coarser == 0 {
            0
        } else {
            Self::try_from(Date::new(2000 + 10 * coarser, 1, 1))
                .unwrap()
                .into()
        }
    }

    fn max_from_(coarser: DecadeIndex) -> usize {
        let coarser = u16::from(coarser);
        Self::try_from(Date::new(2009 + (10 * coarser), 12, 31))
            .unwrap()
            .into()
    }
}

impl PrintableIndex for DateIndex {
    fn to_string() -> &'static str {
        "dateindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["d", "date", "dateindex"]
    }
}

impl fmt::Display for DateIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for DateIndex {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
