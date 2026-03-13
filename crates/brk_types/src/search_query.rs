use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Limit, Metric};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchQuery {
    /// Search query string
    pub q: Metric,
    /// Maximum number of results
    #[serde(default)]
    pub limit: Limit,
}
