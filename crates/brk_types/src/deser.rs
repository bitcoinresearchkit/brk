use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, de::Error};
use serde_json::Value;

pub fn de_unquote_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    de_unquote(deserializer, Value::as_i64)
}

pub fn de_unquote_usize<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    de_unquote(deserializer, |value| value.as_u64().map(|n| n as usize))
}

fn de_unquote<'de, D, T>(
    deserializer: D,
    number: impl FnOnce(&Value) -> Option<T>,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let Some(value) = Option::<Value>::deserialize(deserializer)? else {
        return Ok(None);
    };

    if let Some(s) = value.as_str() {
        let s = s
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(s);
        if s == "null" || s.is_empty() {
            return Ok(None);
        }
        s.parse().map(Some).map_err(Error::custom)
    } else {
        number(&value)
            .map(Some)
            .ok_or_else(|| Error::custom("expected a string or number"))
    }
}

#[cfg(test)]
#[path = "../tests/unit/deser.rs"]
mod tests;
