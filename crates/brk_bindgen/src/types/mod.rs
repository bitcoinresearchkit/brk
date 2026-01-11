//! Core types for client generation.

mod case;
mod metadata;
mod positions;
mod schema;
mod structs;

pub use case::*;
pub use metadata::*;
pub use positions::*;
pub use schema::*;
pub use structs::*;

/// Language-specific syntax for generic type annotations.
#[derive(Clone, Copy)]
pub struct GenericSyntax {
    pub open: char,
    pub close: char,
    pub default_type: &'static str,
}

impl GenericSyntax {
    pub const PYTHON: Self = Self {
        open: '[',
        close: ']',
        default_type: "Any",
    };
    pub const JAVASCRIPT: Self = Self {
        open: '<',
        close: '>',
        default_type: "unknown",
    };
    pub const RUST: Self = Self {
        open: '<',
        close: '>',
        default_type: "_",
    };

    pub fn wrap(&self, name: &str, type_param: &str) -> String {
        // Convert the type_param from Rust syntax to target syntax
        let converted = self.convert(type_param);
        format!("{}{}{}{}", name, self.open, converted, self.close)
    }

    /// Convert a type string from Rust generic syntax to target language syntax.
    ///
    /// For Python, wrapper newtypes like `Close<Cents>` are flattened to just `Cents`
    /// because Python type aliases can't be parameterized. This matches JS behavior.
    pub fn convert(&self, type_str: &str) -> String {
        // Flatten nested generics to innermost type (e.g., Close<Cents> -> Cents)
        // This is needed because wrapper types like Close, Open, High, Low are
        // just type aliases in generated code, not actual generic classes.
        extract_inner_type_recursive(type_str)
    }
}

/// Extract the innermost type from nested generics.
/// E.g., `Close<Cents>` -> `Cents`, `Foo<Bar<Baz>>` -> `Baz`
fn extract_inner_type_recursive(type_str: &str) -> String {
    if let Some(start) = type_str.find('<')
        && let Some(end) = type_str.rfind('>')
    {
        let inner = &type_str[start + 1..end];
        return extract_inner_type_recursive(inner);
    }
    type_str.to_string()
}
