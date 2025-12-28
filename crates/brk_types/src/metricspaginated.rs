use std::borrow::Cow;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A paginated list of available metric names (1000 per page)
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PaginatedMetrics {
    /// Current page number (0-indexed)
    #[schemars(example = 0)]
    pub current_page: usize,
    /// Maximum valid page index (0-indexed)
    #[schemars(example = 21)]
    pub max_page: usize,
    /// List of metric names (max 1000 per page)
    pub metrics: Vec<Cow<'static, str>>,
}
