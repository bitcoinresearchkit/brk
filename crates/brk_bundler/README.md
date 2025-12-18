# brk_bundler

JavaScript bundling with watch mode for BRK web interfaces.

## What It Enables

Bundle and minify JavaScript modules using Rolldown, with file watching for development. Handles module copying, source map generation, and cache-busting via hashed filenames.

## Key Features

- **Rolldown integration**: Fast Rust-based bundler with tree-shaking and minification
- **Watch mode**: Rebuilds on file changes with live module syncing
- **Source maps**: Full debugging support in production builds
- **Cache busting**: Hashes main bundle filename, updates HTML references automatically
- **Service worker versioning**: Injects package version into service worker files

## Core API

```rust,ignore
// One-shot build
let dist = bundle(modules_path, websites_path, "src", false).await?;

// Watch mode for development
bundle(modules_path, websites_path, "src", true).await?;
```

## Build Pipeline

1. Copy shared modules to source scripts directory
2. Bundle with Rolldown (minified, with source maps)
3. Update `index.html` with hashed script references
4. Inject version into service worker
