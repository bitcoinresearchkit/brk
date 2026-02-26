use brk_cohort::Filter;
use brk_types::Version;
use vecdb::Database;

use crate::indexes;

/// Configuration for importing metrics.
pub struct ImportConfig<'a> {
    pub db: &'a Database,
    pub filter: Filter,
    pub full_name: &'a str,
    pub version: Version,
    pub indexes: &'a indexes::Vecs,
}

impl<'a> ImportConfig<'a> {
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
