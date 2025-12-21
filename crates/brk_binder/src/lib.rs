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
//!    - Rust: Uses `brk_types` directly, generates structs with lifetimes
//!    - JavaScript: Generates JSDoc-typed ES modules with factory functions
//!    - Python: Generates typed classes with TypedDict and Generic support

use std::{collections::btree_map::Entry, fs::create_dir_all, io, path::Path};

use brk_query::Vecs;

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

/// Generate all client libraries from the query vecs and OpenAPI JSON
pub fn generate_clients(vecs: &Vecs, openapi_json: &str, output_dir: &Path) -> io::Result<()> {
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
    let rust_path = output_dir.join("rust");
    create_dir_all(&rust_path)?;
    generate_rust_client(&metadata, &endpoints, &rust_path)?;

    // Generate JavaScript client (needs schemas for type definitions)
    let js_path = output_dir.join("javascript");
    create_dir_all(&js_path)?;
    generate_javascript_client(&metadata, &endpoints, &schemas, &js_path)?;

    // Generate Python client (needs schemas for type definitions)
    let python_path = output_dir.join("python");
    create_dir_all(&python_path)?;
    generate_python_client(&metadata, &endpoints, &schemas, &python_path)?;

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
