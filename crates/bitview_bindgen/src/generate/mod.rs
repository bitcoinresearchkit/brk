//! Shared code generation logic.
//!
//! This module contains generation functions that are parameterized by
//! the `LanguageSyntax` trait, allowing them to work across all supported
//! language backends.

mod child_context;
mod constants;
mod field_parts;
mod fields;
mod tree;

pub use child_context::*;
pub use constants::*;
pub use field_parts::*;
pub use fields::*;
pub use tree::*;
