use std::fmt::Display;

use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, Deref, Deserialize, JsonSchema)]
pub struct Metric {
    /// Metric name
    #[schemars(example = &"price_close", example = &"market_cap", example = &"realized_price")]
    metric: String,
}

impl From<String> for Metric {
    #[inline]
    fn from(metric: String) -> Self {
        Self { metric }
    }
}

impl From<&str> for Metric {
    #[inline]
    fn from(metric: &str) -> Self {
        Self {
            metric: metric.to_string(),
        }
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.metric)
    }
}
