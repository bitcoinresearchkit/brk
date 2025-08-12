# brk_bundler

Web asset bundler built on Rolldown that provides JavaScript bundling, minification, and live reloading for BRK's web interface. This crate wraps Rolldown with BRK-specific functionality including automatic file watching, version injection, and hash-based cache busting for optimal web performance.

## Features

- **JavaScript bundling**: Bundles and minifies JavaScript with source maps
- **Live reloading**: Automatic rebuilding and file watching during development
- **Cache busting**: Hash-based filenames for browser cache invalidation
- **Version injection**: Automatic version injection into service workers
- **Asset copying**: Copies static assets from source to distribution directory
- **Development mode**: Optional watch mode for real-time development

## Usage

```rust
use brk_bundler::bundle;
use std::path::Path;

async fn build_website() -> std::io::Result<()> {
    let websites_path = Path::new("./websites");
    let source_folder = "src";
    let watch = false; // Set to true for development

    // Bundle the website
    let dist_path = bundle(websites_path, source_folder, watch).await?;

    println!("Website built to: {}", dist_path.display());
    Ok(())
}
```

## Build Process

1. **Clean**: Removes existing distribution directory
2. **Copy**: Copies all source files to distribution directory
3. **Bundle**: Processes JavaScript entry point with Rolldown
4. **Process**: Updates HTML to reference hashed JS files
5. **Version**: Injects version strings into service workers
6. **Watch** (optional): Monitors files for changes and rebuilds

## File Structure

Expected source structure:
```
websites/
├── src/
│   ├── index.html          # Main HTML file
│   ├── service-worker.js   # Service worker (optional)
│   ├── scripts/
│   │   └── entry.js        # JavaScript entry point
│   └── ...                 # Other static assets
└── dist/                   # Generated distribution files
```

## Watch Mode

When enabled, the bundler:
- Monitors source files for changes
- Automatically rebuilds JavaScript bundles
- Updates HTML with new hashed filenames
- Reprocesses service workers with version updates
- Copies modified static assets
