use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
/// Metric count statistics - distinct metrics and total metric-index combinations
pub struct MetricCount {
    #[schemars(example = 3141)]
    /// Number of unique metrics available (e.g., realized_price, market_cap)
    pub distinct_metrics: usize,
    #[schemars(example = 21000)]
    /// Total number of metric-index combinations across all timeframes
    pub total_endpoints: usize,
}
