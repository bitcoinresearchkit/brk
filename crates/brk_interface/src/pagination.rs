use schemars::JsonSchema;
use serde::Deserialize;

use crate::Index;

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct PaginationParam {
    #[serde(alias = "p")]
    #[schemars(description = "Pagination index")]
    #[serde(default)]
    pub page: usize,
}

impl PaginationParam {
    const PER_PAGE: usize = 1_000;

    pub fn start(&self, len: usize) -> usize {
        (self.page * Self::PER_PAGE).clamp(0, len)
    }

    pub fn end(&self, len: usize) -> usize {
        ((self.page + 1) * Self::PER_PAGE).clamp(0, len)
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PaginatedIndexParam {
    pub index: Index,
    #[serde(flatten)]
    pub pagination: PaginationParam,
}
