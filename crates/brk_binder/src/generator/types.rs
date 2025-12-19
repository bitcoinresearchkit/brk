use std::collections::{BTreeMap, BTreeSet};

use brk_query::Vecs;
use brk_types::{Index, TreeNode};

/// Metadata extracted from brk_query for client generation
#[derive(Debug)]
pub struct ClientMetadata {
    /// All metrics with their available indexes and value type
    pub metrics: BTreeMap<String, MetricInfo>,
    /// The catalog tree structure (with schemas in leaves)
    pub catalog: TreeNode,
    /// Discovered patterns (sets of indexes that appear together frequently)
    pub patterns: Vec<IndexPattern>,
}

/// Information about a single metric
#[derive(Debug, Clone)]
pub struct MetricInfo {
    /// Metric name (e.g., "difficulty", "supply_total")
    pub name: String,
    /// Available indexes for this metric
    pub indexes: BTreeSet<Index>,
    /// Value type name (e.g., "Sats", "StoredF64")
    pub value_type: String,
}

/// A pattern of indexes that appears multiple times across metrics
#[derive(Debug, Clone)]
pub struct IndexPattern {
    /// Unique identifier for this pattern
    pub id: usize,
    /// The set of indexes in this pattern
    pub indexes: BTreeSet<Index>,
    /// How many metrics use this exact pattern
    pub usage_count: usize,
}

impl ClientMetadata {
    /// Extract metadata from brk_query::Vecs
    pub fn from_vecs(vecs: &Vecs) -> Self {
        let mut metrics = BTreeMap::new();
        let mut pattern_counts: BTreeMap<BTreeSet<Index>, usize> = BTreeMap::new();

        // Extract metric information
        for (name, index_to_vec) in &vecs.metric_to_index_to_vec {
            let indexes: BTreeSet<Index> = index_to_vec.keys().copied().collect();

            // Get value type from the first available vec
            let value_type = index_to_vec
                .values()
                .next()
                .map(|v| v.value_type_to_string().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            // Count pattern usage
            *pattern_counts.entry(indexes.clone()).or_insert(0) += 1;

            metrics.insert(
                name.to_string(),
                MetricInfo {
                    name: name.to_string(),
                    indexes,
                    value_type,
                },
            );
        }

        // Extract patterns that are used by multiple metrics
        let mut patterns: Vec<IndexPattern> = pattern_counts
            .into_iter()
            .filter(|(_, count)| *count >= 2) // Only patterns used by 2+ metrics
            .enumerate()
            .map(|(id, (indexes, usage_count))| IndexPattern {
                id,
                indexes,
                usage_count,
            })
            .collect();

        // Sort by usage count descending
        patterns.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));

        ClientMetadata {
            metrics,
            catalog: vecs.catalog().clone(),
            patterns,
        }
    }

    /// Find the pattern that matches a metric's indexes, if any
    pub fn find_pattern_for_metric(&self, metric: &MetricInfo) -> Option<&IndexPattern> {
        self.patterns
            .iter()
            .find(|p| p.indexes == metric.indexes)
    }
}

/// Convert a metric name to PascalCase (for struct/class names)
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Convert a metric name to snake_case (already snake_case, but sanitize)
pub fn to_snake_case(s: &str) -> String {
    let sanitized = s.replace('-', "_");
    // Handle Rust keywords
    match sanitized.as_str() {
        "type" | "const" | "static" | "match" | "if" | "else" | "loop" | "while" | "for"
        | "break" | "continue" | "return" | "fn" | "let" | "mut" | "ref" | "self" | "super"
        | "mod" | "use" | "pub" | "crate" | "extern" | "impl" | "trait" | "struct" | "enum"
        | "where" | "async" | "await" | "dyn" | "move" => format!("r#{}", sanitized),
        _ => sanitized,
    }
}

/// Convert a metric name to camelCase (for JS/TS)
pub fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert a serde_json::Value (JSON Schema) to a JSDoc type annotation
pub fn schema_to_jsdoc(schema: &serde_json::Value) -> String {
    if let Some(ty) = schema.get("type").and_then(|v| v.as_str()) {
        match ty {
            "null" => "null".to_string(),
            "boolean" => "boolean".to_string(),
            "integer" | "number" => "number".to_string(),
            "string" => "string".to_string(),
            "array" => {
                if let Some(items) = schema.get("items") {
                    format!("{}[]", schema_to_jsdoc(items))
                } else {
                    "Array<*>".to_string()
                }
            }
            "object" => "Object".to_string(),
            _ => "*".to_string(),
        }
    } else if schema.get("anyOf").is_some() || schema.get("oneOf").is_some() {
        let variants = schema
            .get("anyOf")
            .or_else(|| schema.get("oneOf"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(schema_to_jsdoc)
                    .collect::<Vec<_>>()
                    .join("|")
            })
            .unwrap_or_else(|| "*".to_string());
        format!("({})", variants)
    } else if let Some(reference) = schema.get("$ref").and_then(|v| v.as_str()) {
        reference.rsplit('/').next().unwrap_or("*").to_string()
    } else {
        "*".to_string()
    }
}
