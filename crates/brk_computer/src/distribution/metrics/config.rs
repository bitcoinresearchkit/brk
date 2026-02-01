use brk_cohort::{CohortContext, Filter, TimeFilter};
use brk_types::Version;
use vecdb::Database;

use crate::{indexes, price};

use super::RealizedMetrics;

/// Configuration for importing metrics.
pub struct ImportConfig<'a> {
    pub db: &'a Database,
    pub filter: Filter,
    pub full_name: &'a str,
    pub context: CohortContext,
    pub version: Version,
    pub indexes: &'a indexes::Vecs,
    pub price: Option<&'a price::Vecs>,
    /// Source for lazy adjusted computation: adjusted = cohort - up_to_1h.
    /// Required for cohorts where `compute_adjusted()` is true.
    pub up_to_1h_realized: Option<&'a RealizedMetrics>,
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

    /// Whether to compute relative metrics (invested capital %, NUPL ratios, etc.).
    pub fn compute_relative(&self) -> bool {
        self.filter.compute_relative()
    }

    /// Get full metric name with filter prefix.
    pub fn name(&self, suffix: &str) -> String {
        if self.full_name.is_empty() {
            suffix.to_string()
        } else if suffix.is_empty() {
            self.full_name.to_string()
        } else {
            format!("{}_{suffix}", self.full_name)
        }
    }

    /// Whether this cohort needs peak_regret metric.
    /// True for UTXO cohorts with age-based filters (all, term, time).
    /// age_range cohorts compute directly, others aggregate from age_range.
    pub fn compute_peak_regret(&self) -> bool {
        matches!(self.context, CohortContext::Utxo)
            && matches!(
                self.filter,
                Filter::All | Filter::Term(_) | Filter::Time(_)
            )
    }

    /// Whether this is an age_range cohort (UTXO context with Time::Range filter).
    /// These cohorts have peak_regret computed directly from chain_state.
    pub fn is_age_range(&self) -> bool {
        matches!(
            (&self.context, &self.filter),
            (CohortContext::Utxo, Filter::Time(TimeFilter::Range(_)))
        )
    }
}
