use brk_cohort::{CohortContext, Filter};
use brk_types::Version;
use vecdb::Database;

use crate::{indexes, price};

/// Configuration for importing metrics.
pub struct ImportConfig<'a> {
    pub db: &'a Database,
    pub filter: Filter,
    pub full_name: &'a str,
    pub context: CohortContext,
    pub version: Version,
    pub indexes: &'a indexes::Vecs,
    pub price: Option<&'a price::Vecs>,
}

impl<'a> ImportConfig<'a> {
    /// Whether price data is available (enables realized/unrealized metrics).
    pub fn compute_dollars(&self) -> bool {
        self.price.is_some()
    }

    /// Whether this is an extended cohort (more relative metrics).
    pub fn extended(&self) -> bool {
        self.filter.is_extended(self.context)
    }

    /// Whether to compute relative-to-all metrics.
    pub fn compute_rel_to_all(&self) -> bool {
        self.filter.compute_rel_to_all()
    }

    /// Whether to compute adjusted metrics (SOPR, etc.).
    pub fn compute_adjusted(&self) -> bool {
        self.filter.compute_adjusted(self.context)
    }

    /// Get full metric name with filter prefix.
    pub fn name(&self, suffix: &str) -> String {
        if self.full_name.is_empty() {
            suffix.to_string()
        } else {
            format!("{}_{suffix}", self.full_name)
        }
    }
}
