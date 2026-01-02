//! Language-specific syntax traits for code generation.
//!
//! This module defines the `LanguageSyntax` trait that abstracts over
//! language-specific code generation patterns, allowing shared generation
//! logic to work across Python, JavaScript, and Rust backends.

use crate::{FieldNamePosition, GenericSyntax};

/// Language-specific syntax for code generation.
///
/// Implementations of this trait provide the language-specific formatting
/// for generated client code. This allows the core generation logic to be
/// written once and reused across all supported languages.
pub trait LanguageSyntax {
    /// Convert a field name to the language's naming convention.
    ///
    /// - Python/Rust: `snake_case`
    /// - JavaScript: `camelCase`
    fn field_name(&self, name: &str) -> String;

    /// Format an interpolated path expression.
    ///
    /// # Arguments
    /// * `base_var` - The variable name to interpolate (e.g., "acc", "base_path")
    /// * `suffix` - The suffix to append (e.g., "_field_name")
    ///
    /// # Returns
    /// - Python: `f'{acc}_suffix'`
    /// - JavaScript: `` `${acc}_suffix` ``
    /// - Rust: `format!("{acc}_suffix")`
    fn path_expr(&self, base_var: &str, suffix: &str) -> String;

    /// Format a `FieldNamePosition` as a path expression.
    ///
    /// This handles the different name transformation patterns (append, prepend,
    /// identity, set_base) in a language-specific way.
    fn position_expr(&self, pos: &FieldNamePosition, base_var: &str) -> String;

    /// Generate a constructor call for patterns and accessors.
    ///
    /// - Python: `TypeName(client, path)`
    /// - JavaScript: `createTypeName(client, path)`
    /// - Rust: `TypeName::new(client.clone(), path)`
    fn constructor(&self, type_name: &str, path_expr: &str) -> String;

    /// Generate a field initialization line.
    ///
    /// # Arguments
    /// * `indent` - The indentation string
    /// * `name` - The field name (already converted to language convention)
    /// * `type_ann` - The type annotation (may be ignored by some languages)
    /// * `value` - The initialization value/expression
    ///
    /// # Returns
    /// - Python: `{indent}self.{name}: {type_ann} = {value}`
    /// - JavaScript: `{indent}{name}: {value},`
    /// - Rust: `{indent}{name}: {value},`
    fn field_init(&self, indent: &str, name: &str, type_ann: &str, value: &str) -> String;

    /// Get the generic type syntax for this language.
    ///
    /// - Python: `[T]` with default `Any`
    /// - JavaScript: `<T>` with default `unknown`
    /// - Rust: `<T>` with default `_`
    fn generic_syntax(&self) -> GenericSyntax;

    /// Generate a struct/class header.
    ///
    /// # Arguments
    /// * `name` - The type name
    /// * `generic_params` - Generic parameters (e.g., "<T>" or "[T]"), empty if none
    /// * `doc` - Optional documentation string
    fn struct_header(&self, name: &str, generic_params: &str, doc: Option<&str>) -> String;

    /// Generate a struct/class footer.
    fn struct_footer(&self) -> String;

    /// Generate a constructor/init method header.
    ///
    /// # Arguments
    /// * `params` - Constructor parameters (language-specific format)
    fn constructor_header(&self, params: &str) -> String;

    /// Generate a constructor/init method footer.
    fn constructor_footer(&self) -> String;

    /// Generate a field declaration (for struct body, not init).
    ///
    /// # Arguments
    /// * `indent` - The indentation string
    /// * `name` - The field name
    /// * `type_ann` - The type annotation
    fn field_declaration(&self, indent: &str, name: &str, type_ann: &str) -> String;

    /// Format an index field name from an Index.
    ///
    /// E.g., `by_date_height`, `by_date`, etc.
    fn index_field_name(&self, index_name: &str) -> String;

    /// Format a string literal.
    ///
    /// - Python/JavaScript: `'value'` (single quotes)
    /// - Rust: `"value"` (double quotes)
    fn string_literal(&self, value: &str) -> String;
}
