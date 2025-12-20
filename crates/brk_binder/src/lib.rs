use std::{fs::create_dir_all, io, path::Path};

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
/// Also collects definitions from schemars-generated schemas (for referenced types).
fn collect_leaf_type_schemas(node: &TreeNode, schemas: &mut TypeSchemas) {
    match node {
        TreeNode::Leaf(leaf) => {
            // Extract the inner type name (e.g., "Dollars" from "Close<Dollars>")
            let type_name = extract_inner_type(leaf.value_type());

            // Only add if not already present (OpenAPI schemas take precedence)
            if !schemas.contains_key(&type_name) {
                // The leaf schema is the schemars-generated JSON schema
                schemas.insert(type_name, leaf.schema.clone());
            }

            // Also collect any definitions from the schema (schemars puts referenced types here)
            collect_schema_definitions(&leaf.schema, schemas);
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
