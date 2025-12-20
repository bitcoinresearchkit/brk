mod javascript;
mod openapi;
mod python;
mod rust;
mod types;

pub use javascript::*;
pub use openapi::*;
pub use python::*;
pub use rust::*;
pub use types::*;

use brk_query::Vecs;
use std::io;
use std::path::Path;

/// Generate all client libraries from the query vecs and OpenAPI JSON
pub fn generate_clients(vecs: &Vecs, openapi_json: &str, output_dir: &Path) -> io::Result<()> {
    let metadata = ClientMetadata::from_vecs(vecs);

    // Parse OpenAPI spec from JSON
    let spec = parse_openapi_json(openapi_json)?;
    let endpoints = extract_endpoints(&spec);

    // Generate Rust client
    let rust_path = output_dir.join("rust");
    std::fs::create_dir_all(&rust_path)?;
    generate_rust_client(&metadata, &endpoints, &rust_path)?;

    // Generate JavaScript client
    let js_path = output_dir.join("javascript");
    std::fs::create_dir_all(&js_path)?;
    generate_javascript_client(&metadata, &endpoints, &js_path)?;

    // Generate Python client
    let python_path = output_dir.join("python");
    std::fs::create_dir_all(&python_path)?;
    generate_python_client(&metadata, &endpoints, &python_path)?;

    Ok(())
}
