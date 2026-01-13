//! Rust language syntax implementation.

use crate::{GenericSyntax, LanguageSyntax, to_snake_case};

/// Rust-specific code generation syntax.
pub struct RustSyntax;

impl LanguageSyntax for RustSyntax {
    fn field_name(&self, name: &str) -> String {
        to_snake_case(name)
    }

    fn path_expr(&self, base_var: &str, suffix: &str) -> String {
        format!("format!(\"{{{}}}{}\")", base_var, suffix)
    }

    fn suffix_expr(&self, acc_var: &str, relative: &str) -> String {
        if relative.is_empty() {
            // Identity: just return acc
            format!("{}.clone()", acc_var)
        } else {
            // _m(&acc, relative) -> if acc.is_empty() { relative } else { format!("{acc}_{relative}") }
            format!("_m(&{}, \"{}\")", acc_var, relative)
        }
    }

    fn prefix_expr(&self, prefix: &str, acc_var: &str) -> String {
        if prefix.is_empty() {
            // Identity: just return acc
            format!("{}.clone()", acc_var)
        } else {
            // _p(prefix, &acc) -> if acc.is_empty() { prefix_base } else { format!("{prefix}{acc}") }
            let prefix_base = prefix.trim_end_matches('_');
            format!("_p(\"{}\", &{})", prefix_base, acc_var)
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
