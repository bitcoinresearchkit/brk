//! Case conversion utilities for identifiers.

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
    let sanitized = s.replace('-', "_");

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
