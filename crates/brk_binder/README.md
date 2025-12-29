# brk_binder

Code generation for BRK client libraries.

## What It Enables

Generate typed client libraries for Rust, JavaScript/TypeScript, and Python from the OpenAPI specification. Keeps frontend code in sync with available metrics and API endpoints without manual maintenance.

## Key Features

- **Multi-language**: Generates Rust, JavaScript, and Python clients
- **OpenAPI-driven**: Extracts endpoints and schemas from the OpenAPI spec
- **Metric catalog**: Includes all metric IDs and their supported indexes
- **Type definitions**: Generates types/interfaces from JSON Schema
- **Selective output**: Generate only the languages you need

## Core API

```rust,ignore
use brk_binder::{generate_clients, ClientOutputPaths};

let paths = ClientOutputPaths::new()
    .rust("crates/brk_client/src/lib.rs")
    .javascript("modules/brk-client/index.js")
    .python("packages/brk_client/brk_client/__init__.py");

generate_clients(&vecs, &openapi_json, &paths)?;
```

## Generated Clients

| Language | Contents |
|----------|----------|
| Rust | Typed API client using `brk_types`, metric catalog |
| JavaScript | ES module with JSDoc types, metric catalog, fetch helpers |
| Python | Typed client with dataclasses, metric catalog |

Each client includes:
- All REST API endpoints as typed functions
- Complete metric catalog with index information
- Type definitions for request/response schemas

## Built On

- `brk_query` for metric enumeration
- `brk_types` for type schemas
