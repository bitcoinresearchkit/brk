//! Rust language syntax implementation.

use crate::{FieldNamePosition, GenericSyntax, LanguageSyntax, to_snake_case};

/// Rust-specific code generation syntax.
pub struct RustSyntax;

impl LanguageSyntax for RustSyntax {
    fn field_name(&self, name: &str) -> String {
        to_snake_case(name)
    }

    fn path_expr(&self, base_var: &str, suffix: &str) -> String {
        format!("format!(\"{{{}}}{}\")", base_var, suffix)
    }

    fn position_expr(&self, pos: &FieldNamePosition, _base_var: &str) -> String {
        match pos {
            FieldNamePosition::Append(s) => {
                // Use helper _m(&acc, suffix) to build metric name
                if let Some(suffix) = s.strip_prefix('_') {
                    format!("_m(&acc, \"{}\")", suffix)
                } else {
                    format!("format!(\"{{acc}}{}\")", s)
                }
            }
            FieldNamePosition::Prepend(s) => {
                // Handle empty acc case for prepend
                if let Some(prefix) = s.strip_suffix('_') {
                    format!(
                        "if acc.is_empty() {{ \"{prefix}\".to_string() }} else {{ format!(\"{s}{{acc}}\") }}",
                        prefix = prefix,
                        s = s
                    )
                } else {
                    format!("format!(\"{}{{acc}}\")", s)
                }
            }
            FieldNamePosition::Identity => "acc.clone()".to_string(),
            FieldNamePosition::SetBase(base) => format!("\"{}\".to_string()", base),
        }
    }

    fn constructor(&self, type_name: &str, path_expr: &str) -> String {
        format!("{}::new(client.clone(), {})", type_name, path_expr)
    }

    fn field_init(&self, indent: &str, name: &str, _type_ann: &str, value: &str) -> String {
        // Rust struct initialization; type is in struct definition, not in init
        format!("{}{}: {},", indent, name, value)
    }

    fn generic_syntax(&self) -> GenericSyntax {
        GenericSyntax::RUST
    }

    fn struct_header(&self, name: &str, generic_params: &str, doc: Option<&str>) -> String {
        let mut result = String::new();
        if let Some(doc) = doc {
            result.push_str(&format!("/// {}\n", doc));
        }
        result.push_str(&format!("pub struct {}{} {{\n", name, generic_params));
        result
    }

    fn struct_footer(&self) -> String {
        "}\n".to_string()
    }

    fn constructor_header(&self, params: &str) -> String {
        format!("    pub fn new({}) -> Self {{\n        Self {{\n", params)
    }

    fn constructor_footer(&self) -> String {
        "        }\n    }\n".to_string()
    }

    fn field_declaration(&self, indent: &str, name: &str, type_ann: &str) -> String {
        format!("{}pub {}: {},\n", indent, name, type_ann)
    }

    fn index_field_name(&self, index_name: &str) -> String {
        format!("by_{}", to_snake_case(index_name))
    }

    fn string_literal(&self, value: &str) -> String {
        format!("\"{}\".to_string()", value)
    }

    fn constructor_name(&self, type_name: &str) -> String {
        format!("{}::new", type_name)
    }
}
