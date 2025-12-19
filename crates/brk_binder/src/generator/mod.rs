mod javascript;
mod python;
mod rust;
mod types;

pub use javascript::generate_javascript_client;
pub use python::generate_python_client;
pub use rust::generate_rust_client;
pub use types::*;

use brk_query::Vecs;
use std::io;
use std::path::Path;

/// Generate all client libraries from the query vecs
pub fn generate_clients(vecs: &Vecs, output_dir: &Path) -> io::Result<()> {
    let metadata = ClientMetadata::from_vecs(vecs);

    // Generate Rust client
    let rust_path = output_dir.join("rust");
    std::fs::create_dir_all(&rust_path)?;
    generate_rust_client(&metadata, &rust_path)?;

    // Generate JavaScript client
    let js_path = output_dir.join("javascript");
    std::fs::create_dir_all(&js_path)?;
    generate_javascript_client(&metadata, &js_path)?;

    // Generate Python client
    let python_path = output_dir.join("python");
    std::fs::create_dir_all(&python_path)?;
    generate_python_client(&metadata, &python_path)?;

    Ok(())
}
