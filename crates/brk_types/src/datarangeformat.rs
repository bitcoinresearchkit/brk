use std::ops::Deref;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::{DataRange, Format};

/// Data range with output format for API query parameters
#[derive(Default, Debug, Deserialize, JsonSchema)]
pub struct DataRangeFormat {
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

impl Deref for DataRangeFormat {
    type Target = DataRange;
    fn deref(&self) -> &Self::Target {
        &self.range
    }
}
