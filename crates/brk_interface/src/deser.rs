use serde::{Deserialize, Deserializer};

pub fn de_unquote_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(serde_json::Value::String(mut s)) => {
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                s = s[1..s.len() - 1].to_string();
            }
            s.parse::<i64>().map(Some).map_err(serde::de::Error::custom)
        }
        Some(serde_json::Value::Number(n)) => {
            // If it's a number, convert it to i64
            n.as_i64()
                .ok_or_else(|| serde::de::Error::custom("number out of range"))
                .map(Some)
        }
        _ => Err(serde::de::Error::custom("expected a string or number")),
    }
}

pub fn de_unquote_usize<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(serde_json::Value::String(mut s)) => {
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                s = s[1..s.len() - 1].to_string();
            }
            s.parse::<usize>()
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        Some(serde_json::Value::Number(n)) => {
            // If it's a number, convert it to usize
            n.as_u64()
                .ok_or_else(|| serde::de::Error::custom("number out of range"))
                .map(|v| v as usize)
                .map(Some)
        }
        _ => {
            dbg!(value);
            Err(serde::de::Error::custom("expected a string or number"))
        }
    }
}
