use std::collections::BTreeMap;

use rustc_hash::FxHashSet;
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
    #[serde(skip)]
    seen: FxHashSet<String>,
}

impl SeriesCount {
    pub fn add_endpoint(&mut self, name: &str, is_lazy: bool) {
        self.total_endpoints += 1;
        if is_lazy {
            self.lazy_endpoints += 1;
        } else {
            self.stored_endpoints += 1;
        }
        if self.seen.insert(name.to_string()) {
            self.distinct_series += 1;
        }
    }
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
