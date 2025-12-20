use std::collections::{BTreeSet, HashMap};
use std::hash::{Hash, Hasher};

use brk_query::Vecs;
use brk_types::{Index, TreeNode};

/// Metadata extracted from brk_query for client generation
#[derive(Debug)]
pub struct ClientMetadata {
    /// The catalog tree structure (with schemas in leaves)
    pub catalog: TreeNode,
    /// Structural patterns - tree node shapes that repeat
    pub structural_patterns: Vec<StructuralPattern>,
    /// All indexes used across the catalog
    pub used_indexes: BTreeSet<Index>,
    /// Index set patterns - sets of indexes that appear together on metrics
    pub index_set_patterns: Vec<IndexSetPattern>,
}

/// A pattern of indexes that appear together on multiple metrics
#[derive(Debug, Clone)]
pub struct IndexSetPattern {
    /// Pattern name (e.g., "DateHeightIndexes")
    pub name: String,
    /// The set of indexes
    pub indexes: BTreeSet<Index>,
}

/// A structural pattern - a branch structure that appears multiple times in the tree
#[derive(Debug, Clone)]
pub struct StructuralPattern {
    /// Pattern name - sanitized for all languages (e.g., "BaseCumulativeSum")
    pub name: String,
    /// Ordered list of child fields (sorted by field name)
    pub fields: Vec<PatternField>,
}

impl StructuralPattern {
    /// Returns true if this pattern contains any leaf fields (fields with indexes).
    /// Patterns with leaves can't use factory functions because leaf.name() is instance-specific.
    pub fn contains_leaves(&self) -> bool {
        self.fields.iter().any(|f| !f.indexes.is_empty())
    }
}

/// A field in a structural pattern
#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct PatternField {
    /// Field name
    pub name: String,
    /// Rust type: brk_types type for leaves ("Sats", "StoredF64") or pattern name for branches
    pub rust_type: String,
    /// JSON type from schema: "integer", "number", "string", "boolean", or pattern name for branches
    pub json_type: String,
    /// For leaves: the set of supported indexes. Empty for branches.
    pub indexes: BTreeSet<Index>,
}

// Manual implementations of Hash/Eq/PartialEq that exclude `indexes`
// since indexes aren't part of the structural pattern identity
impl Hash for PatternField {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.rust_type.hash(state);
        self.json_type.hash(state);
        // indexes excluded from hash
    }
}

impl PartialEq for PatternField {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.rust_type == other.rust_type
            && self.json_type == other.json_type
        // indexes excluded from equality
    }
}

impl Eq for PatternField {}

impl ClientMetadata {
    /// Extract metadata from brk_query::Vecs
    pub fn from_vecs(vecs: &Vecs) -> Self {
        let catalog = vecs.catalog().clone();
        let structural_patterns = detect_structural_patterns(&catalog);
        let (used_indexes, index_set_patterns) = detect_index_patterns(&catalog);

        ClientMetadata {
            catalog,
            structural_patterns,
            used_indexes,
            index_set_patterns,
        }
    }

    /// Check if an index set matches a pattern
    pub fn find_index_set_pattern(&self, indexes: &BTreeSet<Index>) -> Option<&IndexSetPattern> {
        self.index_set_patterns
            .iter()
            .find(|p| &p.indexes == indexes)
    }

    /// Check if a type is a pattern (vs a primitive leaf type)
    pub fn is_pattern_type(&self, type_name: &str) -> bool {
        self.structural_patterns.iter().any(|p| p.name == type_name)
    }

    /// Build a lookup map from field signatures to pattern names
    pub fn pattern_lookup(&self) -> HashMap<Vec<PatternField>, String> {
        self.structural_patterns
            .iter()
            .map(|p| (p.fields.clone(), p.name.clone()))
            .collect()
    }
}

/// Detect structural patterns in the tree using a bottom-up approach.
/// For every branch node, create a signature from its children (sorted field names + types).
/// Patterns that appear 2+ times are deduplicated.
fn detect_structural_patterns(tree: &TreeNode) -> Vec<StructuralPattern> {
    // Map from sorted fields signature to pattern name
    let mut signature_to_pattern: HashMap<Vec<PatternField>, String> = HashMap::new();
    // Count how many times each signature appears
    let mut signature_counts: HashMap<Vec<PatternField>, usize> = HashMap::new();

    // Process tree bottom-up to resolve all branch types
    resolve_branch_patterns(tree, &mut signature_to_pattern, &mut signature_counts);

    // Build final list of patterns (only those appearing 2+ times)
    let mut patterns: Vec<StructuralPattern> = signature_to_pattern
        .into_iter()
        .filter(|(sig, _)| signature_counts.get(sig).copied().unwrap_or(0) >= 2)
        .map(|(fields, name)| StructuralPattern { name, fields })
        .collect();

    // Sort by number of fields descending (larger patterns first)
    patterns.sort_by(|a, b| b.fields.len().cmp(&a.fields.len()));

    patterns
}

/// Recursively resolve branch patterns bottom-up.
/// Returns the pattern name for this node if it's a branch, or None if it's a leaf.
fn resolve_branch_patterns(
    node: &TreeNode,
    signature_to_pattern: &mut HashMap<Vec<PatternField>, String>,
    signature_counts: &mut HashMap<Vec<PatternField>, usize>,
) -> Option<String> {
    match node {
        TreeNode::Leaf(_) => {
            // Leaves don't have patterns, return None
            None
        }
        TreeNode::Branch(children) => {
            // First, recursively resolve all children
            let mut fields: Vec<PatternField> = Vec::new();

            for (child_name, child_node) in children {
                let (rust_type, json_type, indexes) = match child_node {
                    TreeNode::Leaf(leaf) => (
                        leaf.value_type().to_string(),
                        schema_to_json_type(&leaf.schema),
                        leaf.indexes().clone(),
                    ),
                    TreeNode::Branch(_) => {
                        // Branch: recursively get its pattern name
                        let pattern_name = resolve_branch_patterns(
                            child_node,
                            signature_to_pattern,
                            signature_counts,
                        )
                        .unwrap_or_else(|| "Unknown".to_string());
                        (pattern_name.clone(), pattern_name, BTreeSet::new())
                    }
                };

                fields.push(PatternField {
                    name: child_name.clone(),
                    rust_type,
                    json_type,
                    indexes,
                });
            }

            // Sort fields by name for consistent signatures
            fields.sort_by(|a, b| a.name.cmp(&b.name));

            // Increment count for this signature
            *signature_counts.entry(fields.clone()).or_insert(0) += 1;

            // Get or create pattern name for this signature
            let pattern_name = signature_to_pattern
                .entry(fields.clone())
                .or_insert_with(|| generate_pattern_name_from_fields(&fields))
                .clone();

            Some(pattern_name)
        }
    }
}

/// Generate a sanitized pattern name from fields.
/// Names must be valid identifiers in all target languages (Rust, JS, Python).
fn generate_pattern_name_from_fields(fields: &[PatternField]) -> String {
    // Join field names with underscores, then convert to PascalCase
    let joined: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
    let raw_name = joined.join("_");

    // Sanitize: ensure it starts with a letter (prepend "P_" if starts with digit)
    let sanitized = if raw_name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("P_{}", raw_name)
    } else {
        raw_name
    };

    to_pascal_case(&sanitized)
}

/// Extract JSON type from JSON Schema
fn schema_to_json_type(schema: &serde_json::Value) -> String {
    if let Some(ty) = schema.get("type").and_then(|v| v.as_str()) {
        ty.to_string()
    } else {
        "object".to_string()
    }
}

/// Get the field signature for a branch node's children
pub fn get_node_fields(
    children: &std::collections::BTreeMap<String, TreeNode>,
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
) -> Vec<PatternField> {
    let mut fields: Vec<PatternField> = children
        .iter()
        .map(|(name, node)| {
            let (rust_type, json_type, indexes) = match node {
                TreeNode::Leaf(leaf) => (
                    leaf.value_type().to_string(),
                    schema_to_json_type(&leaf.schema),
                    leaf.indexes().clone(),
                ),
                TreeNode::Branch(grandchildren) => {
                    let child_fields = get_node_fields(grandchildren, pattern_lookup);
                    let pattern_name = pattern_lookup
                        .get(&child_fields)
                        .cloned()
                        .unwrap_or_else(|| "Unknown".to_string());
                    (pattern_name.clone(), pattern_name, BTreeSet::new())
                }
            };
            PatternField {
                name: name.clone(),
                rust_type,
                json_type,
                indexes,
            }
        })
        .collect();
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    fields
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

/// Detect index patterns - collect all indexes and find sets that appear 2+ times
fn detect_index_patterns(tree: &TreeNode) -> (BTreeSet<Index>, Vec<IndexSetPattern>) {
    let mut used_indexes: BTreeSet<Index> = BTreeSet::new();
    let mut index_sets: Vec<BTreeSet<Index>> = Vec::new();

    // Traverse tree and collect index information from leaves
    collect_indexes_from_tree(tree, &mut used_indexes, &mut index_sets);

    // Count occurrences of each unique index set
    let mut index_set_counts: Vec<(BTreeSet<Index>, usize)> = Vec::new();
    for index_set in index_sets {
        if let Some(entry) = index_set_counts.iter_mut().find(|(s, _)| s == &index_set) {
            entry.1 += 1;
        } else {
            index_set_counts.push((index_set, 1));
        }
    }

    // Build patterns for index sets appearing 2+ times
    let mut patterns: Vec<IndexSetPattern> = index_set_counts
        .into_iter()
        .filter(|(indexes, count)| *count >= 2 && !indexes.is_empty())
        .map(|(indexes, _)| IndexSetPattern {
            name: generate_index_set_name(&indexes),
            indexes,
        })
        .collect();

    // Sort by number of indexes descending
    patterns.sort_by(|a, b| b.indexes.len().cmp(&a.indexes.len()));

    (used_indexes, patterns)
}

/// Recursively collect indexes from tree leaves
fn collect_indexes_from_tree(
    node: &TreeNode,
    used_indexes: &mut BTreeSet<Index>,
    index_sets: &mut Vec<BTreeSet<Index>>,
) {
    match node {
        TreeNode::Leaf(leaf) => {
            // Add all indexes from this leaf to the global set
            used_indexes.extend(leaf.indexes().iter().cloned());
            // Collect this index set
            index_sets.push(leaf.indexes().clone());
        }
        TreeNode::Branch(children) => {
            for child in children.values() {
                collect_indexes_from_tree(child, used_indexes, index_sets);
            }
        }
    }
}

/// Generate a name for an index set pattern
fn generate_index_set_name(indexes: &BTreeSet<Index>) -> String {
    if indexes.len() == 1 {
        let index = indexes.iter().next().unwrap();
        return format!("{}Accessor", to_pascal_case(index.serialize_long()));
    }

    // For multiple indexes, create a descriptive name
    let names: Vec<&str> = indexes.iter().map(|i| i.serialize_long()).collect();
    format!("{}Accessor", to_pascal_case(&names.join("_")))
}
