# bitview_bindgen

Code generation for Bitview client libraries.

## What It Enables

Generate clients for Rust, CLI, JavaScript, Python, LLMs, and MCP from the OpenAPI
specification and series tree. Keeps every consumer in sync with available
series and API endpoints without manual maintenance.

## Key Features

- **Multi-client**: Generates Rust, CLI, JavaScript, Python, and LLM clients
- **MCP catalog**: Generates the MCP tool manifest from the same OpenAPI operations
- **OpenAPI-driven**: Extracts endpoints and schemas from the OpenAPI spec
- **Series catalog**: Includes all series IDs and their supported indexes
- **Type definitions**: Generates types/interfaces from JSON Schema
- **Selective output**: Generate only the clients you need

## Core API

```rust,ignore
use bitview_bindgen::{generate_clients, ClientOutputPaths};

let paths = ClientOutputPaths::new()
    .rust("crates/bitview_client/src/generated.rs")
    .cli("crates/bitview_cli/src/generated.rs")
    .javascript("modules/bitview-client/index.js")
    .python("packages/bitview_client/bitview_client/__init__.py")
    .llm("website")
    .llm("website_next")
    .llm_manifest("crates/bitview_mcp/generated/manifest.json");

generate_clients(&catalog, &openapi_json, &paths)?;
```

## Generated Clients

| Language | Contents |
|----------|----------|
| Rust | Typed API client using `brk_types` and `bitview_types`, series catalog |
| CLI | Command catalog for every non-deprecated OpenAPI operation |
| JavaScript | ES module with JSDoc types, series catalog, fetch helpers |
| Python | Typed client with dataclasses, series catalog |
| LLM/MCP | Plain-text API references and the MCP tool manifest |

Language clients include:
- All REST API endpoints as typed functions
- Complete series catalog with index information
- Type definitions for request/response schemas

The LLM client emits the standard discovery files and links to the live
OpenAPI and series endpoints instead of duplicating their catalogs.
The official generated MCP catalog is served through the stateless, read-only
endpoint at [mcp.bitview.space](https://mcp.bitview.space/).

## Built On

The Rust client is generated directly from source-declared catalog families,
field-type relationships, and exact series names. It does not infer record
families or reconstruct series names from prefixes and suffixes. Structured
leaf types such as `ByTerm<Sats>` are preserved. Generated record names use
`Catalog` plus their Rust source type; these replace the old inferred helper
types. JavaScript and Python still use the existing inference pipeline.

Regenerate or verify only Rust outputs (client and CLI):

```sh
cargo bindgen -- --rust
cargo bindgen -- --rust --check
```

- `bitview_catalog::TreeNode` for the series catalog
- `bitview_types` and `brk_types` for type schemas

The generator consumes metadata only; it does not depend on the query runtime.
The `bitviewd` generation command still imports plugins into temporary databases
to obtain that catalog. Removing this initialization is a separate change.
