use std::{fmt, mem};

use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

use super::Metric;

#[derive(Debug, Deref, JsonSchema)]
pub struct Metrics {
    /// A list of metrics
    metrics: Vec<Metric>,
}

const MAX_VECS: usize = 32;
const MAX_STRING_SIZE: usize = 64 * MAX_VECS;

impl From<Metric> for Metrics {
    #[inline]
    fn from(metric: Metric) -> Self {
        Self {
            metrics: vec![metric],
        }
    }
}

impl From<String> for Metrics {
    #[inline]
    fn from(value: String) -> Self {
        Self::from(Metric::from(value.replace("-", "_").to_lowercase()))
    }
}

impl<'a> From<Vec<&'a str>> for Metrics {
    #[inline]
    fn from(value: Vec<&'a str>) -> Self {
        Self {
            metrics: value
                .iter()
                .map(|s| Metric::from(s.replace("-", "_").to_lowercase()))
                .collect::<Vec<_>>(),
        }
    }
}

impl<'de> Deserialize<'de> for Metrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if let Some(str) = value.as_str() {
            if str.len() <= MAX_STRING_SIZE {
                Ok(Self {
                    metrics: sanitize(str.split(",").map(|s| s.to_string()))
                        .into_iter()
                        .map(Metric::from)
                        .collect(),
                })
            } else {
                Err(serde::de::Error::custom("Given parameter is too long"))
            }
        } else if let Some(vec) = value.as_array() {
            if vec.len() <= MAX_VECS {
                Ok(Self {
                    metrics: sanitize(vec.iter().map(|s| s.as_str().unwrap().to_string()))
                        .into_iter()
                        .map(Metric::from)
                        .collect(),
                })
            } else {
                Err(serde::de::Error::custom("Given parameter is too long"))
            }
        } else {
            Err(serde::de::Error::custom("Bad ids format"))
        }
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .metrics
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{s}")
    }
}

fn sanitize(dirty: impl Iterator<Item = String>) -> Vec<String> {
    let mut clean = Vec::new();
    dirty.for_each(|s| {
        let mut current = String::new();
        for c in s.to_lowercase().chars() {
            match c {
                ' ' | ',' | '+' => {
                    if !current.is_empty() {
                        clean.push(mem::take(&mut current));
                    }
                }
                '-' => current.push('_'),
                c if c.is_alphanumeric() || c == '_' => current.push(c),
                _ => {}
            }
        }
        if !current.is_empty() {
            clean.push(current);
        }
    });
    clean
}
