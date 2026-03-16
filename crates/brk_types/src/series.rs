use std::fmt::Display;

use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Series name
#[derive(Debug, Clone, Deref, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
#[schemars(
    with = "String",
    example = &"price_close",
    example = &"market_cap",
    example = &"realized_price"
)]
pub struct Series(String);

impl From<String> for Series {
    #[inline]
    fn from(series: String) -> Self {
        Self(series)
    }
}

impl From<&str> for Series {
    #[inline]
    fn from(series: &str) -> Self {
        Self(series.to_owned())
    }
}

impl Display for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
