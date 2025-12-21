//! Pattern detection for structural patterns in the metric tree.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use brk_types::TreeNode;

use super::{
    case::to_pascal_case, schema::schema_to_json_type, FieldNamePosition, PatternField,
    StructuralPattern,
};

/// Detect structural patterns in the tree using a bottom-up approach.
/// Returns (patterns, concrete_to_pattern_mapping).
pub fn detect_structural_patterns(
    tree: &TreeNode,
) -> (Vec<StructuralPattern>, HashMap<Vec<PatternField>, String>) {
    let mut signature_to_pattern: HashMap<Vec<PatternField>, String> = HashMap::new();
    let mut signature_counts: HashMap<Vec<PatternField>, usize> = HashMap::new();
    let mut normalized_to_name: HashMap<Vec<PatternField>, String> = HashMap::new();
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

    // Identify generic patterns
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

    patterns.extend(generic_patterns);

    // Build lookup for field position analysis
    let mut pattern_lookup: HashMap<Vec<PatternField>, String> = HashMap::new();
    for (sig, name) in &signature_to_pattern {
        if signature_counts.get(sig).copied().unwrap_or(0) >= 2 {
            pattern_lookup.insert(sig.clone(), name.clone());
        }
    }
    pattern_lookup.extend(generic_mappings.clone());

    let concrete_to_pattern = pattern_lookup.clone();

    // Second pass: analyze field positions
    analyze_pattern_field_positions(tree, &mut patterns, &pattern_lookup);

    patterns.sort_by(|a, b| b.fields.len().cmp(&a.fields.len()));
    (patterns, concrete_to_pattern)
}

/// Detect generic patterns by grouping signatures by their normalized form.
fn detect_generic_patterns(
    signature_to_pattern: &HashMap<Vec<PatternField>, String>,
) -> (Vec<StructuralPattern>, HashMap<Vec<PatternField>, String>) {
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

    for (normalized_fields, group) in normalized_groups {
        if group.len() >= 2 {
            let generic_name = group[0].1.clone();
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

/// Normalize fields by replacing concrete value types with "T".
fn normalize_fields_for_generic(fields: &[PatternField]) -> Option<Vec<PatternField>> {
    let leaf_types: Vec<&str> = fields
        .iter()
        .filter(|f| f.is_leaf())
        .map(|f| f.rust_type.as_str())
        .collect();

    if leaf_types.is_empty() {
        return None;
    }

    let first_type = leaf_types[0];
    if !leaf_types.iter().all(|t| *t == first_type) {
        return None;
    }

    let normalized = fields
        .iter()
        .map(|f| {
            if f.is_branch() {
                f.clone()
            } else {
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

/// Recursively resolve branch patterns bottom-up.
fn resolve_branch_patterns(
    node: &TreeNode,
    field_name: &str,
    signature_to_pattern: &mut HashMap<Vec<PatternField>, String>,
    signature_counts: &mut HashMap<Vec<PatternField>, usize>,
    normalized_to_name: &mut HashMap<Vec<PatternField>, String>,
    name_counts: &mut HashMap<String, usize>,
) -> Option<String> {
    let TreeNode::Branch(children) = node else {
        return None;
    };

    let mut fields: Vec<PatternField> = Vec::new();
    for (child_name, child_node) in children {
        let (rust_type, json_type, indexes) = match child_node {
            TreeNode::Leaf(leaf) => (
                leaf.value_type().to_string(),
                schema_to_json_type(&leaf.schema),
                leaf.indexes().clone(),
            ),
            TreeNode::Branch(_) => {
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

    fields.sort_by(|a, b| a.name.cmp(&b.name));
    *signature_counts.entry(fields.clone()).or_insert(0) += 1;

    let pattern_name = if let Some(existing) = signature_to_pattern.get(&fields) {
        existing.clone()
    } else {
        let normalized = normalize_fields_for_naming(&fields);
        let name = normalized_to_name
            .entry(normalized)
            .or_insert_with(|| generate_pattern_name(field_name, name_counts))
            .clone();
        signature_to_pattern.insert(fields, name.clone());
        name
    };

    Some(pattern_name)
}

/// Normalize fields for naming (same structure = same name).
fn normalize_fields_for_naming(fields: &[PatternField]) -> Vec<PatternField> {
    fields
        .iter()
        .map(|f| {
            if f.is_branch() {
                f.clone()
            } else {
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

/// Generate a unique pattern name.
fn generate_pattern_name(field_name: &str, name_counts: &mut HashMap<String, usize>) -> String {
    let pascal = to_pascal_case(field_name);
    let sanitized = if pascal.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("_{}", pascal)
    } else {
        pascal
    };

    let base_name = format!("{}Pattern", sanitized);
    let count = name_counts.entry(base_name.clone()).or_insert(0);
    *count += 1;

    if *count == 1 {
        base_name
    } else {
        format!("{}{}", base_name, count)
    }
}

// Field position analysis

fn analyze_pattern_field_positions(
    tree: &TreeNode,
    patterns: &mut [StructuralPattern],
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
) {
    let mut instances: HashMap<String, Vec<(String, String, String)>> = HashMap::new();
    collect_pattern_instances(tree, "", &mut instances, pattern_lookup);

    for pattern in patterns.iter_mut() {
        if let Some(pattern_instances) = instances.get(&pattern.name) {
            pattern.field_positions = analyze_field_positions_from_instances(pattern_instances);
        }
    }
}

fn collect_pattern_instances(
    node: &TreeNode,
    accumulated_name: &str,
    instances: &mut HashMap<String, Vec<(String, String, String)>>,
    pattern_lookup: &HashMap<Vec<PatternField>, String>,
) {
    let TreeNode::Branch(children) = node else {
        return;
    };

    let fields = get_node_fields_for_analysis(children, pattern_lookup);
    if let Some(pattern_name) = pattern_lookup.get(&fields) {
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

    for (field_name, child_node) in children {
        let child_accumulated = match child_node {
            TreeNode::Leaf(leaf) => leaf.name().to_string(),
            TreeNode::Branch(_) => {
                if let Some(desc_leaf_name) = get_descendant_leaf_name(child_node) {
                    infer_accumulated_name(accumulated_name, field_name, &desc_leaf_name)
                } else if accumulated_name.is_empty() {
                    field_name.clone()
                } else {
                    format!("{}_{}", accumulated_name, field_name)
                }
            }
        };
        collect_pattern_instances(child_node, &child_accumulated, instances, pattern_lookup);
    }
}

fn get_descendant_leaf_name(node: &TreeNode) -> Option<String> {
    match node {
        TreeNode::Leaf(leaf) => Some(leaf.name().to_string()),
        TreeNode::Branch(children) => children.values().find_map(get_descendant_leaf_name),
    }
}

fn infer_accumulated_name(parent_acc: &str, field_name: &str, descendant_leaf: &str) -> String {
    if let Some(pos) = descendant_leaf.find(field_name) {
        if pos == 0 {
            return field_name.to_string();
        }
        if pos > 0 && descendant_leaf.chars().nth(pos - 1) == Some('_') {
            return if parent_acc.is_empty() {
                field_name.to_string()
            } else {
                format!("{}_{}", parent_acc, field_name)
            };
        }
    }

    if parent_acc.is_empty() {
        field_name.to_string()
    } else {
        format!("{}_{}", parent_acc, field_name)
    }
}

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

fn analyze_field_positions_from_instances(
    instances: &[(String, String, String)],
) -> HashMap<String, FieldNamePosition> {
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

fn detect_field_position(data: &[(String, String)]) -> Option<FieldNamePosition> {
    if data.is_empty() {
        return None;
    }

    let (first_acc, first_leaf) = &data[0];

    // Identity
    if first_acc == first_leaf {
        return Some(FieldNamePosition::Identity);
    }

    // Append
    if let Some(suffix) = first_leaf.strip_prefix(first_acc.as_str()) {
        let suffix = suffix.to_string();
        if data.iter().all(|(acc, leaf)| {
            if acc.is_empty() {
                leaf == suffix.trim_start_matches('_')
            } else {
                leaf.strip_prefix(acc.as_str()) == Some(&suffix)
            }
        }) {
            return Some(FieldNamePosition::Append(suffix));
        }
    }

    // Prepend
    if let Some(prefix) = first_leaf.strip_suffix(first_acc.as_str()) {
        let prefix = prefix.to_string();
        if data.iter().all(|(acc, leaf)| {
            if acc.is_empty() {
                leaf == prefix.trim_end_matches('_')
            } else {
                leaf.strip_suffix(acc.as_str()) == Some(&prefix)
            }
        }) {
            return Some(FieldNamePosition::Prepend(prefix));
        }
    }

    // SetBase
    if first_acc.is_empty() {
        return Some(FieldNamePosition::SetBase(first_leaf.clone()));
    }

    None
}
