use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Series count statistics - distinct series and total series-index combinations
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SeriesCount {
    /// Number of unique series available (e.g., realized_price, market_cap)
    #[schemars(example = 3141)]
    pub distinct_series: usize,
    /// Total number of series-index combinations across all timeframes
    #[schemars(example = 21000)]
    pub total_endpoints: usize,
    /// Number of lazy (computed on-the-fly) series-index combinations
    #[schemars(example = 5000)]
    pub lazy_endpoints: usize,
    /// Number of eager (stored on disk) series-index combinations
    #[schemars(example = 16000)]
    pub stored_endpoints: usize,
}

/// Detailed series count with per-database breakdown
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DetailedSeriesCount {
    /// Aggregate counts
    #[serde(flatten)]
    pub total: SeriesCount,
    /// Per-database breakdown of counts
    pub by_db: BTreeMap<String, SeriesCount>,
}
