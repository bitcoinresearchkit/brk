use brk_types::Index;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::deser::de_unquote_usize;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct PaginationParam {
    #[schemars(description = "Pagination index")]
    #[serde(default, alias = "p", deserialize_with = "de_unquote_usize")]
    pub page: Option<usize>,
}

impl PaginationParam {
    pub const PER_PAGE: usize = 1_000;

    pub fn start(&self, len: usize) -> usize {
        (self.page.unwrap_or_default() * Self::PER_PAGE).clamp(0, len)
    }

    pub fn end(&self, len: usize) -> usize {
        ((self.page.unwrap_or_default() + 1) * Self::PER_PAGE).clamp(0, len)
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PaginatedIndexParam {
    pub index: Index,
    #[serde(flatten)]
    pub pagination: PaginationParam,
}

/// A paginated list of available metric names (1000 per page)
#[derive(Debug, Serialize, JsonSchema)]
pub struct PaginatedMetrics {
    /// Current page number (0-indexed)
    #[schemars(example = 0)]
    pub current_page: usize,
    /// Maximum valid page index (0-indexed)
    #[schemars(example = 21000)]
    pub max_page: usize,
    /// List of metric names (max 1000 per page)
    #[schemars(example = ["price_open", "price_close", "realized_price", "..."])]
    pub metrics: &'static [&'static str],
}
