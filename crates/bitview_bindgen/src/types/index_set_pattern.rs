use std::collections::BTreeSet;

use brk_types::Index;

/// A pattern of indexes that appear together on multiple series.
#[derive(Debug, Clone)]
pub struct IndexSetPattern {
    /// Pattern name (e.g., "DateHeightIndexes")
    pub name: String,
    /// The set of indexes
    pub indexes: BTreeSet<Index>,
}
