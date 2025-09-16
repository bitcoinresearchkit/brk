# brk_bundler

Asset bundling and development server for BRK web interfaces with hot reloading and file watching.

[![Crates.io](https://img.shields.io/crates/v/brk_bundler.svg)](https://crates.io/crates/brk_bundler)
[![Documentation](https://docs.rs/brk_bundler/badge.svg)](https://docs.rs/brk_bundler)

## Overview

This crate provides a thin wrapper around the Rolldown JavaScript bundler specifically designed for BRK web interface development. It handles asset bundling, file copying, template processing, and development-mode file watching with automatic rebuilds and hot reloading for efficient web development workflows.

**Key Features:**

- JavaScript bundling with Rolldown (Rust-based bundler)
- Automatic file watching and hot reloading in development mode
- Template processing with version injection and asset hash replacement
- Service worker generation with version management
- Source map generation for debugging
- Minification for production builds
- Async/await support with Tokio integration

**Target Use Cases:**

- BRK blockchain explorer web interfaces
- Development of Bitcoin analytics dashboards
- Building responsive web applications for blockchain data visualization
- Hot reloading development environment for rapid iteration

## Installation

```toml
cargo add brk_bundler
```

## Quick Start

```rust
use brk_bundler::bundle;
use std::path::Path;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let websites_path = Path::new("./web");
    let source_folder = "src";
    let watch = true; // Enable hot reloading

    // Bundle assets and start development server
    let dist_path = bundle(websites_path, source_folder, watch).await?;

    println!("Assets bundled to: {}", dist_path.display());

    // Keep running for file watching (in watch mode)
    if watch {
        tokio::signal::ctrl_c().await?;
    }

    Ok(())
}
```

## API Overview

### Core Functions

**`bundle(websites_path: &Path, source_folder: &str, watch: bool) -> io::Result<PathBuf>`**
Main bundling function that processes web assets and optionally starts file watching.

### Bundling Process

1. **Directory Setup**: Creates `dist/` directory and copies source files
2. **JavaScript Bundling**: Processes `scripts/entry.js` with Rolldown bundler
3. **Template Processing**: Updates `index.html` with hashed asset references
4. **Service Worker**: Generates service worker with version injection
5. **File Watching**: Optionally monitors source files for changes

### Configuration

**Rolldown Bundler Options:**

- **Input**: `./src/scripts/entry.js` (main JavaScript entry point)
- **Output**: `./dist/scripts/` directory
- **Minification**: Enabled for production builds
- **Source Maps**: File-based source maps for debugging
- **Asset Hashing**: Automatic hash generation for cache busting

## Examples

### Development Mode with Hot Reloading

```rust
use brk_bundler::bundle;
use std::path::Path;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let web_root = Path::new("./websites");

    // Start development server with file watching
    let _dist_path = bundle(web_root, "explorer", true).await?;

    println!("Development server started!");
    println!("Hot reloading enabled - edit files to see changes");

    // Keep server running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
```

### Production Build

```rust
use brk_bundler::bundle;
use std::path::Path;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let web_root = Path::new("./websites");

    // Build for production (no watching)
    let dist_path = bundle(web_root, "dashboard", false).await?;

    println!("Production build completed: {}", dist_path.display());

    // Assets are minified and ready for deployment
    Ok(())
}
```

### Custom Web Application Structure

```rust
use brk_bundler::bundle;
use std::path::Path;

// Expected directory structure:
// websites/
// ├── my_app/
// │   ├── index.html          // Main HTML template
// │   ├── service-worker.js   // Service worker template
// │   ├── scripts/
// │   │   └── entry.js        // JavaScript entry point
// │   ├── styles/
// │   │   └── main.css        // CSS files
// │   └── assets/
// │       └── images/         // Static assets
// └── dist/                   // Generated output

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let websites_path = Path::new("./websites");
    let source_folder = "my_app";

    let dist_path = bundle(websites_path, source_folder, false).await?;

    // Result: dist/ contains bundled and processed files
    // - dist/index.html (with updated script references)
    // - dist/service-worker.js (with version injection)
    // - dist/scripts/main.[hash].js (minified and hashed)
    // - dist/styles/ (copied CSS files)
    // - dist/assets/ (copied static assets)

    Ok(())
}
```

## Architecture

### File Processing Pipeline

1. **Source Copying**: Recursively copies all source files to dist directory
2. **JavaScript Bundling**: Rolldown processes entry.js with dependencies
3. **Asset Hashing**: Generates content-based hashes for cache busting
4. **Template Updates**: Replaces placeholders in HTML templates
5. **Version Injection**: Updates service worker with current package version

### File Watching System

**Development Mode Watchers:**

- **Source File Watcher**: Monitors non-script files for changes
- **Bundle Watcher**: Watches JavaScript files and triggers rebuilds
- **Template Watcher**: Updates HTML when bundled assets change

**Event Handling:**

- **File Creation/Modification**: Automatic copying to dist directory
- **Script Changes**: Triggers Rolldown rebuild and template update
- **Template Changes**: Processes HTML and updates asset references

### Template Processing

**index.html Processing:**

- Scans bundled JavaScript for asset hash
- Replaces `/scripts/main.js` with `/scripts/main.[hash].js`
- Maintains cache busting while preserving template structure

**service-worker.js Processing:**

- Replaces `__VERSION__` placeholder with current crate version
- Enables version-based cache invalidation
- Maintains service worker functionality

### Async Architecture

Built on Tokio async runtime:

- **Non-blocking I/O**: Efficient file operations and watching
- **Concurrent Tasks**: Parallel file watching and bundle processing
- **Background Processing**: Development server runs in background task

## Configuration Options

### Rolldown Configuration

The bundler uses optimized Rolldown settings:

```rust
BundlerOptions {
    input: Some(vec!["./src/scripts/entry.js".into()]),
    dir: Some("./dist/scripts".to_string()),
    minify: Some(RawMinifyOptions::Bool(true)),
    sourcemap: Some(SourceMapType::File),
    // ... other default options
}
```

### File Structure Requirements

**Required Files:**

- `src/scripts/entry.js` - JavaScript entry point
- `src/index.html` - HTML template
- `src/service-worker.js` - Service worker template

**Optional Directories:**

- `src/styles/` - CSS stylesheets
- `src/assets/` - Static assets (images, fonts, etc.)
- `src/components/` - Additional JavaScript modules

## Development Workflow

### Setup

1. Create web application in `websites/app_name/`
2. Add required files (index.html, entry.js, service-worker.js)
3. Run bundler in watch mode for development

### Hot Reloading

- **Script Changes**: Automatic bundle rebuild and browser refresh
- **Template Changes**: Immediate HTML update with asset hash replacement
- **Asset Changes**: Instant copy to dist directory
- **Style Changes**: Direct copy without bundling

### Production Deployment

1. Run bundler without watch mode
2. Deploy `dist/` directory contents
3. Assets include content hashes for cache busting
4. Service worker includes version for cache management

## Code Analysis Summary

**Main Function**: `bundle()` async function coordinating Rolldown bundler with file processing and watching \
**File Operations**: Recursive directory copying with `copy_dir_all()` and selective file processing \
**Templating**: String replacement for asset hash injection and version management \
**File Watching**: Multi-watcher system using `notify` crate for real-time development feedback \
**Async Integration**: Tokio-based async architecture with background task spawning \
**Bundler Integration**: Rolldown wrapper with optimized configuration for web development \
**Architecture**: Development-focused asset pipeline with hot reloading and production optimization

---

_This README was generated by Claude Code_
