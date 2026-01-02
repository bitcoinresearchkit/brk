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
    pub const PYTHON: Self = Self { open: '[', close: ']', default_type: "Any" };
    pub const JAVASCRIPT: Self = Self { open: '<', close: '>', default_type: "unknown" };
    pub const RUST: Self = Self { open: '<', close: '>', default_type: "_" };

    pub fn wrap(&self, name: &str, type_param: &str) -> String {
        format!("{}{}{}{}", name, self.open, type_param, self.close)
    }
}
