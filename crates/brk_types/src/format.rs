use schemars::JsonSchema;
use serde::Deserialize;

#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    #[default]
    #[serde(alias = "json")]
    JSON,
    #[serde(alias = "csv")]
    CSV,
}

impl From<Option<String>> for Format {
    #[inline]
    fn from(value: Option<String>) -> Self {
        if let Some(value) = value {
            let value = value.to_lowercase();
            let value = value.as_str();
            if value == "csv" {
                return Self::CSV;
            }
        }
        Self::JSON
    }
}
