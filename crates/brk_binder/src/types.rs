use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::hash::{Hash, Hasher};

use brk_query::Vecs;
use brk_types::{Index, TreeNode};

/// How a field modifies the accumulated metric name
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
    /// Maps concrete field signatures to pattern names (includes generic pattern mappings)
    pub concrete_to_pattern: HashMap<Vec<PatternField>, String>,
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
    /// How each field modifies the accumulated name (field_name -> position)
    pub field_positions: HashMap<String, FieldNamePosition>,
    /// If true, all leaf fields use a type parameter T instead of concrete types
    pub is_generic: bool,
}

impl StructuralPattern {
    /// Returns true if this pattern contains any leaf fields (fields with indexes).
    /// Patterns with leaves can't use factory functions because leaf.name() is instance-specific.
    pub fn contains_leaves(&self) -> bool {
        self.fields.iter().any(|f| !f.indexes.is_empty())
    }

    /// Returns true if all leaf fields have consistent name transformations.
    /// A pattern is parameterizable if we can detect prepend/append patterns.
    pub fn is_parameterizable(&self) -> bool {
        !self.field_positions.is_empty()
            && self.fields.iter().all(|f| {
                // Branch fields are always OK (they delegate to nested patterns)
                f.indexes.is_empty() || self.field_positions.contains_key(&f.name)
            })
    }

    /// Get the field position for a given field name
    pub fn get_field_position(&self, field_name: &str) -> Option<&FieldNamePosition> {
        self.field_positions.get(field_name)
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
        let (structural_patterns, concrete_to_pattern) = detect_structural_patterns(&catalog);
        let (used_indexes, index_set_patterns) = detect_index_patterns(&catalog);

        ClientMetadata {
            catalog,
            structural_patterns,
            used_indexes,
            index_set_patterns,
            concrete_to_pattern,
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

    /// Find a pattern by name
    pub fn find_pattern(&self, name: &str) -> Option<&StructuralPattern> {
        self.structural_patterns.iter().find(|p| p.name == name)
    }

    /// Check if a pattern is generic
    pub fn is_pattern_generic(&self, name: &str) -> bool {
        self.find_pattern(name)
            .map(|p| p.is_generic)
            .unwrap_or(false)
    }

    /// Extract the value type from concrete fields for a generic pattern.
    /// Returns the first leaf field's rust_type if this pattern is generic.
    /// If the type is a wrapper like `Close<Dollars>`, extracts the inner type `Dollars`.
    pub fn get_generic_value_type(
        &self,
        pattern_name: &str,
        fields: &[PatternField],
    ) -> Option<String> {
        if !self.is_pattern_generic(pattern_name) {
            return None;
        }
        // Find first leaf field (has indexes)
        fields
            .iter()
            .find(|f| !f.indexes.is_empty())
            .map(|f| extract_inner_type(&f.rust_type))
    }

    /// Build a lookup map from field signatures to pattern names.
    /// Includes both generic pattern signatures and concrete signatures.
    pub fn pattern_lookup(&self) -> HashMap<Vec<PatternField>, String> {
        // Start with concrete-to-pattern mappings (includes generic pattern concrete signatures)
        let mut lookup = self.concrete_to_pattern.clone();
        // Also add the normalized generic signatures
        for p in &self.structural_patterns {
            lookup.insert(p.fields.clone(), p.name.clone());
        }
        lookup
    }
}

use serde_json::Value;

/// Unwrap allOf with a single element, returning the inner schema.
/// schemars uses allOf for composition, but often with just one $ref.
pub fn unwrap_allof(schema: &Value) -> &Value {
    if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array())
        && all_of.len() == 1
    {
        return &all_of[0];
    }
    schema
}

/// Extract inner type from a wrapper generic like `Close<Dollars>` -> `Dollars`.
/// Also handles malformed types like `Dollars>` (from vecdb's short_type_name which
/// extracts "Dollars>" from "Close<brk_types::Dollars>" using rsplit("::")).
/// If not a generic, returns the type as-is.
pub fn extract_inner_type(type_str: &str) -> String {
    // Handle proper generic wrappers like `Close<Dollars>` -> `Dollars`
    if let Some(start) = type_str.find('<') {
        if let Some(end) = type_str.rfind('>') {
            if start < end {
                return type_str[start + 1..end].to_string();
            }
        }
    }
    // Handle malformed types like `Dollars>` (trailing > without <)
    // This happens due to vecdb's short_type_name using rsplit("::")
    if type_str.ends_with('>') && !type_str.contains('<') {
        return type_str.trim_end_matches('>').to_string();
    }
    type_str.to_string()
}

/// Detect structural patterns in the tree using a bottom-up approach.
/// For every branch node, create a signature from its children (sorted field names + types).
/// Patterns that appear 2+ times are deduplicated.
/// Returns (patterns, concrete_to_pattern_mapping).
fn detect_structural_patterns(
    tree: &TreeNode,
) -> (Vec<StructuralPattern>, HashMap<Vec<PatternField>, String>) {
    // Map from sorted fields signature to pattern name
    let mut signature_to_pattern: HashMap<Vec<PatternField>, String> = HashMap::new();
    // Count how many times each signature appears
    let mut signature_counts: HashMap<Vec<PatternField>, usize> = HashMap::new();
    // Map normalized signatures to names (so patterns differing only in value type share names)
    let mut normalized_to_name: HashMap<Vec<PatternField>, String> = HashMap::new();
    // Track name usage to append index for duplicates
    let mut name_counts: HashMap<String, usize> = HashMap::new();

    // Process tree bottom-up to resolve all branch types
    resolve_branch_patterns(
        tree,
        "root",
        &mut signature_to_pattern,
        &mut signature_counts,
        &mut normalized_to_name,
        &mut name_counts,
    );

    // First, identify generic patterns by grouping ALL signatures by their normalized form.
    // Even if each concrete signature appears only once, if 2+ different value types
    // normalize to the same pattern, we create a generic pattern.
    let (generic_patterns, generic_mappings) = detect_generic_patterns(&signature_to_pattern);

    // Build non-generic patterns: signatures appearing 2+ times that weren't merged into generics
    let mut patterns: Vec<StructuralPattern> = signature_to_pattern
        .iter()
        .filter(|(sig, _)| {
            signature_counts.get(*sig).copied().unwrap_or(0) >= 2
                && !generic_mappings.contains_key(*sig)
        })
        .map(|(fields, name)| StructuralPattern {
            name: name.clone(),
            fields: fields.clone(),
            field_positions: HashMap::new(),
            is_generic: false,
        })
        .collect();

    // Add the generic patterns
    patterns.extend(generic_patterns);

    // Build lookup for second pass - include all concrete signatures
    let mut pattern_lookup: HashMap<Vec<PatternField>, String> = HashMap::new();
    // Add non-generic patterns that appear 2+ times
    for (sig, name) in &signature_to_pattern {
        if signature_counts.get(sig).copied().unwrap_or(0) >= 2 {
            pattern_lookup.insert(sig.clone(), name.clone());
        }
    }
    // Add generic mappings (overwrite if there's overlap)
    pattern_lookup.extend(generic_mappings.clone());

    // Build the concrete_to_pattern map to return
    let concrete_to_pattern = pattern_lookup.clone();

    // Second pass: analyze field positions by traversing tree instances
    analyze_pattern_field_positions(tree, &mut patterns, &pattern_lookup);

    // Sort by number of fields descending (larger patterns first)
    patterns.sort_by(|a, b| b.fields.len().cmp(&a.fields.len()));

    (patterns, concrete_to_pattern)
}

/// Detect generic patterns by grouping all signatures by their normalized form.
/// Returns (generic_patterns, concrete_signature -> generic_pattern_name mapping).
fn detect_generic_patterns(
    signature_to_pattern: &HashMap<Vec<PatternField>, String>,
) -> (Vec<StructuralPattern>, HashMap<Vec<PatternField>, String>) {
    // Group signatures by their normalized (generic) form
    let mut normalized_groups: HashMap<Vec<PatternField>, Vec<(Vec<PatternField>, String)>> =
        HashMap::new();

    for (fields, name) in signature_to_pattern {
        if let Some(normalized) = normalize_fields_for_generic(fields) {
            normalized_groups
                .entry(normalized)
                .or_default()
                .push((fields.clone(), name.clone()));
        }
    }

    let mut patterns = Vec::new();
    let mut mappings: HashMap<Vec<PatternField>, String> = HashMap::new();

    // Create generic patterns for groups with 2+ different concrete signatures
    for (normalized_fields, group) in normalized_groups {
        if group.len() >= 2 {
            // Use the first pattern's name as the generic pattern name
            let generic_name = group[0].1.clone();

            // Map all concrete signatures to this generic pattern
            for (concrete_fields, _) in &group {
                mappings.insert(concrete_fields.clone(), generic_name.clone());
            }

            patterns.push(StructuralPattern {
                name: generic_name,
                fields: normalized_fields,
                field_positions: HashMap::new(),
                is_generic: true,
            });
        }
    }

    (patterns, mappings)
}

/// Normalize fields by replacing concrete value types with "T" for generic matching.
/// Returns None if the pattern is not suitable for generics (e.g., mixed value types).
fn normalize_fields_for_generic(fields: &[PatternField]) -> Option<Vec<PatternField>> {
    // Get all leaf field value types
    let leaf_types: Vec<&str> = fields
        .iter()
        .filter(|f| !f.indexes.is_empty()) // Only leaves have indexes
        .map(|f| f.rust_type.as_str())
        .collect();

    // Need at least one leaf to be generic
    if leaf_types.is_empty() {
        return None;
    }

    // All leaves must have the same value type
    let first_type = leaf_types[0];
    if !leaf_types.iter().all(|t| *t == first_type) {
        return None;
    }

    // Create normalized fields with "T" as the value type
    let normalized: Vec<PatternField> = fields
        .iter()
        .map(|f| {
            if f.indexes.is_empty() {
                // Branch field - keep as is
                f.clone()
            } else {
                // Leaf field - replace value type with T
                PatternField {
                    name: f.name.clone(),
                    rust_type: "T".to_string(),
                    json_type: "T".to_string(),
                    indexes: f.indexes.clone(),
                }
            }
        })
        .collect();

    Some(normalized)
}

/// Analyze field positions for all patterns by traversing tree instances.
/// For each pattern instance, we compare parent accumulated name with child leaf names.
fn analyze_pattern_field_positions(
    tree: &TreeNode,
    patterns: &mut [StructuralPattern],
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
) {
    // Collect instances: pattern_name -> vec of (accumulated_name, field_name, leaf_name)
    let mut instances: HashMap<String, Vec<(String, String, String)>> = HashMap::new();

    // Traverse tree and collect instances
    collect_pattern_instances(tree, "", &mut instances, pattern_lookup);

    // For each pattern, analyze field positions from instances
    for pattern in patterns.iter_mut() {
        if let Some(pattern_instances) = instances.get(&pattern.name) {
            pattern.field_positions = analyze_field_positions_from_instances(pattern_instances);
        }
    }
}

/// Recursively traverse tree and collect pattern instances with accumulated metric names.
fn collect_pattern_instances(
    node: &TreeNode,
    accumulated_name: &str,
    instances: &mut HashMap<String, Vec<(String, String, String)>>,
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
) {
    if let TreeNode::Branch(children) = node {
        // Check if this branch matches a pattern
        let fields = get_node_fields_for_analysis(children, pattern_lookup);
        if let Some(pattern_name) = pattern_lookup.get(&fields) {
            // Collect instances for this pattern
            for (field_name, child_node) in children {
                if let TreeNode::Leaf(leaf) = child_node {
                    instances.entry(pattern_name.clone()).or_default().push((
                        accumulated_name.to_string(),
                        field_name.clone(),
                        leaf.name().to_string(),
                    ));
                }
            }
        }

        // Continue traversing children
        for (field_name, child_node) in children {
            let child_accumulated = match child_node {
                TreeNode::Leaf(leaf) => leaf.name().to_string(),
                TreeNode::Branch(_) => {
                    // For branches, we need to infer the accumulated name
                    // If there's a leaf descendant, use its name as the basis
                    if let Some(desc_leaf_name) = get_descendant_leaf_name(child_node) {
                        // Try to extract what this level contributes
                        infer_accumulated_name(accumulated_name, field_name, &desc_leaf_name)
                    } else {
                        // No descendants - use field name as base
                        if accumulated_name.is_empty() {
                            field_name.clone()
                        } else {
                            format!("{}_{}", accumulated_name, field_name)
                        }
                    }
                }
            };
            collect_pattern_instances(child_node, &child_accumulated, instances, pattern_lookup);
        }
    }
}

/// Get a descendant leaf name from a branch node (first one found)
fn get_descendant_leaf_name(node: &TreeNode) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => Some(leaf.name().to_string()),
        TreeNode::Branch(children) => {
            for child in children.values() {
                if let Some(name) = get_descendant_leaf_name(child) {
                    return Some(name);
                }
            }
            None
        }
    }
}

/// Infer the accumulated name at this level by analyzing what part of the descendant's name
/// comes from the current field.
fn infer_accumulated_name(parent_acc: &str, field_name: &str, descendant_leaf: &str) -> String {
    // Try to find field_name in the descendant's metric name
    if let Some(pos) = descendant_leaf.find(field_name) {
        // Extract the part that corresponds to this level
        if pos == 0 {
            // Field is at the start
            field_name.to_string()
        } else if pos > 0 && descendant_leaf.chars().nth(pos - 1) == Some('_') {
            // Field appears after underscore - this is likely an append
            if parent_acc.is_empty() {
                field_name.to_string()
            } else {
                format!("{}_{}", parent_acc, field_name)
            }
        } else {
            field_name.to_string()
        }
    } else {
        // Field name not directly found - use as is
        if parent_acc.is_empty() {
            field_name.to_string()
        } else {
            format!("{}_{}", parent_acc, field_name)
        }
    }
}

/// Analyze instances to determine field positions (prepend/append/identity).
fn analyze_field_positions_from_instances(
    instances: &[(String, String, String)],
) -> HashMap<String, FieldNamePosition> {
    // Group by field name
    let mut field_instances: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for (acc, field, leaf) in instances {
        field_instances
            .entry(field.clone())
            .or_default()
            .push((acc.clone(), leaf.clone()));
    }

    let mut positions = HashMap::new();

    for (field_name, field_data) in field_instances {
        if let Some(position) = detect_field_position(&field_data) {
            positions.insert(field_name, position);
        }
    }

    positions
}

/// Detect the position transformation for a field based on (accumulated, leaf_name) pairs.
fn detect_field_position(data: &[(String, String)]) -> Option<FieldNamePosition> {
    if data.is_empty() {
        return None;
    }

    // Try to detect pattern from first instance, then validate against others
    let (first_acc, first_leaf) = &data[0];

    // Case 1: Identity - leaf == accumulated
    if first_acc == first_leaf {
        return Some(FieldNamePosition::Identity);
    }

    // Case 2: Append - leaf = acc + suffix
    if let Some(suffix) = first_leaf.strip_prefix(first_acc.as_str()) {
        let suffix = suffix.to_string();
        // Validate this pattern holds for all instances
        if data.iter().all(|(acc, leaf)| {
            if acc.is_empty() {
                // When acc is empty, leaf should equal suffix (without leading _)
                leaf == suffix.trim_start_matches('_')
            } else {
                leaf.strip_prefix(acc.as_str()) == Some(&suffix)
            }
        }) {
            return Some(FieldNamePosition::Append(suffix));
        }
    }

    // Case 3: Prepend - leaf = prefix + acc
    if let Some(prefix) = first_leaf.strip_suffix(first_acc.as_str()) {
        let prefix = prefix.to_string();
        // Validate this pattern holds for all instances
        if data.iter().all(|(acc, leaf)| {
            if acc.is_empty() {
                // When acc is empty, leaf should equal prefix (without trailing _)
                leaf == prefix.trim_end_matches('_')
            } else {
                leaf.strip_suffix(acc.as_str()) == Some(&prefix)
            }
        }) {
            return Some(FieldNamePosition::Prepend(prefix));
        }
    }

    // Case 4: SetBase - the field name IS the metric base
    // This happens at entry points where accumulated is empty
    if first_acc.is_empty() {
        return Some(FieldNamePosition::SetBase(first_leaf.clone()));
    }

    None
}

/// Get node fields for pattern matching during analysis
fn get_node_fields_for_analysis(
    children: &BTreeMap<String, TreeNode>,
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
                    let child_fields = get_node_fields_for_analysis(grandchildren, pattern_lookup);
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

/// Recursively resolve branch patterns bottom-up.
/// Returns the pattern name for this node if it's a branch, or None if it's a leaf.
fn resolve_branch_patterns(
    node: &TreeNode,
    field_name: &str, // The field name in the parent where this node appears
    signature_to_pattern: &mut HashMap<Vec<PatternField>, String>,
    signature_counts: &mut HashMap<Vec<PatternField>, usize>,
    normalized_to_name: &mut HashMap<Vec<PatternField>, String>, // Normalized sig -> name
    name_counts: &mut HashMap<String, usize>,
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
                            child_name,
                            signature_to_pattern,
                            signature_counts,
                            normalized_to_name,
                            name_counts,
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

            // Get or create pattern name - use normalized signature for naming
            // so patterns that differ only in value type get the same name
            let pattern_name = if let Some(existing) = signature_to_pattern.get(&fields) {
                existing.clone()
            } else {
                // Check if normalized form already has a name
                let normalized = normalize_fields_for_naming(&fields);
                let name = normalized_to_name
                    .entry(normalized)
                    .or_insert_with(|| generate_pattern_name(field_name, name_counts))
                    .clone();
                signature_to_pattern.insert(fields.clone(), name.clone());
                name
            };

            Some(pattern_name)
        }
    }
}

/// Normalize fields for naming: replace value types with a placeholder
/// so patterns with same structure but different value types get the same name.
fn normalize_fields_for_naming(fields: &[PatternField]) -> Vec<PatternField> {
    fields
        .iter()
        .map(|f| {
            if f.indexes.is_empty() {
                // Branch field - keep rust_type (it's a pattern name)
                f.clone()
            } else {
                // Leaf field - normalize value type
                PatternField {
                    name: f.name.clone(),
                    rust_type: "_".to_string(),
                    json_type: "_".to_string(),
                    indexes: f.indexes.clone(),
                }
            }
        })
        .collect()
}

/// Generate a pattern name from the field name where it's used.
/// Appends an index if the same base name is used multiple times.
fn generate_pattern_name(field_name: &str, name_counts: &mut HashMap<String, usize>) -> String {
    let pascal = to_pascal_case(field_name);

    // Sanitize: ensure it starts with a letter (prepend "_" if starts with digit)
    let base_name = if pascal
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("_{}", pascal)
    } else {
        pascal
    };

    // Track usage count and append index if needed
    let count = name_counts.entry(base_name.clone()).or_insert(0);
    *count += 1;

    if *count == 1 {
        base_name
    } else {
        format!("{}{}", base_name, count)
    }
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
    // Normalize separators: replace - with _
    let normalized = s.replace('-', "_");
    normalized
        .split('_')
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

    // Prefix with _ if starts with digit
    let sanitized = if sanitized
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("_{}", sanitized)
    } else {
        sanitized
    };

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
    let result = match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    };

    // Prefix with _ if starts with digit
    if result
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        format!("_{}", result)
    } else {
        result
    }
}

/// Get the first leaf name from a tree node (used across all generators)
pub fn get_first_leaf_name(node: &TreeNode) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => Some(leaf.name().to_string()),
        TreeNode::Branch(children) => {
            for child in children.values() {
                if let Some(name) = get_first_leaf_name(child) {
                    return Some(name);
                }
            }
            None
        }
    }
}

/// Get the metric base for a pattern instance by analyzing the first leaf descendant.
/// This extracts the common base that all leaves in this pattern instance share.
pub fn get_pattern_instance_base(node: &TreeNode, field_name: &str) -> String {
    if let Some(leaf_name) = get_first_leaf_name(node) {
        // Look for field_name in the leaf metric name
        if leaf_name.contains(field_name) {
            // The field name is part of the metric - use it as base
            return field_name.to_string();
        }
    }
    // Fallback: use field name
    field_name.to_string()
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
        .enumerate()
        .map(|(i, (indexes, _))| IndexSetPattern {
            name: if i == 0 {
                "Indexes".to_string()
            } else {
                format!("Indexes{}", i + 1)
            },
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
