//! Client library generator for BRK.
//!
//! This crate generates typed client libraries in multiple languages (Rust, JavaScript, Python)
//! from the BRK metric catalog and OpenAPI specification.
//!
//! # Usage
//!
//! ```ignore
//! use brk_binder::generate_clients;
//! use brk_query::Vecs;
//! use std::path::Path;
//!
//! let vecs = Vecs::load("path/to/data")?;
//! let openapi_json = std::fs::read_to_string("openapi.json")?;
//! generate_clients(&vecs, &openapi_json, Path::new("output"))?;
//! ```
//!
//! # Architecture
//!
//! The generator works in several phases:
//!
//! 1. **Metadata extraction** - Analyzes the metric catalog tree to detect:
//!    - Structural patterns (repeated tree shapes)
//!    - Index set patterns (common index combinations)
//!    - Generic patterns (structures that differ only in value type)
//!
//! 2. **Schema collection** - Merges OpenAPI schemas with schemars-generated type schemas
//!
//! 3. **Code generation** - Produces language-specific clients:
//!    - Rust: Uses `brk_types` directly, generates structs with Arc-based sharing
//!    - JavaScript: Generates JSDoc-typed ES modules with factory functions
//!    - Python: Generates typed classes with TypedDict and Generic support

use std::{collections::btree_map::Entry, fs::create_dir_all, io, path::PathBuf};

use brk_query::Vecs;

/// Output path configuration for each language client.
///
/// Each path should be the full path to the output file, not just a directory.
/// Parent directories will be created automatically if they don't exist.
///
/// # Example
/// ```ignore
/// let paths = ClientOutputPaths::new()
///     .rust("crates/brk_client/src/lib.rs")
///     .javascript("modules/brk-client/index.js")
///     .python("packages/brk_client/__init__.py");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ClientOutputPaths {
    /// Full path to Rust client file (e.g., "crates/brk_client/src/lib.rs")
    pub rust: Option<PathBuf>,
    /// Full path to JavaScript client file (e.g., "modules/brk-client/index.js")
    pub javascript: Option<PathBuf>,
    /// Full path to Python client file (e.g., "packages/brk_client/__init__.py")
    pub python: Option<PathBuf>,
}

impl ClientOutputPaths {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rust(mut self, path: impl Into<PathBuf>) -> Self {
        self.rust = Some(path.into());
        self
    }

    pub fn javascript(mut self, path: impl Into<PathBuf>) -> Self {
        self.javascript = Some(path.into());
        self
    }

    pub fn python(mut self, path: impl Into<PathBuf>) -> Self {
        self.python = Some(path.into());
        self
    }
}

mod javascript;
mod js;
mod openapi;
mod python;
mod rust;
mod types;

pub use javascript::*;
pub use js::*;
pub use openapi::*;
pub use python::*;
pub use rust::*;
pub use types::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Generate all client libraries from the query vecs and OpenAPI JSON.
///
/// Uses `ClientOutputPaths` to specify the output file path for each language.
/// Only languages with a configured path will be generated.
///
/// # Example
/// ```ignore
/// let paths = ClientOutputPaths::new()
///     .rust("crates/brk_client/src/lib.rs")
///     .javascript("modules/brk-client/index.js")
///     .python("packages/brk_client/__init__.py");
///
/// generate_clients(&vecs, &openapi_json, &paths)?;
/// ```
pub fn generate_clients(
    vecs: &Vecs,
    openapi_json: &str,
    output_paths: &ClientOutputPaths,
) -> io::Result<()> {
    let metadata = ClientMetadata::from_vecs(vecs);

    // Parse OpenAPI spec
    let spec = parse_openapi_json(openapi_json)?;
    let endpoints = extract_endpoints(&spec);
    let mut schemas = extract_schemas(openapi_json);

    // Collect leaf type schemas from the catalog and merge into schemas
    collect_leaf_type_schemas(&metadata.catalog, &mut schemas);

    // Also collect definitions from all schemas (including OpenAPI schemas)
    // We need to do this after collecting leaf schemas so we process everything
    let schema_values: Vec<_> = schemas.values().cloned().collect();
    for schema in &schema_values {
        collect_schema_definitions(schema, &mut schemas);
    }

    // Generate Rust client (uses real brk_types, no schema conversion needed)
    if let Some(rust_path) = &output_paths.rust {
        if let Some(parent) = rust_path.parent() {
            create_dir_all(parent)?;
        }
        generate_rust_client(&metadata, &endpoints, rust_path)?;
    }

    // Generate JavaScript client (needs schemas for type definitions)
    if let Some(js_path) = &output_paths.javascript {
        if let Some(parent) = js_path.parent() {
            create_dir_all(parent)?;
        }
        generate_javascript_client(&metadata, &endpoints, &schemas, js_path)?;
    }

    // Generate Python client (needs schemas for type definitions)
    if let Some(python_path) = &output_paths.python {
        if let Some(parent) = python_path.parent() {
            create_dir_all(parent)?;
        }
        generate_python_client(&metadata, &endpoints, &schemas, python_path)?;
    }

    Ok(())
}

use brk_types::TreeNode;
use serde_json::Value;

/// Recursively collect leaf type schemas from the tree and add to schemas map.
/// Only adds schemas that aren't already present (OpenAPI schemas take precedence).
/// Collects definitions from schemars-generated schemas (for referenced types).
fn collect_leaf_type_schemas(node: &TreeNode, schemas: &mut TypeSchemas) {
    match node {
        TreeNode::Leaf(leaf) => {
            // Collect definitions from the schema (schemars puts type schemas here)
            // This includes the inner types like `Bitcoin` from `Close<Bitcoin>`
            collect_schema_definitions(&leaf.schema, schemas);

            // Get the type name for this leaf
            let type_name = extract_inner_type(leaf.value_type());

            if let Entry::Vacant(e) = schemas.entry(type_name) {
                // Unwrap single-element allOf
                let schema = unwrap_allof(&leaf.schema);

                // Add the schema if it's usable:
                // - Simple type (has "type")
                // - Object type with properties (complex types like OHLCCents, EmptyAddressData)
                // - Enum type (has "enum" or "oneOf")
                // - Or a $ref to another type
                let has_type = schema.get("type").is_some();
                let has_properties = schema.get("properties").is_some();
                let has_enum = schema.get("enum").is_some() || schema.get("oneOf").is_some();
                let is_ref = schema.get("$ref").is_some();

                if has_type || has_properties || has_enum || is_ref {
                    e.insert(schema.clone());
                }
            }
        }
        TreeNode::Branch(children) => {
            for child in children.values() {
                collect_leaf_type_schemas(child, schemas);
            }
        }
    }
}

/// Collect type definitions from schemars-generated schema's definitions section.
/// Schemars uses `definitions` or `$defs` to store referenced types.
fn collect_schema_definitions(schema: &Value, schemas: &mut TypeSchemas) {
    // Check for definitions (JSON Schema draft-07 style)
    if let Some(defs) = schema.get("definitions").and_then(|d| d.as_object()) {
        for (name, def_schema) in defs {
            // Use the definition name as-is (schemars names match $ref paths)
            if !schemas.contains_key(name) {
                schemas.insert(name.clone(), def_schema.clone());
            }
        }
    }

    // Check for $defs (JSON Schema draft 2019-09+ style)
    if let Some(defs) = schema.get("$defs").and_then(|d| d.as_object()) {
        for (name, def_schema) in defs {
            if !schemas.contains_key(name) {
                schemas.insert(name.clone(), def_schema.clone());
            }
        }
    }
}
