use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Metric count statistics - distinct metrics and total metric-index combinations
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MetricCount {
    /// Number of unique metrics available (e.g., realized_price, market_cap)
    #[schemars(example = 3141)]
    pub distinct_metrics: usize,
    /// Total number of metric-index combinations across all timeframes
    #[schemars(example = 21000)]
    pub total_endpoints: usize,
    /// Number of lazy (computed on-the-fly) metric-index combinations
    #[schemars(example = 5000)]
    pub lazy_endpoints: usize,
    /// Number of eager (stored on disk) metric-index combinations
    #[schemars(example = 16000)]
    pub stored_endpoints: usize,
}

impl MetricCount {
    pub fn add_endpoint(&mut self, is_lazy: bool) {
        self.total_endpoints += 1;
        if is_lazy {
            self.lazy_endpoints += 1;
        } else {
            self.stored_endpoints += 1;
        }
    }
}

/// Detailed metric count with per-database breakdown
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DetailedMetricCount {
    /// Aggregate counts
    #[serde(flatten)]
    pub total: MetricCount,
    /// Per-database breakdown of counts
    pub by_db: BTreeMap<String, MetricCount>,
}
