use derive_deref::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deref, Deserialize, JsonSchema)]
pub struct Limit {
    /// Maximum number of results to return. Defaults to 100 if not specified.
    #[serde(default = "default_search_limit")]
    #[schemars(
        example = "1",
        example = "10",
        example = "100",
        example = "1000",
        example = "10000",
        example = "100000"
    )]
    limit: usize,
}

impl Limit {
    pub const MIN: Self = Self { limit: 1 };
}

fn default_search_limit() -> usize {
    100
}
