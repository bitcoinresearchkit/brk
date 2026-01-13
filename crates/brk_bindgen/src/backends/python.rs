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

    fn string_literal(&self, value: &str) -> String {
        format!("'{}'", value)
    }

    fn constructor_name(&self, type_name: &str) -> String {
        type_name.to_string()
    }
}
