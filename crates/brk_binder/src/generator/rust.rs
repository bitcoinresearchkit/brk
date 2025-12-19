use std::io;
use std::path::Path;

use super::ClientMetadata;

/// Generate Rust client from metadata
pub fn generate_rust_client(_metadata: &ClientMetadata, _output_dir: &Path) -> io::Result<()> {
    // TODO: Implement Rust client generation
    Ok(())
}
