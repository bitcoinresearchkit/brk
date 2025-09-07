use brk_error::Error;
use schemars::JsonSchema;
use serde::Deserialize;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    #[serde(alias = "json")]
    JSON,
    #[serde(alias = "csv")]
    CSV,
    #[serde(alias = "tsv")]
    TSV,
    #[serde(alias = "md", alias = "markdown")]
    MD,
}

impl TryFrom<Option<String>> for Format {
    type Error = Error;
    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        if let Some(value) = value {
            let value = value.to_lowercase();
            let value = value.as_str();
            if value == "md" || value == "markdown" {
                Ok(Self::MD)
            } else if value == "csv" {
                Ok(Self::CSV)
            } else if value == "tsv" {
                Ok(Self::TSV)
            } else if value == "json" {
                Ok(Self::JSON)
            } else {
                Err(Error::Str("Fail"))
            }
        } else {
            Err(Error::Str("Fail"))
        }
    }
}
