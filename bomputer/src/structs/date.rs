use bindex::Timestamp;
use color_eyre::eyre::eyre;
use derive_deref::Deref;
use jiff::{civil::Date as _Date, tz::TimeZone};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref)]
pub struct Date(_Date);

impl Date {
    const INDEX_ZERO: Self = Self(_Date::constant(2009, 1, 3));
    const INDEX_ONE: Self = Self(_Date::constant(2009, 1, 9));
}

impl From<_Date> for Date {
    fn from(value: _Date) -> Self {
        Self(value)
    }
}

impl From<&Timestamp> for Date {
    fn from(value: &Timestamp) -> Self {
        Self(_Date::from(value.to_zoned(TimeZone::UTC)))
    }
}

impl TryFrom<Date> for usize {
    type Error = color_eyre::Report;
    fn try_from(value: Date) -> Result<Self, Self::Error> {
        if value < Date::INDEX_ZERO {
            Err(eyre!("Date is too early"))
        } else if value == Date::INDEX_ZERO {
            Ok(0)
        } else if value < Date::INDEX_ONE {
            Err(eyre!("Date is between first and second"))
        } else if value == Date::INDEX_ONE {
            Ok(1)
        } else {
            Ok(Date::INDEX_ONE.until(*value)?.get_days() as usize + 1)
        }
    }
}
