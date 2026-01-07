use derive_more::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

/// Maximum number of results to return. Defaults to 100 if not specified.
#[derive(Debug, Deref, Deserialize, JsonSchema)]
#[serde(transparent)]
#[allow(clippy::duplicated_attributes)]
#[schemars(default, example = 1, example = 10, example = 100)]
pub struct Limit(usize);

impl Limit {
    pub const MIN: Self = Self(1);
    pub const DEFAULT: Self = Self(100);
}

impl Default for Limit {
    fn default() -> Self {
        Self::DEFAULT
    }
}
