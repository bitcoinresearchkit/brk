//! Code generators for client libraries.
//!
//! Each language has its own submodule with focused files:
//! - `types.rs` - Type definitions
//! - `client.rs` - Base client and pattern factories
//! - `tree.rs` - Tree structure generation
//! - `api.rs` - API method generation
//! - `mod.rs` - Entry point

pub mod javascript;
pub mod python;
pub mod rust;

pub use javascript::generate_javascript_client;
pub use python::generate_python_client;
pub use rust::generate_rust_client;
