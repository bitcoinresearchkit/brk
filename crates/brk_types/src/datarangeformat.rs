use derive_more::Deref;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{DataRange, Format};

/// Data range with output format for API query parameters
#[derive(Default, Debug, Deref, Deserialize, JsonSchema)]
pub struct DataRangeFormat {
    #[deref]
    #[serde(flatten)]
    pub range: DataRange,

    /// Format of the output
    #[serde(default)]
    format: Format,
}

impl DataRangeFormat {
    pub fn format(&self) -> Format {
        self.format
    }

    pub fn set_from(mut self, from: i64) -> Self {
        self.range = self.range.set_from(from);
        self
    }

    pub fn set_to(mut self, to: i64) -> Self {
        self.range = self.range.set_to(to);
        self
    }

    pub fn set_count(mut self, count: usize) -> Self {
        self.range = self.range.set_count(count);
        self
    }
}

