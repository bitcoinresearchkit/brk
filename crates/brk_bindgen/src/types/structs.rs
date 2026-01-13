//! Structural pattern and field types.

use std::collections::BTreeSet;

use brk_types::Index;

use super::PatternMode;

/// A pattern of indexes that appear together on multiple metrics.
#[derive(Debug, Clone)]
pub struct IndexSetPattern {
    /// Pattern name (e.g., "DateHeightIndexes")
    pub name: String,
    /// The set of indexes
    pub indexes: BTreeSet<Index>,
}

/// A structural pattern - a branch structure that appears multiple times.
#[derive(Debug, Clone)]
pub struct StructuralPattern {
    /// Pattern name
    pub name: String,
    /// Ordered list of child fields
    pub fields: Vec<PatternField>,
    /// How fields construct metric names from acc (None = not parameterizable)
    pub mode: Option<PatternMode>,
    /// If true, all leaf fields use a type parameter T
    pub is_generic: bool,
}

impl StructuralPattern {
    /// Returns true if this pattern contains any leaf fields.
    pub fn contains_leaves(&self) -> bool {
        self.fields.iter().any(|f| f.is_leaf())
    }

    /// Returns true if this pattern can be parameterized with an accumulator.
    pub fn is_parameterizable(&self) -> bool {
        self.mode.is_some()
    }

    /// Get the field part (relative name or prefix) for a given field.
    pub fn get_field_part(&self, field_name: &str) -> Option<&str> {
        match &self.mode {
            Some(PatternMode::Suffix { relatives }) => relatives.get(field_name).map(|s| s.as_str()),
            Some(PatternMode::Prefix { prefixes }) => prefixes.get(field_name).map(|s| s.as_str()),
            None => None,
        }
    }

    /// Returns true if this pattern is in suffix mode.
    pub fn is_suffix_mode(&self) -> bool {
        matches!(&self.mode, Some(PatternMode::Suffix { .. }))
    }

    /// Returns true if this pattern is in prefix mode.
    pub fn is_prefix_mode(&self) -> bool {
        matches!(&self.mode, Some(PatternMode::Prefix { .. }))
    }
}

/// A field in a structural pattern.
#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct PatternField {
    /// Field name
    pub name: String,
    /// Rust type for leaves or pattern name for branches
    pub rust_type: String,
    /// JSON type from schema
    pub json_type: String,
    /// For leaves: the set of supported indexes. Empty for branches.
    pub indexes: BTreeSet<Index>,
    /// For branches referencing generic patterns: the concrete type parameter
    pub type_param: Option<String>,
}

impl PatternField {
    /// Returns true if this is a leaf field (has indexes).
    pub fn is_leaf(&self) -> bool {
        !self.indexes.is_empty()
    }

    /// Returns true if this is a branch field (no indexes).
    pub fn is_branch(&self) -> bool {
        self.indexes.is_empty()
    }
}

impl std::hash::Hash for PatternField {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.rust_type.hash(state);
        self.json_type.hash(state);
        self.indexes.hash(state);
    }
}

impl PartialEq for PatternField {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.rust_type == other.rust_type
            && self.json_type == other.json_type
            && self.indexes == other.indexes
    }
}

impl Eq for PatternField {}
