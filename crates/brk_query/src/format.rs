use clap::ValueEnum;
use color_eyre::eyre::eyre;
use serde::Deserialize;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Deserialize)]
pub enum Format {
    #[serde(alias = "json")]
    JSON,
    #[serde(alias = "csv")]
    CSV,
    #[serde(alias = "tsv")]
    TSV,
    #[serde(alias = "md", alias = "markdown")]
    #[value(alias("markdown"))]
    MD,
}

impl TryFrom<Option<String>> for Format {
    type Error = color_eyre::Report;
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
                Err(eyre!("Fail"))
            }
        } else {
            Err(eyre!("Fail"))
        }
    }
}
