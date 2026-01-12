//! Code generators for client libraries.
//!
//! Each language has its own submodule with focused files:
//! - `types.rs` - Type definitions
//! - `client.rs` - Base client and pattern factories
//! - `tree.rs` - Tree structure generation
//! - `api.rs` - API method generation
//! - `mod.rs` - Entry point

use std::fmt::Write;

pub mod javascript;
pub mod python;
pub mod rust;

pub use javascript::generate_javascript_client;
pub use python::generate_python_client;
pub use rust::generate_rust_client;

/// Types that are manually defined as generics in client code, not from schema.
pub const MANUAL_GENERIC_TYPES: &[&str] = &["MetricData", "MetricEndpoint"];

/// Write a multi-line description with the given prefix for each line.
/// `empty_prefix` is used for blank lines (e.g., "   *" without trailing space).
pub fn write_description(output: &mut String, desc: &str, prefix: &str, empty_prefix: &str) {
    for line in desc.lines() {
        if line.is_empty() {
            writeln!(output, "{}", empty_prefix).unwrap();
        } else {
            writeln!(output, "{}{}", prefix, line).unwrap();
        }
    }
}
