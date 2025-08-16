# brk_bundler

**Asset bundling for BRK web interfaces using Rolldown**

`brk_bundler` provides JavaScript/TypeScript bundling capabilities for BRK's web interfaces. It's a thin wrapper around Rolldown (Rust-based Rollup alternative) with BRK-specific configuration for building optimized web assets with file watching and automatic rebuilding.

## What it provides

- **JavaScript Bundling**: Modern ES modules bundling with minification
- **File Watching**: Automatic rebuilding on source file changes
- **Asset Processing**: Copies and processes static assets
- **Version Injection**: Automatic version string replacement in service workers
- **Development Mode**: Live rebuilding for rapid development

## Key Features

### Bundling Capabilities
- **ES Module Support**: Modern JavaScript bundling with tree-shaking
- **Minification**: Automatic code minification for production builds
- **Source Maps**: Generated source maps for debugging
- **Entry Point Processing**: Configurable entry points with hashed output names

### File System Operations
- **Directory Copying**: Copies entire source directories to distribution
- **Selective Processing**: Special handling for specific file types
- **Path Resolution**: Automatic path resolution and asset linking

### Development Features
- **Hot Rebuilding**: Automatic rebuilds on file changes
- **Watch Mode**: Monitors source files and triggers rebuilds
- **Version Replacement**: Injects build version into service workers

## Usage

### Basic Bundling

```rust
use brk_bundler::bundle;
use std::path::Path;

// Bundle without watching (production)
let websites_path = Path::new("./websites");
let source_folder = "default";
let dist_path = bundle(websites_path, source_folder, false).await?;

println!("Bundled to: {:?}", dist_path);
```

### Development Mode with Watching

```rust
// Bundle with file watching (development)
let dist_path = bundle(websites_path, "default", true).await?;

// Bundler now watches for changes and rebuilds automatically
// This will run in the background until the process exits
```

### Integration with BRK CLI

```rust
// Typically called from brk_cli when serving websites
async fn setup_website(config: &Config) -> Result<PathBuf> {
    let websites_path = config.websites_path();
    let source_folder = match config.website_mode {
        WebsiteMode::Default => "default",
        WebsiteMode::Custom => "custom",
        WebsiteMode::None => return Ok(PathBuf::new()),
    };
    
    // Bundle the website assets
    let dist_path = bundle(websites_path, source_folder, config.dev_mode).await?;
    
    Ok(dist_path)
}
```

## File Structure

The bundler expects this directory structure:

```
websites/
├── default/                 # Default website source
│   ├── index.html          # Main HTML file
│   ├── service-worker.js   # Service worker (version injected)
│   ├── scripts/            # JavaScript/TypeScript source
│   │   ├── entry.js        # Main entry point
│   │   ├── main.js         # Application logic
│   │   └── ...             # Other JS modules
│   └── assets/             # Static assets
└── dist/                   # Generated output directory
    ├── index.html          # Processed HTML with updated script references
    ├── service-worker.js   # Service worker with version injected
    ├── scripts/            # Bundled and minified JavaScript
    │   └── main-[hash].js  # Hashed output file
    └── assets/             # Copied static assets
```

## Bundling Process

1. **Clean**: Removes existing `dist/` directory
2. **Copy**: Copies all source files to `dist/`
3. **Bundle JavaScript**: 
   - Processes `scripts/entry.js` as entry point
   - Generates minified bundle with source maps
   - Creates hashed filename for cache busting
4. **Process HTML**: Updates script references to hashed filenames
5. **Process Service Worker**: Injects current version string
6. **Watch** (if enabled): Monitors for file changes and rebuilds

## Configuration

The bundler uses Rolldown with these optimized settings:

```rust
BundlerOptions {
    input: Some(vec![source_entry.into()]),    // scripts/entry.js
    dir: Some("./dist/scripts".to_string()),   // Output directory
    cwd: Some(websites_path),                  // Working directory
    minify: Some(RawMinifyOptions::Bool(true)), // Enable minification
    sourcemap: Some(SourceMapType::File),      // Generate source maps
    ..Default::default()
}
```

## File Watching

In watch mode, the bundler monitors:

- **Source files**: Non-script files are copied on change
- **JavaScript files**: Trigger full rebuild via Rolldown watcher
- **HTML files**: Processed to update script references
- **Service worker**: Version injection on changes

### Watch Events Handled

- `Create` - New files added
- `Modify` - Existing files changed
- Ignores `Delete` and other events

## Version Injection

Service workers get automatic version injection:

```javascript
// In source service-worker.js
const VERSION = '__VERSION__';

// After bundling
const VERSION = 'v0.0.88';
```

This enables proper cache invalidation across releases.

## Performance Features

- **Async Operations**: All bundling operations are async
- **Incremental Builds**: Only rebuilds changed files in watch mode
- **Parallel Processing**: Uses Tokio for concurrent file operations
- **Efficient Copying**: Direct file system operations

## Error Handling

- **Graceful Failures**: Logs errors but continues watching
- **Path Resolution**: Automatic path absolutization and validation
- **File System Errors**: Proper error propagation with context

## Dependencies

- `brk_rolldown` - Rust-based Rollup bundler
- `notify` - File system watching
- `tokio` - Async runtime for file operations
- `sugar_path` - Path manipulation utilities
- `log` - Error logging

## Integration Points

The bundler integrates with:
- **brk_cli**: Called during website setup
- **brk_server**: Serves bundled assets
- **Development workflow**: Provides live rebuilding

---

*This README was generated by Claude Code*