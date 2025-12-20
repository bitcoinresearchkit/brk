# brk_binder Design Document

## Goal

Generate typed API clients for **Rust, JavaScript, and Python** with:
- **Discoverability**: Full IDE autocomplete for 20k+ metrics
- **Ease of use**: Fluent API with `.fetch()` on each metric node

## Current State

### What's Working ✅

1. **JS + JSDoc generator**: Generates `client.js` with full JSDoc type annotations
2. **Python generator**: Generates `client.py` with type hints and httpx
3. **Rust generator**: Generates `client.rs` with strong typing and reqwest
4. **schemars integration**: JSON schemas embedded in `MetricLeafWithSchema` for type info
5. **Tree navigation**: `client.tree.blocks.difficulty.fetch()` pattern
6. **OpenAPI integration**: All GET endpoints generate typed methods
7. **Server integration**: brk_server calls brk_binder on startup (when clients/ dir exists)

### Generated Output

When `crates/brk_binder/clients/` directory exists, running the server generates:

```
crates/brk_binder/clients/
├── javascript/
│   └── client.js      # JS + JSDoc with tree + API methods
├── python/
│   └── client.py      # Python with type hints + httpx
└── rust/
    └── client.rs      # Rust with reqwest + strong typing
```

## Target Architecture

### Input Sources

```
┌─────────────────────────────────────────────────────────────┐
│                      Input Sources                          │
├─────────────────────────────────────────────────────────────┤
│  1. OpenAPI spec (from aide) - endpoint definitions         │
│  2. brk_query catalog - metric tree structure               │
│  3. brk_types - Rust types for responses (Rust client only) │
└─────────────────────────────────────────────────────────────┘
```

### Output: Fluent Client

```javascript
// JavaScript (with JSDoc for IDE support)
const client = new BrkClient("http://localhost:3000");
const data = await client.tree.supply.active.by_date.fetch();
//                        ^^^^ autocomplete all the way down
```

```python
# Python
client = BrkClient("http://localhost:3000")
data = client.tree.supply.active.by_date.fetch()
```

```rust
// Rust
let client = BrkClient::new("http://localhost:3000")?;
let data = client.tree().supply.active.by_date.fetch()?;
```

## Implementation Details

### Smart Metric Nodes

Each tree leaf becomes a "smart node" holding a client reference:

```javascript
// JavaScript + JSDoc
/**
 * Metric node with fetch capability
 * @template T
 */
class MetricNode {
  constructor(client, path) {
    this._client = client;
    this._path = path;
  }

  async fetch() {
    return this._client.get(this._path);
  }
}
```

```python
# Python
class MetricNode(Generic[T]):
    def __init__(self, client: BrkClientBase, path: str):
        self._client = client
        self._path = path

    def fetch(self) -> T:
        return self._client.get(self._path)
```

```rust
// Rust
pub struct MetricNode<'a, T> {
    client: &'a BrkClientBase,
    path: &'static str,
    _marker: PhantomData<T>,
}

impl<'a, T: DeserializeOwned> MetricNode<'a, T> {
    pub fn fetch(&self) -> Result<T> {
        self.client.get(self.path)
    }
}
```

### Pattern Reuse

To avoid 20k+ individual types, reuse structural patterns:

```rust
// Shared pattern for metrics with same index groupings
struct ByDateHeightMonth<T> {
    by_date: MetricNode<T>,
    by_height: MetricNode<T>,
    by_month: MetricNode<T>,
}

// Composed into full tree
struct Supply {
    active: ByDateHeightMonth<Vec<f64>>,
    total: ByDateHeightMonth<Vec<f64>>,
}
```

## Type Discovery Solution ✅ IMPLEMENTED

### The Problem

Type information was erased at runtime because metrics are stored as `&dyn AnyExportableVec` trait objects.

### The Solution

Use `std::any::type_name::<T>()` with caching to extract short type names.

#### Implementation (vecdb)

Added `short_type_name<T>()` helper and `value_type_to_string()` to `AnyVec` trait.

### Result

`brk_query` now exposes:

```rust
for (metric_name, index_to_vec) in &vecs.metric_to_index_to_vec {
    for (index, vec) in index_to_vec {
        println!("{} @ {} -> {}",
            metric_name,                    // "difficulty"
            vec.index_type_to_string(),     // "Height"
            vec.value_type_to_string(),     // "StoredF64"
        );
    }
}
```

## TreeNode Enhancement ✅ IMPLEMENTED

Changed `TreeNode::Leaf(String)` to `TreeNode::Leaf(MetricLeafWithSchema)` where:

```rust
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct MetricLeafWithSchema {
    #[serde(flatten)]
    pub leaf: MetricLeaf,
    #[serde(skip)]
    pub schema: serde_json::Value,  // JSON Schema from schemars
}
```

## OpenAPI Integration ✅ IMPLEMENTED

### Flow

1. brk_server creates OpenAPI spec via aide
2. On startup, serializes spec to JSON string
3. Passes JSON to `brk_binder::generate_clients()`
4. brk_binder parses with `oas3` crate (supports OpenAPI 3.1)
5. Generates typed methods for all GET endpoints

### Why oas3?

aide generates OpenAPI 3.1 specs. The `openapiv3` crate only supports 3.0.x.
The `oas3` crate supports OpenAPI 3.1.x parsing.

## Tasks

### Phase 0: Type Infrastructure ✅ COMPLETE

- [x] vecdb: Add `short_type_name<T>()` and `value_type_to_string()`
- [x] vecdb: Add optional `schemars` feature with `AnySchemaVec` trait
- [x] brk_types: Enhance `TreeNode::Leaf` to include `MetricLeafWithSchema`
- [x] brk_traversable: Update all `to_tree_node()` with schemars integration
- [x] brk_binder: Set up generator module structure

### Phase 1: JavaScript Client ✅ COMPLETE

- [x] Define `MetricNode` class with JSDoc generics
- [x] Define `BrkClient` with base HTTP functionality
- [x] Generate `client.js` with full JSDoc type annotations
- [x] Tree navigation: `client.tree.category.metric.fetch()`
- [x] API methods from OpenAPI endpoints

### Phase 2: OpenAPI Integration ✅ COMPLETE

- [x] Add `oas3` crate dependency (OpenAPI 3.1 support)
- [x] brk_server passes OpenAPI JSON to brk_binder on startup
- [x] Parse OpenAPI spec and extract endpoint definitions
- [x] Generate typed methods for each GET endpoint

### Phase 3: Python Client ✅ COMPLETE

- [x] Define `MetricNode` class with type hints
- [x] Define `BrkClient` with httpx
- [x] Generate typed methods from OpenAPI
- [x] Generate tree navigation

### Phase 4: Rust Client ✅ COMPLETE

- [x] Define `MetricNode<T>` struct with lifetimes
- [x] Define `BrkClient` with reqwest (blocking)
- [x] Generate tree navigation with proper lifetimes
- [x] Generate typed methods from OpenAPI

### Phase 5: Polish

- [x] Switch from `openapiv3` to `oas3` crate
- [ ] Error types per language
- [ ] Documentation generation
- [ ] Tests
- [ ] Example usage in each language
- [ ] Async Rust client variant

## File Structure

```
crates/brk_binder/
├── src/
│   ├── lib.rs
│   ├── js.rs              # JS constants generation (existing)
│   └── generator/
│       ├── mod.rs         # generate_clients() entry point
│       ├── types.rs       # ClientMetadata, MetricInfo, IndexPattern
│       ├── openapi.rs     # OpenAPI 3.1 spec parsing (oas3)
│       ├── javascript.rs  # JavaScript + JSDoc client ✅
│       ├── python.rs      # Python client ✅
│       └── rust.rs        # Rust client ✅
├── clients/               # Generated output (gitignored)
│   ├── javascript/
│   ├── python/
│   └── rust/
├── Cargo.toml
├── README.md
└── DESIGN.md

crates/brk_server/
└── src/
    ├── lib.rs             # Calls brk_binder::generate_clients() on startup
    └── api/
        └── openapi.rs     # create_openapi() for aide
```

## Dependencies

```toml
[dependencies]
brk_query = { workspace = true }
brk_types = { workspace = true }
oas3 = "0.20"                # OpenAPI 3.1 spec parsing
schemars = { workspace = true }
serde_json = { workspace = true }
```

## Usage

To generate clients:

```bash
# Create the output directory
mkdir -p crates/brk_binder/clients

# Run the server (generates clients on startup)
cargo run -p brk_server
```
