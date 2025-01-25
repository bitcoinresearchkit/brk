use jiff::tz::TimeZone;

use super::Timestamp;

#[derive(Debug, Clone)]
pub struct Date(jiff::civil::Date);

impl From<&Timestamp> for Date {
    fn from(value: &Timestamp) -> Self {
        Self(jiff::civil::Date::from(value.to_zoned(TimeZone::UTC)))
    }
}

impl From<Date> for usize {
    // 2009-01-03 => 0
    // 2009-01-09 => 1
    // 2009-01-10 => 2
    // ...
}
