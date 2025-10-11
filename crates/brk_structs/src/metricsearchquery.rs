use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
/// Search query parameters for finding metrics by name
pub struct MetricSearchQuery {
    /// Search query string. Supports fuzzy matching, partial matches, and typos.
    #[schemars(example = &"price", example = &"low", example = &"sth", example = &"realized", example = &"pric")]
    pub q: String,

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
    pub limit: usize,
}

fn default_search_limit() -> usize {
    100
}
