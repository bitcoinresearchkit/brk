use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer};

use crate::{Date, Timestamp};

/// A range boundary: integer index, date, or timestamp.
#[derive(Debug, Clone, Copy, JsonSchema)]
#[serde(untagged)]
pub enum RangeIndex {
    Int(i64),
    Date(Date),
    Timestamp(Timestamp),
}

impl From<i64> for RangeIndex {
    fn from(i: i64) -> Self {
        Self::Int(i)
    }
}

impl From<Date> for RangeIndex {
    fn from(d: Date) -> Self {
        Self::Date(d)
    }
}

impl From<Timestamp> for RangeIndex {
    fn from(t: Timestamp) -> Self {
        Self::Timestamp(t)
    }
}

impl fmt::Display for RangeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Date(d) => write!(f, "{d}"),
            Self::Timestamp(t) => write!(f, "{t}"),
        }
    }
}

impl<'de> Deserialize<'de> for RangeIndex {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let s = s.trim().trim_matches('"');
        if s.is_empty() {
            return Err(serde::de::Error::custom("empty range index"));
        }
        if let Ok(i) = s.parse::<i64>() {
            return Ok(Self::Int(i));
        }
        if let Some(date) = parse_date(s) {
            return Ok(Self::Date(date));
        }
        if let Ok(ts) = s.parse::<jiff::Timestamp>() {
            return Ok(Self::Timestamp(Timestamp::new(ts.as_second() as u32)));
        }
        Err(serde::de::Error::custom(format!(
            "expected integer, YYYY-MM-DD, or ISO 8601 timestamp: {s}"
        )))
    }
}

fn parse_date(s: &str) -> Option<Date> {
    if s.len() != 10 {
        return None;
    }
    let b = s.as_bytes();
    if b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    let year = s[0..4].parse().ok()?;
    let month = s[5..7].parse().ok()?;
    let day = s[8..10].parse().ok()?;
    Some(Date::new(year, month, day))
}
