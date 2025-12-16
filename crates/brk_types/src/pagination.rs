use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Pagination {
    /// Pagination index
    #[serde(default, alias = "p")]
    #[schemars(example = 0, example = 1, example = 2)]
    pub page: Option<usize>,
}

impl Pagination {
    pub const PER_PAGE: usize = 1_000;

    pub fn page(&self) -> usize {
        self.page.unwrap_or_default()
    }

    pub fn start(&self, len: usize) -> usize {
        (self.page() * Self::PER_PAGE).clamp(0, len)
    }

    pub fn end(&self, len: usize) -> usize {
        ((self.page() + 1) * Self::PER_PAGE).clamp(0, len)
    }
}
