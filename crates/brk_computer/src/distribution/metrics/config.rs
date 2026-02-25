use brk_cohort::{CohortContext, Filter};
use brk_types::Version;
use vecdb::Database;

use crate::{indexes, prices};

/// Configuration for importing metrics.
pub struct ImportConfig<'a> {
    pub db: &'a Database,
    pub filter: Filter,
    pub full_name: &'a str,
    pub context: CohortContext,
    pub version: Version,
    pub indexes: &'a indexes::Vecs,
    pub prices: &'a prices::Vecs,
}

impl<'a> ImportConfig<'a> {
    /// Whether this is an extended cohort (more relative metrics).
    pub(crate) fn extended(&self) -> bool {
        self.filter.is_extended(self.context)
    }

    /// Get full metric name with filter prefix.
    pub(crate) fn name(&self, suffix: &str) -> String {
        if self.full_name.is_empty() {
            suffix.to_string()
        } else if suffix.is_empty() {
            self.full_name.to_string()
        } else {
            format!("{}_{suffix}", self.full_name)
        }
    }

}
