//! JavaScript language syntax implementation.

use crate::{GenericSyntax, LanguageSyntax, to_camel_case, to_pascal_case};

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

    fn suffix_expr(&self, acc_var: &str, relative: &str) -> String {
        let var_name = to_camel_case(acc_var);
        if relative.is_empty() {
            // Identity: just return acc
            var_name
        } else {
            // _m(acc, relative) -> acc ? `${acc}_relative` : 'relative'
            format!("_m({}, '{}')", var_name, relative)
        }
    }

    fn prefix_expr(&self, prefix: &str, acc_var: &str) -> String {
        let var_name = to_camel_case(acc_var);
        if prefix.is_empty() {
            // Identity: just return acc
            var_name
        } else {
            // _p(prefix, acc) -> acc ? `${prefix}${acc}` : 'prefix_without_underscore'
            let prefix_base = prefix.trim_end_matches('_');
            format!("_p('{}', {})", prefix_base, var_name)
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

    fn constructor_name(&self, type_name: &str) -> String {
        format!("create{}", type_name)
    }
}
