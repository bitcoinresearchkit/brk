use std::fmt;

use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deref, JsonSchema)]
pub struct MaybeIds(Vec<String>);

const MAX_STRING_SIZE: usize = 10_000;
const MAX_VECS: usize = 64;

impl From<String> for MaybeIds {
    fn from(value: String) -> Self {
        Self(vec![value])
    }
}

impl<'a> From<Vec<&'a str>> for MaybeIds {
    fn from(value: Vec<&'a str>) -> Self {
        Self(value.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }
}

impl<'de> Deserialize<'de> for MaybeIds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::String(str) => {
                if str.len() <= MAX_STRING_SIZE {
                    Ok(MaybeIds(sanitize_ids(
                        str.split(",").map(|s| s.to_string()),
                    )))
                } else {
                    dbg!(str.len(), MAX_STRING_SIZE);
                    Err(serde::de::Error::custom("Given parameter is too long"))
                }
            }
            serde_json::Value::Array(vec) => {
                if vec.len() <= MAX_VECS {
                    Ok(MaybeIds(sanitize_ids(
                        vec.into_iter().map(|s| s.as_str().unwrap().to_string()),
                    )))
                } else {
                    dbg!(vec.len(), MAX_VECS);
                    Err(serde::de::Error::custom("Given parameter is too long"))
                }
            }
            _ => Err(serde::de::Error::custom("Bad ids format")),
        }
    }
}

impl fmt::Display for MaybeIds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.join(",");
        write!(f, "{s}")
    }
}

fn sanitize_ids(raw_ids: impl Iterator<Item = String>) -> Vec<String> {
    let mut results = Vec::new();
    raw_ids.for_each(|s| {
        let mut current = String::new();
        for c in s.to_lowercase().chars() {
            match c {
                ' ' | ',' | '+' => {
                    if !current.is_empty() {
                        results.push(std::mem::take(&mut current));
                    }
                }
                '-' => current.push('_'),
                c if c.is_alphanumeric() || c == '_' => current.push(c),
                _ => {}
            }
        }
        if !current.is_empty() {
            results.push(current);
        }
    });
    results
}
