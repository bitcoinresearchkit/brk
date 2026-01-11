use brk_types::Index;

/// Convert a string to PascalCase (e.g., "fee_rate" -> "FeeRate").
pub fn to_pascal_case(s: &str) -> String {
    s.replace('-', "_")
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

/// Convert a string to snake_case, handling Rust keywords.
pub fn to_snake_case(s: &str) -> String {
    // Convert to lowercase and replace dashes with underscores
    let sanitized = s.to_lowercase().replace('-', "_");

    // Prefix with _ if starts with digit
    let sanitized = if sanitized.chars().next().is_some_and(|c| c.is_ascii_digit()) {
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

/// Convert a string to camelCase (e.g., "fee_rate" -> "feeRate").
pub fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();

    let result = match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    };

    // Prefix with _ if starts with digit
    if result.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("_{}", result)
    } else {
        result
    }
}

/// Convert an Index to a snake_case field name (e.g., DateIndex -> dateindex).
pub fn index_to_field_name(index: &Index) -> String {
    to_snake_case(index.serialize_long())
}

/// Generate a child type/struct/class name (e.g., ParentName + child_name -> ParentName_ChildName).
pub fn child_type_name(parent: &str, child: &str) -> String {
    format!("{}_{}", parent, to_pascal_case(child))
}

/// Escape Python reserved keywords by appending an underscore.
/// Also prefixes names starting with digits with an underscore.
pub fn escape_python_keyword(name: &str) -> String {
    const PYTHON_KEYWORDS: &[&str] = &[
        "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class",
        "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global",
        "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return",
        "try", "while", "with", "yield",
    ];

    // Prefix with underscore if starts with digit
    let name = if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{}", name)
    } else {
        name.to_string()
    };

    // Append underscore if it's a keyword
    if PYTHON_KEYWORDS.contains(&name.as_str()) {
        format!("{}_", name)
    } else {
        name
    }
}
