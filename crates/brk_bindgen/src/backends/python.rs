//! Python language syntax implementation.

use crate::{GenericSyntax, LanguageSyntax, escape_python_keyword, to_snake_case};

/// Python-specific code generation syntax.
pub struct PythonSyntax;

impl LanguageSyntax for PythonSyntax {
    fn field_name(&self, name: &str) -> String {
        escape_python_keyword(&to_snake_case(name))
    }

    fn path_expr(&self, base_var: &str, suffix: &str) -> String {
        format!("f'{{{}}}{}'", base_var, suffix)
    }

    fn suffix_expr(&self, acc_var: &str, relative: &str) -> String {
        if relative.is_empty() {
            // Identity: just return acc
            acc_var.to_string()
        } else {
            // _m(acc, relative) -> f'{acc}_{relative}' if acc else 'relative'
            format!("_m({}, '{}')", acc_var, relative)
        }
    }

    fn prefix_expr(&self, prefix: &str, acc_var: &str) -> String {
        if prefix.is_empty() {
            // Identity: just return acc
            acc_var.to_string()
        } else {
            // _p(prefix, acc) -> f'{prefix}{acc}' if acc else 'prefix_base'
            let prefix_base = prefix.trim_end_matches('_');
            format!("_p('{}', {})", prefix_base, acc_var)
        }
    }

    fn constructor(&self, type_name: &str, path_expr: &str) -> String {
        format!("{}(client, {})", type_name, path_expr)
    }

    fn field_init(&self, indent: &str, name: &str, type_ann: &str, value: &str) -> String {
        format!("{}self.{}: {} = {}", indent, name, type_ann, value)
    }

    fn generic_syntax(&self) -> GenericSyntax {
        GenericSyntax::PYTHON
    }

    fn struct_header(&self, name: &str, generic_params: &str, doc: Option<&str>) -> String {
        let mut result = format!("class {}{}:\n", name, generic_params);
        if let Some(doc) = doc {
            result.push_str(&format!("    \"\"\"{}\"\"\"\n", doc));
        }
        result
    }

    fn struct_footer(&self) -> String {
        String::new()
    }

    fn constructor_header(&self, params: &str) -> String {
        format!("    def __init__(self{}) -> None:\n", params)
    }

    fn constructor_footer(&self) -> String {
        String::new()
    }

    fn field_declaration(&self, _indent: &str, _name: &str, _type_ann: &str) -> String {
        // Python uses __init__ for field declarations, so this is a no-op
        String::new()
    }

    fn index_field_name(&self, index_name: &str) -> String {
        to_snake_case(index_name)
    }

    fn string_literal(&self, value: &str) -> String {
        format!("'{}'", value)
    }

    fn constructor_name(&self, type_name: &str) -> String {
        type_name.to_string()
    }
}
