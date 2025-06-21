use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deref, JsonSchema)]
pub struct MaybeIds(Vec<String>);

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
        let maybe_ids = match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::String(str) => {
                str.split(",").map(|s| s.to_string()).collect::<Vec<_>>()
            }
            serde_json::Value::Array(vec) => vec
                .into_iter()
                .map(|s| s.as_str().unwrap().to_string())
                .collect::<Vec<_>>(),
            _ => return Err(serde::de::Error::custom("Bad ids format")),
        };
        // dbg!(&maybe_ids);
        Ok(MaybeIds(maybe_ids))
    }
}
