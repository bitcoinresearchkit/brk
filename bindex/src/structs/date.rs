use jiff::tz::TimeZone;

use super::Timestamp;

#[derive(Debug, Clone)]
pub struct Date(jiff::civil::Date);

impl From<&Timestamp> for Date {
    fn from(value: &Timestamp) -> Self {
        Self(jiff::civil::Date::from(value.to_zoned(TimeZone::UTC)))
    }
}
