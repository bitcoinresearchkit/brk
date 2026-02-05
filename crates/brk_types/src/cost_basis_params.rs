use std::{fmt, ops::Deref};

use schemars::JsonSchema;
use serde::Deserialize;

use crate::{CostBasisBucket, CostBasisValue, Date};

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

/// Path parameters for cost basis distribution endpoint.
#[derive(Deserialize, JsonSchema)]
pub struct CostBasisParams {
    pub cohort: Cohort,
    #[schemars(with = "String", example = &"2024-01-01")]
    pub date: Date,
}

/// Path parameters for cost basis dates endpoint.
#[derive(Deserialize, JsonSchema)]
pub struct CostBasisCohortParam {
    pub cohort: Cohort,
}

/// Query parameters for cost basis distribution endpoint.
#[derive(Deserialize, JsonSchema)]
pub struct CostBasisQuery {
    /// Bucket type for aggregation. Default: raw (no aggregation).
    #[serde(default)]
    pub bucket: CostBasisBucket,
    /// Value type to return. Default: supply.
    #[serde(default)]
    pub value: CostBasisValue,
}
