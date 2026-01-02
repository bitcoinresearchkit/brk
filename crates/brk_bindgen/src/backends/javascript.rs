//! JavaScript language syntax implementation.

use crate::{FieldNamePosition, GenericSyntax, LanguageSyntax, to_camel_case, to_pascal_case};

/// JavaScript-specific code generation syntax.
pub struct JavaScriptSyntax;

impl LanguageSyntax for JavaScriptSyntax {
    fn field_name(&self, name: &str) -> String {
        to_camel_case(name)
    }

    fn path_expr(&self, base_var: &str, suffix: &str) -> String {
        // Convert base_var to camelCase for JavaScript
        let var_name = to_camel_case(base_var);
        format!("`${{{}}}{}`", var_name, suffix)
    }

    fn position_expr(&self, pos: &FieldNamePosition, base_var: &str) -> String {
        // Convert base_var to camelCase for JavaScript
        let var_name = to_camel_case(base_var);
        match pos {
            FieldNamePosition::Append(s) => {
                // Use helper _m(acc, suffix) to build metric name
                // e.g., _m(acc, "cap") produces: acc ? `${acc}_cap` : 'cap'
                if let Some(suffix) = s.strip_prefix('_') {
                    format!("_m({}, '{}')", var_name, suffix)
                } else {
                    format!("`${{{}}}{}`", var_name, s)
                }
            }
            FieldNamePosition::Prepend(s) => {
                // Handle empty acc case for prepend
                if let Some(prefix) = s.strip_suffix('_') {
                    format!(
                        "({} ? `{}${{{}}}` : '{}')",
                        var_name, s, var_name, prefix
                    )
                } else {
                    format!("`{}${{{}}}`", s, var_name)
                }
            }
            FieldNamePosition::Identity => var_name,
            FieldNamePosition::SetBase(s) => format!("'{}'", s),
        }
    }

    fn constructor(&self, type_name: &str, path_expr: &str) -> String {
        format!("create{}(client, {})", type_name, path_expr)
    }

    fn field_init(&self, indent: &str, name: &str, _type_ann: &str, value: &str) -> String {
        // JavaScript uses object literal syntax; type is in JSDoc, not in assignment
        format!("{}{}: {},", indent, name, value)
    }

    fn generic_syntax(&self) -> GenericSyntax {
        GenericSyntax::JAVASCRIPT
    }

    fn struct_header(&self, name: &str, generic_params: &str, doc: Option<&str>) -> String {
        let mut result = String::new();
        if let Some(doc) = doc {
            result.push_str(&format!("/** {} */\n", doc));
        }
        // JavaScript uses factory functions that return object literals
        result.push_str(&format!(
            "function create{}{}(client, basePath) {{\n  return {{\n",
            name, generic_params
        ));
        result
    }

    fn struct_footer(&self) -> String {
        "  };\n}\n".to_string()
    }

    fn constructor_header(&self, _params: &str) -> String {
        // JavaScript factory functions don't have a separate constructor
        String::new()
    }

    fn constructor_footer(&self) -> String {
        String::new()
    }

    fn field_declaration(&self, indent: &str, _name: &str, type_ann: &str) -> String {
        // JSDoc property declaration
        format!("{}/** @type {{{}}} */\n", indent, type_ann)
    }

    fn index_field_name(&self, index_name: &str) -> String {
        format!("by{}", to_pascal_case(index_name))
    }

    fn string_literal(&self, value: &str) -> String {
        format!("'{}'", value)
    }
}
