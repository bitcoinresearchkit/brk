# brk_binder

Code generation for BRK client libraries.

## What It Enables

Generate typed metric catalogs and constants for JavaScript/TypeScript clients. Keeps frontend code in sync with available metrics without manual maintenance.

## Key Features

- **Metric catalog**: Generates `metrics.js` with all metric IDs and their supported indexes
- **Compression**: Metric names compressed via word-to-base62 mapping for smaller bundles
- **Mining pools**: Generates `pools.js` with pool ID to name mapping
- **Version sync**: Generates `version.js` matching server version

## Core API

```rust
generate_js_files(&query, &modules_path)?;
```

## Generated Files

```
modules/brk-client/generated/
├── version.js    # export const VERSION = "vX.Y.Z"
├── metrics.js    # INDEXES, COMPRESSED_METRIC_TO_INDEXES
└── pools.js      # POOL_ID_TO_POOL_NAME
```

## Metric Compression

To minimize bundle size, metric names are compressed:
1. Extract all words from metric names
2. Sort by frequency
3. Map to base52 codes (A-Z, a-z)
4. Store compressed metric → index group mapping

## Built On

- `brk_query` for metric enumeration
- `brk_types` for mining pool data
