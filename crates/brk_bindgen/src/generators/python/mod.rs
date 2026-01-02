//! Python client generation.
//!
//! This module generates a Python client with type hints for the BRK API.

mod api;
mod client;
mod tree;
mod types;

use std::{fmt::Write, fs, io, path::Path};

use crate::{ClientMetadata, Endpoint, TypeSchemas};

/// Generate Python client from metadata and OpenAPI endpoints.
///
/// `output_path` is the full path to the output file (e.g., "packages/brk_client/__init__.py").
pub fn generate_python_client(
    metadata: &ClientMetadata,
    endpoints: &[Endpoint],
    schemas: &TypeSchemas,
    output_path: &Path,
) -> io::Result<()> {
    let mut output = String::new();

    writeln!(output, "# Auto-generated BRK Python client").unwrap();
    writeln!(output, "# Do not edit manually\n").unwrap();
    writeln!(output, "from __future__ import annotations").unwrap();
    writeln!(
        output,
        "from typing import TypeVar, Generic, Any, Optional, List, Literal, TypedDict, Final, Union, Protocol"
    )
    .unwrap();
    writeln!(output, "import httpx\n").unwrap();
    writeln!(output, "T = TypeVar('T')\n").unwrap();

    types::generate_type_definitions(&mut output, schemas);
    client::generate_base_client(&mut output);
    client::generate_endpoint_class(&mut output);
    client::generate_index_accessors(&mut output, &metadata.index_set_patterns);
    client::generate_structural_patterns(&mut output, &metadata.structural_patterns, metadata);
    tree::generate_tree_classes(&mut output, &metadata.catalog, metadata);
    api::generate_main_client(&mut output, endpoints);

    fs::write(output_path, output)?;

    Ok(())
}
