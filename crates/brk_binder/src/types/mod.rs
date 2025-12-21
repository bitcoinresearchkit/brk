//! Types and utilities for client generation.

mod case;
mod patterns;
mod schema;
mod tree;

pub use case::*;
pub use schema::*;
pub use tree::*;

use std::collections::{BTreeSet, HashMap};

use brk_query::Vecs;
use brk_types::Index;

/// How a field modifies the accumulated metric name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldNamePosition {
    /// Field prepends a prefix: leaf.name() = prefix + accumulated
    Prepend(String),
    /// Field appends a suffix: leaf.name() = accumulated + suffix
    Append(String),
    /// Field IS the accumulated name (no modification)
    Identity,
    /// Field sets a new base name (used at pattern entry points)
    SetBase(String),
}

/// Metadata extracted from brk_query for client generation.
#[derive(Debug)]
pub struct ClientMetadata {
    /// The catalog tree structure (with schemas in leaves)
    pub catalog: brk_types::TreeNode,
    /// Structural patterns - tree node shapes that repeat
    pub structural_patterns: Vec<StructuralPattern>,
    /// All indexes used across the catalog
    pub used_indexes: BTreeSet<Index>,
    /// Index set patterns - sets of indexes that appear together on metrics
    pub index_set_patterns: Vec<IndexSetPattern>,
    /// Maps concrete field signatures to pattern names
    pub concrete_to_pattern: HashMap<Vec<PatternField>, String>,
}

impl ClientMetadata {
    /// Extract metadata from brk_query::Vecs.
    pub fn from_vecs(vecs: &Vecs) -> Self {
        let catalog = vecs.catalog().clone();
        let (structural_patterns, concrete_to_pattern) =
            patterns::detect_structural_patterns(&catalog);
        let (used_indexes, index_set_patterns) = tree::detect_index_patterns(&catalog);

        ClientMetadata {
            catalog,
            structural_patterns,
            used_indexes,
            index_set_patterns,
            concrete_to_pattern,
        }
    }

    /// Find an index set pattern that matches the given indexes.
    pub fn find_index_set_pattern(&self, indexes: &BTreeSet<Index>) -> Option<&IndexSetPattern> {
        self.index_set_patterns
            .iter()
            .find(|p| &p.indexes == indexes)
    }

    /// Check if a type is a structural pattern name.
    pub fn is_pattern_type(&self, type_name: &str) -> bool {
        self.structural_patterns.iter().any(|p| p.name == type_name)
    }

    /// Find a pattern by name.
    pub fn find_pattern(&self, name: &str) -> Option<&StructuralPattern> {
        self.structural_patterns.iter().find(|p| p.name == name)
    }

    /// Check if a pattern is generic.
    pub fn is_pattern_generic(&self, name: &str) -> bool {
        self.find_pattern(name).is_some_and(|p| p.is_generic)
    }

    /// Extract the value type from concrete fields for a generic pattern.
    pub fn get_generic_value_type(
        &self,
        pattern_name: &str,
        fields: &[PatternField],
    ) -> Option<String> {
        if !self.is_pattern_generic(pattern_name) {
            return None;
        }
        fields
            .iter()
            .find(|f| f.is_leaf())
            .map(|f| extract_inner_type(&f.rust_type))
    }

    /// Build a lookup map from field signatures to pattern names.
    pub fn pattern_lookup(&self) -> HashMap<Vec<PatternField>, String> {
        let mut lookup = self.concrete_to_pattern.clone();
        for p in &self.structural_patterns {
            lookup.insert(p.fields.clone(), p.name.clone());
        }
        lookup
    }

    /// Check if a field should use a shared index accessor.
    pub fn field_uses_accessor(&self, field: &PatternField) -> bool {
        self.find_index_set_pattern(&field.indexes).is_some()
    }
}

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
    /// How each field modifies the accumulated name
    pub field_positions: HashMap<String, FieldNamePosition>,
    /// If true, all leaf fields use a type parameter T
    pub is_generic: bool,
}

impl StructuralPattern {
    /// Returns true if this pattern contains any leaf fields.
    pub fn contains_leaves(&self) -> bool {
        self.fields.iter().any(|f| f.is_leaf())
    }

    /// Returns true if all leaf fields have consistent name transformations.
    pub fn is_parameterizable(&self) -> bool {
        !self.field_positions.is_empty()
            && self
                .fields
                .iter()
                .all(|f| f.is_branch() || self.field_positions.contains_key(&f.name))
    }

    /// Get the field position for a given field name.
    pub fn get_field_position(&self, field_name: &str) -> Option<&FieldNamePosition> {
        self.field_positions.get(field_name)
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
    }
}

impl PartialEq for PatternField {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.rust_type == other.rust_type
            && self.json_type == other.json_type
    }
}

impl Eq for PatternField {}
