use schemars::JsonSchema;
use serde::Deserialize;

use brk_types::{Cohort, CostBasisBucket, CostBasisValue, Date};

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
