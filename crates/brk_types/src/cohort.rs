use std::{fmt, ops::Deref};

use schemars::JsonSchema;
use serde::Deserialize;

/// Cohort identifier for cost basis distribution.
#[derive(Deserialize, JsonSchema)]
#[schemars(example = &"all", example = &"sth", example = &"lth")]
pub struct Cohort(String);

impl fmt::Display for Cohort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<T: Into<String>> From<T> for Cohort {
    fn from(s: T) -> Self {
        Self(s.into())
    }
}

impl Deref for Cohort {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
