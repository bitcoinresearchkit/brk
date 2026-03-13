use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Index, Pagination};

/// Pagination parameters with an index filter
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PaginationIndex {
    /// The index to filter by
    pub index: Index,
    #[serde(flatten)]
    pub pagination: Pagination,
}
