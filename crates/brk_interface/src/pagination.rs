use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Index, deser::de_unquote_usize};

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
