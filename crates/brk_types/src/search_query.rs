use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Limit, Series};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchQuery {
    /// Search query string
    pub q: Series,
    /// Maximum number of results
    #[serde(default)]
    pub limit: Limit,
}
