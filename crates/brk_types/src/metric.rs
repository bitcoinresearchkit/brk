use std::fmt::Display;

use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

/// Metric name
#[derive(Debug, Clone, Deref, Deserialize, JsonSchema)]
#[serde(transparent)]
#[schemars(
    with = "String",
    example = &"price_close",
    example = &"market_cap",
    example = &"realized_price"
)]
pub struct Metric(String);

impl From<String> for Metric {
    #[inline]
    fn from(metric: String) -> Self {
        Self(metric)
    }
}

impl From<&str> for Metric {
    #[inline]
    fn from(metric: &str) -> Self {
        Self(metric.to_owned())
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
