//! Python language syntax implementation.

use crate::{FieldNamePosition, GenericSyntax, LanguageSyntax, escape_python_keyword, to_snake_case};

/// Python-specific code generation syntax.
pub struct PythonSyntax;

impl LanguageSyntax for PythonSyntax {
    fn field_name(&self, name: &str) -> String {
        escape_python_keyword(&to_snake_case(name))
    }

    fn path_expr(&self, base_var: &str, suffix: &str) -> String {
        format!("f'{{{}}}{}'", base_var, suffix)
    }

    fn position_expr(&self, pos: &FieldNamePosition, base_var: &str) -> String {
        match pos {
            FieldNamePosition::Append(s) => {
                // Use helper _m(acc, suffix) to build metric name
                if let Some(suffix) = s.strip_prefix('_') {
                    format!("_m({}, '{}')", base_var, suffix)
                } else {
                    format!("f'{{{}}}{}'", base_var, s)
                }
            }
            FieldNamePosition::Prepend(s) => {
                // Handle empty acc case for prepend
                // Want to produce: (f'prefix_{acc}' if acc else 'prefix')
                if let Some(prefix) = s.strip_suffix('_') {
                    format!(
                        "(f'{}{{{}}}' if {} else '{}')",
                        s, base_var, base_var, prefix
                    )
                } else {
                    format!("f'{}{{{}}}'" , s, base_var)
                }
            }
            FieldNamePosition::Identity => base_var.to_string(),
            FieldNamePosition::SetBase(s) => format!("'{}'", s),
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
}
