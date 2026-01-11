//! Client metadata extracted from brk_query.

use std::collections::{BTreeSet, HashMap};

use brk_query::Vecs;
use brk_types::Index;

use super::{GenericSyntax, IndexSetPattern, PatternField, StructuralPattern, extract_inner_type};
use crate::analysis;

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
    concrete_to_pattern: HashMap<Vec<PatternField>, String>,
    /// Maps concrete field signatures to their type parameter (for generic patterns)
    concrete_to_type_param: HashMap<Vec<PatternField>, String>,
}

impl ClientMetadata {
    /// Extract metadata from brk_query::Vecs.
    pub fn from_vecs(vecs: &Vecs) -> Self {
        let catalog = vecs.catalog().clone();
        let (structural_patterns, concrete_to_pattern, concrete_to_type_param) =
            analysis::detect_structural_patterns(&catalog);
        let (used_indexes, index_set_patterns) = analysis::detect_index_patterns(&catalog);

        ClientMetadata {
            catalog,
            structural_patterns,
            used_indexes,
            index_set_patterns,
            concrete_to_pattern,
            concrete_to_type_param,
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

    /// Check if a pattern by name is parameterizable.
    pub fn is_parameterizable(&self, name: &str) -> bool {
        self.find_pattern(name).is_some_and(|p| p.is_parameterizable())
    }

    /// Check if child fields match a parameterizable pattern.
    /// Returns true only if the fields match a pattern AND that pattern is parameterizable.
    pub fn is_parameterizable_fields(&self, fields: &[PatternField]) -> bool {
        self.concrete_to_pattern
            .get(fields)
            .or_else(|| {
                self.structural_patterns
                    .iter()
                    .find(|p| p.fields == fields)
                    .map(|p| &p.name)
            })
            .is_some_and(|name| self.is_parameterizable(name))
    }

    /// Resolve the type name for a tree field, considering parameterizability.
    /// If the field matches a parameterizable pattern, returns type annotation from callback.
    /// Otherwise returns the inline type name (parent_child format).
    pub fn resolve_tree_field_type<F>(
        &self,
        child_fields: Option<&[PatternField]>,
        parent_name: &str,
        child_name: &str,
        type_annotation_fn: F,
    ) -> String
    where
        F: FnOnce(Option<&str>) -> String,
    {
        match child_fields {
            Some(cf) if self.is_parameterizable_fields(cf) => {
                let generic_value_type = self.get_type_param(cf).map(String::as_str);
                type_annotation_fn(generic_value_type)
            }
            Some(_) => crate::child_type_name(parent_name, child_name),
            None => type_annotation_fn(None),
        }
    }

    /// Get the type parameter for a generic pattern given its concrete fields.
    pub fn get_type_param(&self, fields: &[PatternField]) -> Option<&String> {
        self.concrete_to_type_param.get(fields)
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

    /// Generate type annotation for a field with language-specific syntax.
    pub fn field_type_annotation(
        &self,
        field: &PatternField,
        is_generic: bool,
        generic_value_type: Option<&str>,
        syntax: GenericSyntax,
    ) -> String {
        let value_type = if is_generic && field.rust_type == "T" {
            "T".to_string()
        } else {
            extract_inner_type(&field.rust_type)
        };

        if self.is_pattern_type(&field.rust_type) {
            if self.is_pattern_generic(&field.rust_type) {
                let type_param = field
                    .type_param
                    .as_deref()
                    .or(generic_value_type)
                    .unwrap_or(if is_generic { "T" } else { syntax.default_type });
                return syntax.wrap(&field.rust_type, type_param);
            }
            field.rust_type.clone()
        } else if field.is_branch() {
            field.rust_type.clone()
        } else if let Some(accessor) = self.find_index_set_pattern(&field.indexes) {
            syntax.wrap(&accessor.name, &value_type)
        } else {
            syntax.wrap("MetricNode", &value_type)
        }
    }
}
