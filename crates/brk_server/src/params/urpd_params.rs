use schemars::JsonSchema;
use serde::Deserialize;

use brk_types::{Cohort, Date, UrpdAggregation};

/// Path parameters for `/api/urpd/{cohort}/{date}`.
#[derive(Deserialize, JsonSchema)]
pub struct UrpdParams {
    pub cohort: Cohort,
    #[schemars(with = "String", example = &"2024-01-01")]
    pub date: Date,
}

/// Path parameters for per-cohort URPD endpoints.
#[derive(Deserialize, JsonSchema)]
pub struct UrpdCohortParam {
    pub cohort: Cohort,
}

/// Query parameters for URPD endpoints.
#[derive(Deserialize, JsonSchema)]
pub struct UrpdQuery {
    /// Aggregation strategy. Default: raw (no aggregation). Accepts `bucket` as alias.
    #[serde(default, rename = "agg", alias = "bucket")]
    pub aggregation: UrpdAggregation,
}
