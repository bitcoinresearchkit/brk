use schemars::JsonSchema;
use serde::Serialize;

/// Metric count statistics - distinct metrics and total metric-index combinations
#[derive(Debug, Serialize, JsonSchema)]
pub struct MetricCount {
    /// Number of unique metrics available (e.g., realized_price, market_cap)
    #[schemars(example = 3141)]
    pub distinct_metrics: usize,
    /// Total number of metric-index combinations across all timeframes
    #[schemars(example = 21000)]
    pub total_endpoints: usize,
}
