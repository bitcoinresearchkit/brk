use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pattern {
    fields: Vec<String>,
    field_count: usize,
}

fn sanitize_name(name: &str) -> String {
    // Python identifiers can't start with numbers
    if name.chars().next().unwrap().is_numeric() {
        format!("_{}", name)
    } else {
        name.replace("-", "_")
    }
}

fn extract_pattern(obj: &Map<String, Value>) -> Pattern {
    let mut fields: Vec<String> = obj.keys().cloned().collect();
    fields.sort();
    Pattern {
        field_count: fields.len(),
        fields,
    }
}

// Calculate similarity between two patterns (0.0 = different, 1.0 = identical)
fn pattern_similarity(p1: &Pattern, p2: &Pattern) -> f64 {
    if p1.field_count == 0 || p2.field_count == 0 {
        return 0.0;
    }

    let set1: HashSet<_> = p1.fields.iter().collect();
    let set2: HashSet<_> = p2.fields.iter().collect();

    let intersection = set1.intersection(&set2).count();
    let union = set1.union(&set2).count();

    intersection as f64 / union as f64
}

// Group similar patterns together
fn cluster_patterns(patterns: &HashMap<Pattern, Vec<String>>) -> Vec<Vec<(Pattern, Vec<String>)>> {
    let mut clusters: Vec<Vec<(Pattern, Vec<String>)>> = Vec::new();
    let similarity_threshold = 0.7; // 70% overlap

    for (pattern, paths) in patterns {
        let mut found_cluster = false;

        for cluster in clusters.iter_mut() {
            let representative = &cluster[0].0;
            if pattern_similarity(pattern, representative) >= similarity_threshold {
                cluster.push((pattern.clone(), paths.clone()));
                found_cluster = true;
                break;
            }
        }

        if !found_cluster {
            clusters.push(vec![(pattern.clone(), paths.clone())]);
        }
    }

    clusters
}

// Merge similar patterns into a flexible pattern
fn merge_patterns_in_cluster(
    cluster: &[(Pattern, Vec<String>)],
) -> (Pattern, HashMap<String, bool>) {
    let mut all_fields: HashSet<String> = HashSet::new();
    let mut field_counts: HashMap<String, usize> = HashMap::new();
    let total_patterns = cluster.len();

    // Collect all fields and count occurrences
    for (pattern, _) in cluster {
        for field in &pattern.fields {
            all_fields.insert(field.clone());
            *field_counts.entry(field.clone()).or_insert(0) += 1;
        }
    }

    // Sort fields
    let mut sorted_fields: Vec<String> = all_fields.into_iter().collect();
    sorted_fields.sort();

    // Mark which fields are required (present in >80% of patterns)
    let mut required_fields: HashMap<String, bool> = HashMap::new();
    for field in &sorted_fields {
        let count = field_counts.get(field).unwrap_or(&0);
        required_fields.insert(field.clone(), *count as f64 / total_patterns as f64 > 0.8);
    }

    (
        Pattern {
            fields: sorted_fields,
            field_count: field_counts.len(),
        },
        required_fields,
    )
}

fn find_patterns(tree: &Value, patterns: &mut HashMap<Pattern, Vec<String>>, path: String) {
    match tree {
        Value::Object(map) => {
            // Check if this is a leaf object (all values are strings)
            let is_leaf = map.values().all(|v| v.is_string());

            if is_leaf && map.len() > 5 {
                // This might be a reusable pattern
                let pattern = extract_pattern(map);
                patterns
                    .entry(pattern)
                    .or_insert_with(Vec::new)
                    .push(path.clone());
            }

            // Recurse into children
            for (key, value) in map {
                let new_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };
                find_patterns(value, patterns, new_path);
            }
        }
        _ => {}
    }
}

fn traverse_to_path<'a>(tree: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = tree;
    for segment in path {
        if let Value::Object(map) = current {
            current = map.get(*segment)?;
        } else {
            return None;
        }
    }
    Some(current)
}

fn generate_python_pattern_class(
    merged_pattern: &Pattern,
    required_fields: &HashMap<String, bool>,
    class_name: &str,
    example_path: &str,
    tree: &Value,
) -> String {
    let mut output = String::new();

    output.push_str(&format!("class {}Namespace:\n", class_name));
    output.push_str(&format!(
        "    \"\"\"Pattern for {} (supports {} fields)\"\"\"\n",
        class_name, merged_pattern.field_count
    ));

    let slots: Vec<String> = merged_pattern
        .fields
        .iter()
        .map(|f| sanitize_name(f))
        .collect();
    output.push_str(&format!(
        "    __slots__ = ({})\n\n",
        slots
            .iter()
            .map(|s| format!("'{}'", s))
            .collect::<Vec<_>>()
            .join(", ")
    ));

    output.push_str("    def __init__(self, path: str, prefix: str):\n");

    let path_segments: Vec<&str> = example_path.split('.').collect();
    if let Some(obj) = traverse_to_path(tree, &path_segments) {
        if let Value::Object(map) = obj {
            for field in &merged_pattern.fields {
                let safe_field = sanitize_name(field);
                if let Some(Value::String(metric_name)) = map.get(field) {
                    output.push_str(&format!(
                        "        self.{} = f\"{{path}}/{{prefix}}_{}\"\n",
                        safe_field, metric_name
                    ));
                }
            }
        }
    }

    output.push_str("\n\n");
    output
}

fn generate_python_namespace_class(
    name: &str,
    obj: &Map<String, Value>,
    tree: &Value,
    api_path: &str,
    pattern_classes: &HashMap<Pattern, String>,
) -> String {
    let mut output = String::new();
    let class_name = format!(
        "{}Namespace",
        name.split('_')
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<String>()
    );

    output.push_str(&format!("class {}:\n", class_name));
    output.push_str(&format!("    \"\"\"Namespace for {} metrics\"\"\"\n", name));

    let mut slots = vec![];
    let mut init_lines = vec![];

    for (key, value) in obj {
        let safe_key = sanitize_name(key);
        slots.push(safe_key.clone());

        match value {
            Value::String(metric_name) => {
                init_lines.push(format!(
                    "        self.{} = f\"{}/{}\"",
                    safe_key, api_path, metric_name
                ));
            }
            Value::Object(nested_map) => {
                let pattern = extract_pattern(nested_map);
                if let Some(pattern_class) = pattern_classes.get(&pattern) {
                    init_lines.push(format!(
                        "        self.{} = {}Namespace(\"{}\", \"{}\")",
                        safe_key, pattern_class, api_path, key
                    ));
                } else {
                    let nested_class = format!(
                        "{}{}",
                        class_name.trim_end_matches("Namespace"),
                        key.split('_')
                            .map(|s| {
                                let mut c = s.chars();
                                match c.next() {
                                    None => String::new(),
                                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                                }
                            })
                            .collect::<String>()
                    );
                    init_lines.push(format!("        self.{} = {}Namespace()", safe_key));
                }
            }
            _ => {}
        }
    }

    output.push_str(&format!(
        "    __slots__ = ({})\n\n",
        slots
            .iter()
            .map(|s| format!("'{}'", s))
            .collect::<Vec<_>>()
            .join(", ")
    ));

    output.push_str("    def __init__(self):\n");
    for line in init_lines {
        output.push_str(&format!("{}\n", line));
    }

    output.push_str("\n\n");
    output
}

fn generate_python_namespaces_recursive(
    obj: &Map<String, Value>,
    tree: &Value,
    pattern_classes: &HashMap<Pattern, String>,
    path: &str,
    output: &mut String,
) {
    for (key, value) in obj {
        if let Value::Object(nested_map) = value {
            let new_path = if path.is_empty() {
                key.clone()
            } else {
                format!("{}/{}", path, key)
            };

            let is_leaf = nested_map.values().all(|v| v.is_string());
            if !is_leaf {
                generate_python_namespaces_recursive(
                    nested_map,
                    tree,
                    pattern_classes,
                    &new_path,
                    output,
                );
            }
        }
    }

    let api_path = path.replace(".", "/");
    let name = path.split('/').last().unwrap_or("Root");
    output.push_str(&generate_python_namespace_class(
        name,
        obj,
        tree,
        &api_path,
        pattern_classes,
    ));
}

fn generate_python_client(tree: &Value) -> String {
    let mut output = String::new();

    output.push_str(
        r#""""
BRK API Tree - Auto-generated from config

Each attribute is a string representing the API path + metric name.
Use these paths with your own fetch implementation.

DO NOT EDIT - This file is generated by codegen
"""

"#,
    );

    output.push_str(
        "# ============================================================================\n",
    );
    output.push_str("# PATTERN CLASSES\n");
    output.push_str(
        "# ============================================================================\n\n",
    );

    let mut patterns: HashMap<Pattern, Vec<String>> = HashMap::new();
    find_patterns(tree, &mut patterns, String::new());

    let clusters = cluster_patterns(&patterns);
    let mut pattern_classes: HashMap<Pattern, String> = HashMap::new();
    let mut cluster_id = 0;

    for cluster in clusters.iter() {
        let total_usage: usize = cluster.iter().map(|(_, paths)| paths.len()).sum();

        if total_usage >= 3 && cluster[0].0.field_count >= 8 {
            let (merged_pattern, required_fields) = merge_patterns_in_cluster(cluster);

            let class_name = if merged_pattern.fields.iter().any(|f| f.contains("ratio")) {
                format!("RatioPattern{}", cluster_id)
            } else if merged_pattern.fields.iter().any(|f| f.contains("count")) {
                format!("CountPattern{}", cluster_id)
            } else {
                format!("CommonPattern{}", cluster_id)
            };

            output.push_str(&generate_python_pattern_class(
                &merged_pattern,
                &required_fields,
                &class_name,
                &cluster[0].1[0],
                tree,
            ));

            for (pattern, _) in cluster {
                pattern_classes.insert(pattern.clone(), class_name.clone());
            }

            cluster_id += 1;
        }
    }

    output.push_str(
        "# ============================================================================\n",
    );
    output.push_str("# NAMESPACE CLASSES\n");
    output.push_str(
        "# ============================================================================\n\n",
    );

    if let Value::Object(root) = tree {
        generate_python_namespaces_recursive(root, tree, &pattern_classes, "", &mut output);
    }

    output.push_str(
        r#"
class BRKTree:
    """
    BRK API Tree

    Usage:
        tree = BRKTree()
        path = tree.computed.chain.block_count.base
        # path is now "computed/chain/block_count"
        # Use this path with your own HTTP client
    """
    __slots__ = ("computed", "cointime", "constants", "fetched", "indexes", "market")

    def __init__(self):
"#,
    );

    if let Value::Object(root) = tree {
        for key in root.keys() {
            output.push_str(&format!(
                "        self.{} = {}Namespace()\n",
                sanitize_name(key),
                key.split('_')
                    .map(|s| {
                        let mut c = s.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        }
                    })
                    .collect::<String>()
            ));
        }
    }

    output
}

fn to_pascal_case(s: &str) -> String {
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

fn generate_typescript_pattern_class(
    merged_pattern: &Pattern,
    class_name: &str,
    example_path: &str,
    tree: &Value,
) -> String {
    let mut output = String::new();

    output.push_str(&format!("export class {}Namespace {{\n", class_name));

    for field in &merged_pattern.fields {
        let safe_field = sanitize_name(field);
        output.push_str(&format!("  readonly {}: string;\n", safe_field));
    }

    output.push_str("\n  constructor(path: string, prefix: string) {\n");

    let path_segments: Vec<&str> = example_path.split('.').collect();
    if let Some(obj) = traverse_to_path(tree, &path_segments) {
        if let Value::Object(map) = obj {
            for field in &merged_pattern.fields {
                let safe_field = sanitize_name(field);
                if let Some(Value::String(metric_name)) = map.get(field) {
                    output.push_str(&format!(
                        "    this.{} = `${{path}}/${{prefix}}_{}`;\n",
                        safe_field, metric_name
                    ));
                }
            }
        }
    }

    output.push_str("  }\n}\n\n");
    output
}

fn generate_typescript_namespaces_recursive(
    obj: &Map<String, Value>,
    tree: &Value,
    pattern_classes: &HashMap<Pattern, String>,
    path: &str,
    output: &mut String,
) {
    for (key, value) in obj {
        if let Value::Object(nested_map) = value {
            let new_path = if path.is_empty() {
                key.clone()
            } else {
                format!("{}/{}", path, key)
            };

            let is_leaf = nested_map.values().all(|v| v.is_string());
            if !is_leaf {
                generate_typescript_namespaces_recursive(
                    nested_map,
                    tree,
                    pattern_classes,
                    &new_path,
                    output,
                );
            }
        }
    }

    let api_path = path.replace(".", "/");
    let name = path.split('/').last().unwrap_or("Root");
    let class_name = to_pascal_case(name);

    output.push_str(&format!("export class {}Namespace {{\n", class_name));

    for (key, value) in obj {
        let safe_key = sanitize_name(key);
        match value {
            Value::String(_) => {
                output.push_str(&format!("  readonly {}: string;\n", safe_key));
            }
            Value::Object(nested_map) => {
                let pattern = extract_pattern(nested_map);
                if let Some(pattern_class) = pattern_classes.get(&pattern) {
                    output.push_str(&format!(
                        "  readonly {}: {}Namespace;\n",
                        safe_key, pattern_class
                    ));
                } else {
                    let nested_class = format!("{}{}", class_name, to_pascal_case(key));
                    output.push_str(&format!(
                        "  readonly {}: {}Namespace;\n",
                        safe_key, nested_class
                    ));
                }
            }
            _ => {}
        }
    }

    output.push_str("\n  constructor() {\n");

    for (key, value) in obj {
        let safe_key = sanitize_name(key);
        match value {
            Value::String(metric_name) => {
                output.push_str(&format!(
                    "    this.{} = '{}/{}';\n",
                    safe_key, api_path, metric_name
                ));
            }
            Value::Object(nested_map) => {
                let pattern = extract_pattern(nested_map);
                if let Some(pattern_class) = pattern_classes.get(&pattern) {
                    output.push_str(&format!(
                        "    this.{} = new {}Namespace('{}', '{}');\n",
                        safe_key, pattern_class, api_path, key
                    ));
                } else {
                    let nested_class = format!("{}{}", class_name, to_pascal_case(key));
                    output.push_str(&format!(
                        "    this.{} = new {}Namespace();\n",
                        safe_key, nested_class
                    ));
                }
            }
            _ => {}
        }
    }

    output.push_str("  }\n}\n\n");
}

fn generate_typescript_client(tree: &Value) -> String {
    let mut output = String::new();

    output.push_str(
        r#"/**
 * BRK API Tree - Auto-generated from config
 *
 * Each property is a string representing the API path + metric name.
 * Use these paths with your own fetch implementation.
 *
 * DO NOT EDIT - This file is generated by codegen
 */

"#,
    );

    let mut patterns: HashMap<Pattern, Vec<String>> = HashMap::new();
    find_patterns(tree, &mut patterns, String::new());
    let clusters = cluster_patterns(&patterns);

    let mut pattern_classes: HashMap<Pattern, String> = HashMap::new();
    let mut cluster_id = 0;

    for cluster in clusters.iter() {
        let total_usage: usize = cluster.iter().map(|(_, paths)| paths.len()).sum();

        if total_usage >= 3 && cluster[0].0.field_count >= 8 {
            let (merged_pattern, _) = merge_patterns_in_cluster(cluster);

            let class_name = if merged_pattern.fields.iter().any(|f| f.contains("ratio")) {
                format!("RatioPattern{}", cluster_id)
            } else if merged_pattern.fields.iter().any(|f| f.contains("count")) {
                format!("CountPattern{}", cluster_id)
            } else {
                format!("CommonPattern{}", cluster_id)
            };

            output.push_str(&generate_typescript_pattern_class(
                &merged_pattern,
                &class_name,
                &cluster[0].1[0],
                tree,
            ));

            for (pattern, _) in cluster {
                pattern_classes.insert(pattern.clone(), class_name.clone());
            }

            cluster_id += 1;
        }
    }

    if let Value::Object(root) = tree {
        generate_typescript_namespaces_recursive(root, tree, &pattern_classes, "", &mut output);
    }

    output.push_str(
        r#"
export class BRKTree {
"#,
    );

    if let Value::Object(root) = tree {
        for key in root.keys() {
            let class_name = to_pascal_case(key);
            output.push_str(&format!(
                "  readonly {}: {}Namespace;\n",
                sanitize_name(key),
                class_name
            ));
        }
    }

    output.push_str("\n  constructor() {\n");

    if let Value::Object(root) = tree {
        for key in root.keys() {
            let class_name = to_pascal_case(key);
            output.push_str(&format!(
                "    this.{} = new {}Namespace();\n",
                sanitize_name(key),
                class_name
            ));
        }
    }

    output.push_str("  }\n}\n");

    output
}

fn to_snake_case(s: &str) -> String {
    let sanitized = s.replace("-", "_");
    match sanitized.as_str() {
        "type" | "const" | "static" | "match" | "if" | "else" | "loop" | "while" => {
            format!("r#{}", sanitized)
        }
        _ => sanitized,
    }
}

fn generate_rust_pattern_struct(
    merged_pattern: &Pattern,
    struct_name: &str,
    example_path: &str,
    tree: &Value,
) -> String {
    let mut output = String::new();

    output.push_str(&format!("/// Pattern for {} metrics\n", struct_name));
    output.push_str("#[derive(Clone, Debug)]\n");
    output.push_str(&format!("pub struct {}Namespace {{\n", struct_name));

    for field in &merged_pattern.fields {
        let safe_field = to_snake_case(&sanitize_name(field));
        output.push_str(&format!("    pub {}: String,\n", safe_field));
    }

    output.push_str("}\n\n");

    output.push_str(&format!("impl {}Namespace {{\n", struct_name));
    output.push_str("    fn new(path: &str, prefix: &str) -> Self {\n");
    output.push_str("        Self {\n");

    let path_segments: Vec<&str> = example_path.split('.').collect();
    if let Some(obj) = traverse_to_path(tree, &path_segments) {
        if let Value::Object(map) = obj {
            for field in &merged_pattern.fields {
                let safe_field = to_snake_case(&sanitize_name(field));
                if let Some(Value::String(metric_name)) = map.get(field) {
                    output.push_str(&format!(
                        "            {}: format!(\"{{}}/{{}}_{}}\", path, prefix),\n",
                        safe_field, metric_name
                    ));
                }
            }
        }
    }

    output.push_str("        }\n    }\n}\n\n");
    output
}

fn generate_rust_namespaces_recursive(
    obj: &Map<String, Value>,
    tree: &Value,
    pattern_classes: &HashMap<Pattern, String>,
    path: &str,
    output: &mut String,
) {
    for (key, value) in obj {
        if let Value::Object(nested_map) = value {
            let new_path = if path.is_empty() {
                key.clone()
            } else {
                format!("{}/{}", path, key)
            };

            let is_leaf = nested_map.values().all(|v| v.is_string());
            if !is_leaf {
                generate_rust_namespaces_recursive(
                    nested_map,
                    tree,
                    pattern_classes,
                    &new_path,
                    output,
                );
            }
        }
    }

    let api_path = path.replace(".", "/");
    let name = path.split('/').last().unwrap_or("Root");
    let struct_name = to_pascal_case(name);

    output.push_str(&format!("/// Namespace for {} metrics\n", name));
    output.push_str("#[derive(Clone, Debug)]\n");
    output.push_str(&format!("pub struct {}Namespace {{\n", struct_name));

    for (key, value) in obj {
        let safe_key = to_snake_case(&sanitize_name(key));
        match value {
            Value::String(_) => {
                output.push_str(&format!("    pub {}: String,\n", safe_key));
            }
            Value::Object(nested_map) => {
                let pattern = extract_pattern(nested_map);
                if let Some(pattern_class) = pattern_classes.get(&pattern) {
                    output.push_str(&format!(
                        "    pub {}: {}Namespace,\n",
                        safe_key, pattern_class
                    ));
                } else {
                    let nested_struct = format!("{}{}", struct_name, to_pascal_case(key));
                    output.push_str(&format!(
                        "    pub {}: {}Namespace,\n",
                        safe_key, nested_struct
                    ));
                }
            }
            _ => {}
        }
    }

    output.push_str("}\n\n");

    output.push_str(&format!("impl {}Namespace {{\n", struct_name));
    output.push_str("    fn new() -> Self {\n        Self {\n");

    for (key, value) in obj {
        let safe_key = to_snake_case(&sanitize_name(key));
        match value {
            Value::String(metric_name) => {
                output.push_str(&format!(
                    "            {}: \"{}/{}\".to_string(),\n",
                    safe_key, api_path, metric_name
                ));
            }
            Value::Object(nested_map) => {
                let pattern = extract_pattern(nested_map);
                if let Some(pattern_class) = pattern_classes.get(&pattern) {
                    output.push_str(&format!(
                        "            {}: {}Namespace::new(\"{}\", \"{}\"),\n",
                        safe_key, pattern_class, api_path, key
                    ));
                } else {
                    let nested_struct = format!("{}{}", struct_name, to_pascal_case(key));
                    output.push_str(&format!(
                        "            {}: {}Namespace::new(),\n",
                        safe_key, nested_struct
                    ));
                }
            }
            _ => {}
        }
    }

    output.push_str("        }\n    }\n}\n\n");
}

fn generate_rust_client(tree: &Value) -> String {
    let mut output = String::new();

    output.push_str(
        r#"//! BRK API Tree - Auto-generated from config
//!
//! Each field is a String representing the API path + metric name.
//! Use these paths with your own HTTP client.
//!
//! DO NOT EDIT - This file is generated by codegen

"#,
    );

    let mut patterns: HashMap<Pattern, Vec<String>> = HashMap::new();
    find_patterns(tree, &mut patterns, String::new());
    let clusters = cluster_patterns(&patterns);

    let mut pattern_classes: HashMap<Pattern, String> = HashMap::new();
    let mut cluster_id = 0;

    for cluster in clusters.iter() {
        let total_usage: usize = cluster.iter().map(|(_, paths)| paths.len()).sum();

        if total_usage >= 3 && cluster[0].0.field_count >= 8 {
            let (merged_pattern, _) = merge_patterns_in_cluster(cluster);

            let class_name = if merged_pattern.fields.iter().any(|f| f.contains("ratio")) {
                format!("RatioPattern{}", cluster_id)
            } else if merged_pattern.fields.iter().any(|f| f.contains("count")) {
                format!("CountPattern{}", cluster_id)
            } else {
                format!("CommonPattern{}", cluster_id)
            };

            output.push_str(&generate_rust_pattern_struct(
                &merged_pattern,
                &class_name,
                &cluster[0].1[0],
                tree,
            ));

            for (pattern, _) in cluster {
                pattern_classes.insert(pattern.clone(), class_name.clone());
            }

            cluster_id += 1;
        }
    }

    if let Value::Object(root) = tree {
        generate_rust_namespaces_recursive(root, tree, &pattern_classes, "", &mut output);
    }

    output.push_str("/// Main BRK API tree\n");
    output.push_str("#[derive(Clone, Debug)]\n");
    output.push_str("pub struct BRKTree {\n");

    if let Value::Object(root) = tree {
        for key in root.keys() {
            let struct_name = to_pascal_case(key);
            output.push_str(&format!(
                "    pub {}: {}Namespace,\n",
                to_snake_case(key),
                struct_name
            ));
        }
    }

    output.push_str("}\n\nimpl BRKTree {\n    pub fn new() -> Self {\n        Self {\n");

    if let Value::Object(root) = tree {
        for key in root.keys() {
            let struct_name = to_pascal_case(key);
            output.push_str(&format!(
                "            {}: {}Namespace::new(),\n",
                to_snake_case(key),
                struct_name
            ));
        }
    }

    output.push_str("        }\n    }\n}\n\nimpl Default for BRKTree {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n");

    output
}

fn main() {
    let json_str = fs::read_to_string("brk_config.json").expect("Failed to read config file");

    let tree: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    // Generate Python tree
    let python_code = generate_python_client(&tree);
    fs::write("brk_tree_generated.py", python_code).expect("Failed to write Python file");
    println!("✓ Generated brk_tree_generated.py");

    // Generate TypeScript tree
    let ts_code = generate_typescript_client(&tree);
    fs::write("brk_tree_generated.ts", ts_code).expect("Failed to write TypeScript file");
    println!("✓ Generated brk_tree_generated.ts");

    // Generate Rust tree
    let rust_code = generate_rust_client(&tree);
    fs::write("brk_tree_generated.rs", rust_code).expect("Failed to write Rust file");
    println!("✓ Generated brk_tree_generated.rs");
}
