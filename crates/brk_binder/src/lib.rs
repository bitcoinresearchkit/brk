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
    let schemas = extract_schemas(openapi_json);

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
