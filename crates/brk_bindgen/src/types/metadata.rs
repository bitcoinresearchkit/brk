//! Client metadata extracted from brk_query.

use std::collections::{BTreeSet, HashMap};

use brk_query::Vecs;
use brk_types::{Index, MetricLeafWithSchema};

use super::{GenericSyntax, IndexSetPattern, PatternField, StructuralPattern, extract_inner_type};
use crate::analysis;

/// Metadata extracted from brk_query for client generation.
#[derive(Debug)]
pub struct ClientMetadata {
    /// The catalog tree structure (with schemas in leaves)
    pub catalog: brk_types::TreeNode,
    /// Structural patterns - tree node shapes that repeat
    pub structural_patterns: Vec<StructuralPattern>,
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
        Self::from_catalog(vecs.catalog().clone())
    }

    /// Extract metadata from a catalog TreeNode directly.
    pub fn from_catalog(catalog: brk_types::TreeNode) -> Self {
        let (structural_patterns, concrete_to_pattern, concrete_to_type_param) =
            analysis::detect_structural_patterns(&catalog);
        let index_set_patterns = analysis::detect_index_patterns(&catalog);

        ClientMetadata {
            catalog,
            structural_patterns,
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

    /// Check if a pattern by name is fully parameterizable.
    /// A pattern is parameterizable if it has a mode AND all its branch fields
    /// are also parameterizable (or not patterns at all).
    pub fn is_parameterizable(&self, name: &str) -> bool {
        self.find_pattern(name).is_some_and(|p| {
            if !p.is_parameterizable() {
                return false;
            }
            // Check all branch fields have parameterizable types (or are not patterns)
            p.fields.iter().all(|f| {
                if f.is_branch() {
                    self.structural_patterns
                        .iter()
                        .find(|pat| pat.name == f.rust_type)
                        .is_none_or(|pat| pat.is_parameterizable())
                } else {
                    true
                }
            })
        })
    }

    /// Check if child fields match ANY pattern (parameterizable or not).
    /// Used for type annotations - we want to reuse pattern types for all patterns.
    pub fn matches_pattern(&self, fields: &[PatternField]) -> bool {
        self.concrete_to_pattern.contains_key(fields)
            || self.structural_patterns.iter().any(|p| p.fields == fields)
    }

    /// Resolve the type name for a tree field.
    /// If the field matches ANY pattern (parameterizable or not), returns pattern type.
    /// Otherwise returns the inline type name (parent_child format).
    pub fn resolve_tree_field_type(
        &self,
        field: &PatternField,
        child_fields: Option<&[PatternField]>,
        parent_name: &str,
        child_name: &str,
        syntax: GenericSyntax,
    ) -> String {
        match child_fields {
            // Use pattern type for ANY matching pattern (parameterizable or not)
            Some(cf) if self.matches_pattern(cf) => {
                let generic_value_type = self.get_type_param(cf).map(String::as_str);
                self.field_type_annotation(field, false, generic_value_type, syntax)
            }
            Some(_) => crate::child_type_name(parent_name, child_name),
            None => self.field_type_annotation(field, false, None, syntax),
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

    /// Generate type annotation for a leaf node with language-specific syntax.
    ///
    /// This is a simpler version of `field_type_annotation` that works directly
    /// with a `MetricLeafWithSchema` node instead of a `PatternField`.
    pub fn field_type_annotation_from_leaf(
        &self,
        leaf: &MetricLeafWithSchema,
        syntax: GenericSyntax,
    ) -> String {
        let value_type = leaf.kind().to_string();
        if let Some(accessor) = self.find_index_set_pattern(leaf.indexes()) {
            syntax.wrap(&accessor.name, &value_type)
        } else {
            syntax.wrap("MetricNode", &value_type)
        }
    }
}
