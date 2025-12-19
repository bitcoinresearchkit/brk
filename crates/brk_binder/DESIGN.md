# brk_binder Design Document

## Goal

Generate typed API clients for **Rust, TypeScript, and Python** with:
- **Discoverability**: Full IDE autocomplete for 20k+ metrics
- **Ease of use**: Fluent API with `.fetch()` on each metric node

## Current State

### What Exists

1. **`js.rs`**: Generates compressed metric catalogs for JS (constants only, no HTTP client)
2. **`tree.rs`**: (kept for reference, not compiled) Brainstorming output for pattern extraction
3. **`generator/`**: Module structure for client generation
   - `types.rs`: Intermediate representation (`ClientMetadata`, `MetricInfo`, `IndexPattern`)
   - `rust.rs`: Rust client generation (stub)
   - `typescript.rs`: TypeScript client generation (stub)
   - `python.rs`: Python client generation (stub)

### What's Missing

- HTTP client integration (`.fetch()` methods)
- OpenAPI as input source
- Rust client using `brk_types` instead of generating types
- Typed response types per metric

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

### Output: Fluent Client (Option B)

```typescript
// TypeScript
const client = new BrkClient("http://localhost:3000");
const data = await client.tree.supply.active.by_date.fetch();
//                        ^^^^ autocomplete all the way down
```

```python
# Python
client = BrkClient("http://localhost:3000")
data = await client.tree.supply.active.by_date.fetch()
```

```rust
// Rust
let client = BrkClient::new("http://localhost:3000");
let data = client.tree.supply.active.by_date.fetch().await?;
```

## Implementation Details

### Smart Metric Nodes

Each tree leaf becomes a "smart node" holding a client reference:

```typescript
// TypeScript
class MetricNode<T = unknown> {
  constructor(private client: BrkClient, private path: string) {}
  async fetch(): Promise<T> {
    return this.client.get<T>(this.path);
  }
}
```

```python
# Python
class MetricNode(Generic[T]):
    def __init__(self, client: BrkClient, path: str):
        self._client = client
        self._path = path

    async def fetch(self) -> T:
        return await self._client.get(self._path)
```

```rust
// Rust
pub struct MetricNode<T> {
    client: Arc<BrkClient>,
    path: &'static str,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> MetricNode<T> {
    pub async fn fetch(&self) -> Result<T, BrkError> {
        self.client.get(&self.path).await
    }
}
```

### Pattern Reuse (from tree.rs)

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

### Rust Client: Using brk_types

The Rust client should import `brk_types` rather than generating duplicate types:

```rust
use brk_types::{Height, Sats, DateIndex, ...};

// Response types come from brk_types
pub struct MetricNode<T: brk_types::Metric> { ... }
```

## Type Discovery Solution ✅ IMPLEMENTED

### The Problem

Type information was erased at runtime because metrics are stored as `&dyn AnyExportableVec` trait objects.

### The Solution

Use `std::any::type_name::<T>()` with caching to extract short type names.

> **Note**: Unlike `PrintableIndex` which needs `to_possible_strings()` for parsing from
> multiple string representations, for values we only need output, so `type_name` suffices.

#### Implementation (vecdb)

Added `short_type_name<T>()` helper in `traits/printable.rs`:

```rust
pub fn short_type_name<T: 'static>() -> &'static str {
    static CACHE: OnceLock<Mutex<HashMap<&'static str, &'static str>>> = OnceLock::new();

    let full: &'static str = std::any::type_name::<T>();
    // ... caching logic, extracts "Sats" from "brk_types::sats::Sats"
}
```

Added `value_type_to_string()` to `AnyVec` trait in `traits/any.rs`:

```rust
pub trait AnyVec: Send + Sync {
    // ... existing methods
    fn value_type_to_string(&self) -> &'static str;
}
```

Implemented in all vec variants:
- `variants/eager/mod.rs`
- `variants/lazy/from1/mod.rs`, `from2/mod.rs`, `from3/mod.rs`
- `variants/raw/inner/mod.rs`
- `variants/compressed/inner/mod.rs`
- `variants/macros.rs` (for wrapper types)

```rust
fn value_type_to_string(&self) -> &'static str {
    short_type_name::<V::T>()
}
```

**No changes needed to brk_types** - works automatically for all types.

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

This enables fully typed client generation.

## TreeNode Enhancement ✅ IMPLEMENTED

### The Problem

`TreeNode::Leaf` originally held just a `String` (the metric name), losing type and index information.

### The Solution

Changed `TreeNode::Leaf(String)` to `TreeNode::Leaf(MetricLeaf)` where:

```rust
#[derive(Debug, Clone, Serialize, PartialEq, Eq, JsonSchema)]
pub struct MetricLeaf {
    pub name: String,
    pub value_type: String,
    pub indexes: BTreeSet<Index>,
}
```

#### Implementation

**brk_types/src/treenode.rs**:
- Added `MetricLeaf` struct with `name`, `value_type`, and `indexes`
- Added `merge_indexes()` method to union indexes when flattening tree
- Updated `TreeNode` enum to use `Leaf(MetricLeaf)`
- Updated merge logic to handle index merging

**brk_traversable/src/lib.rs**:
- Added `make_leaf<I, V>()` helper that creates `MetricLeaf` with proper fields
- Updated all `Traversable::to_tree_node()` implementations

### Result

The catalog tree now includes full type information at each leaf:

```rust
TreeNode::Leaf(MetricLeaf {
    name: "difficulty".to_string(),
    value_type: "StoredF64".to_string(),
    indexes: btreeset![Index::Height, Index::Date],
})
```

When trees are merged/simplified, indexes are unioned together.

### 2. Async Runtime

- TypeScript: Native `Promise`
- Python: `asyncio` or sync variant?
- Rust: `tokio` assumed, or feature-flag for other runtimes?

### 3. Error Handling

- HTTP errors (4xx, 5xx)
- Deserialization errors
- Network errors
- Should errors be typed per language?

### 4. Additional Client Features

- Request timeout configuration
- Retry logic
- Rate limiting
- Caching
- Batch requests (fetch multiple metrics at once)

## Tasks

### Phase 0: Type Infrastructure ✅ COMPLETE

- [x] **vecdb**: Add `short_type_name<T>()` helper in `traits/printable.rs`
- [x] **vecdb**: Add `value_type_to_string()` to `AnyVec` trait
- [x] **vecdb**: Implement in all vec variants (eager, lazy, raw, compressed, macros)
- [x] **brk_types**: Enhance `TreeNode::Leaf` to include `MetricLeaf` with name, value_type, indexes
- [x] **brk_traversable**: Update all `to_tree_node()` implementations to populate `MetricLeaf`
- [x] **brk_query**: Export `Vecs` publicly for client generation
- [x] **brk_binder**: Set up generator module structure (types, rust, typescript, python stubs)
- [x] **brk**: Verify compilation

### Phase 1: Client Foundation

- [ ] Define `MetricNode<T>` struct/class for each language
- [ ] Define `BrkClient` with base HTTP functionality
- [ ] Implement `ClientMetadata::from_vecs()` to extract metadata from `brk_query::Vecs`
- [ ] Client holds reference, nodes borrow it

### Phase 2: Type-Aware Generation

- [ ] Create type mapping: `value_type_string → Rust type / TS type / Python type`
- [ ] Generate typed `MetricNode<T>` with correct `T` per metric
- [ ] For Rust: import from `brk_types` instead of generating

### Phase 3: OpenAPI Integration

- [ ] Parse OpenAPI spec with `openapiv3` crate
- [ ] Extract non-metric endpoint definitions
- [ ] Generate methods for health, info, catalog, etc.

### Phase 4: Polish

- [ ] Error types per language
- [ ] Documentation generation
- [ ] Tests
- [ ] Example usage in each language

## File Structure

```
crates/brk_binder/
├── src/
│   ├── lib.rs
│   ├── js.rs           # JS constants generation (existing)
│   ├── tree.rs         # Pattern extraction (reference only, not compiled)
│   └── generator/
│       ├── mod.rs
│       ├── types.rs    # ClientMetadata, MetricInfo, IndexPattern
│       ├── rust.rs     # Rust client generation
│       ├── typescript.rs
│       └── python.rs
├── Cargo.toml
├── README.md
└── DESIGN.md           # This file
```

## Dependencies (Proposed)

```toml
[dependencies]
openapiv3 = "2"           # OpenAPI parsing
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"        # If parsing YAML specs
tera = "1"                # Optional: templating
```
