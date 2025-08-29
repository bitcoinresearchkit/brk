# brk_server

**HTTP server providing REST API access to Bitcoin analytics data**

`brk_server` provides a high-performance HTTP server that exposes BRK's indexed blockchain data and computed analytics through a comprehensive REST API. It offers multiple output formats, intelligent caching, compression, and optional web interface serving.

## What it provides

- **REST API**: Vector-based data access with flexible querying and pagination
- **Multiple Output Formats**: JSON, CSV, TSV, and Markdown table formatting
- **Performance Features**: ETag caching, compression, and request weight limiting
- **Web Interface**: Optional static file serving for web applications
- **MCP Integration**: Model Context Protocol support for LLM integration

## Key Features

### API Capabilities
- **Direct vector access**: Single vector queries with index-to-ID pattern
- **Multi-vector queries**: Query multiple datasets simultaneously
- **Flexible pagination**: Positive/negative indexing with range queries
- **Format negotiation**: Multiple output formats based on use case

### Performance Features
- **ETag caching**: Conditional requests with 304 Not Modified responses
- **Compression**: Brotli, Gzip, Zstd, Deflate support with automatic negotiation
- **Request weight limiting**: Protects against oversized queries (max 320,000 weight)
- **In-memory caching**: 10,000-item cache with 50ms guard timeout

### HTTP Features
- **Auto port assignment**: Starts from port 3110, increments if busy
- **CORS enabled**: Cross-origin requests for web clients
- **Tracing middleware**: Colored request/response logging
- **Static file serving**: Optional website hosting

## Usage

### Basic Server Setup

```rust
use brk_server::Server;
use brk_interface::Interface;
use brk_indexer::Indexer;
use brk_computer::Computer;

// Load data sources
let indexer = Indexer::forced_import("./brk_data")?;
let computer = Computer::forced_import("./brk_data", &indexer, None)?;

// Create interface and server
let interface = Interface::build(&indexer, &computer);
let server = Server::new(interface, Some("./website".into()));

// Start server (with MCP support)
server.serve(true).await?;
```

### API Access Patterns

#### Single Vector Queries

```bash
# Latest 100 price values
curl "http://brekit.org/api/vecs/date-to-close?from=-100"

# First 50 difficulty values as CSV
curl "http://brekit.org/api/vecs/height-to-difficulty?count=50&format=csv"

# Range from block 800,000 to 800,100
curl "https://brekit.org/api/vecs/height-to-timestamp?from=800000&to=800100"
```

#### Multi-Vector Queries

```bash
# Multiple price metrics for last 30 days
curl "http://brekit.org/api/vecs/query?index=date&ids=open,high,low,close&from=-30&format=csv"

# Block statistics for specific range
curl "https://brekit.org/api/vecs/query?index=height&ids=size,weight,tx_count,fee_sum&from=800000&count=100"

# Weekly analytics as JSON matrix
curl "https://brekit.org/api/vecs/query?index=week&ids=close,difficulty&from=-52"
```

## API Reference

### Vector Metadata Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /api/vecs/index-count` | Total number of available indexes |
| `GET /api/vecs/id-count` | Total number of vector IDs |
| `GET /api/vecs/vec-count` | Total vectors (all index/ID combinations) |
| `GET /api/vecs/indexes` | List of all available indexes |
| `GET /api/vecs/accepted-indexes` | Mapping of indexes to accepted variants |
| `GET /api/vecs/ids?page=N` | Paginated vector IDs (1000/page) |
| `GET /api/vecs/index-to-ids?index=INDEX&page=N` | IDs supporting given index |
| `GET /api/vecs/id-to-indexes?id=ID` | Indexes supported by given ID |

### Vector Data Access

#### Direct Access Pattern
`GET /api/vecs/{INDEX}-to-{ID}`

```bash
# Examples
curl "/api/vecs/height-to-timestamp"
curl "/api/vecs/date-to-close"
curl "/api/vecs/month-to-supply"
```

#### Multi-Vector Query
`GET /api/vecs/query`

**Required Parameters:**
- `index`: Vector index type (height, date, week, month, etc.)
- `ids`: Comma or space-separated vector IDs

**Optional Parameters:**
- `from` (i64): Start index (negative = from end, default: 0)
- `to` (i64): End index (exclusive, negative = from end)
- `count` (usize): Maximum number of results
- `format`: Output format (`json`, `csv`, `tsv`, `md`)

### Response Types

**Single Value** (one vector, one result):
```json
42.5
```

**Array** (one vector, multiple results):
```json
[42.5, 43.1, 44.2]
```

**Matrix** (multiple vectors):
```json
[
  [42.5, 43.1, 44.2],
  [1500, 1520, 1480],
  [0.05, 0.052, 0.048]
]
```

**CSV Format**:
```csv
index,close,supply,fee_rate
0,42.5,1500,0.05
1,43.1,1520,0.052
2,44.2,1480,0.048
```

### System Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /version` | Server version information |
| `GET /health` | Health check with timestamp |
| `GET /api` | API documentation redirect |
| `GET /*` | Static file serving (if enabled) |

## Configuration

### Server State

```rust
pub struct AppState {
    interface: &'static Interface<'static>,  // Data access interface
    path: Option<PathBuf>,                   // Static files path
    cache: Arc<Cache<String, Bytes>>,        // Response cache
}
```

### Middleware Stack

- **Compression Layer**: Brotli, Gzip, Zstd, Deflate
- **Response URI Layer**: Adds URI to response extensions for logging
- **Trace Layer**: Request/response logging with colored output

### Caching Strategy

- **ETag generation**: Based on data content and query parameters
- **Conditional requests**: 304 Not Modified for unchanged data
- **In-memory cache**: 10,000 items with LRU eviction
- **Cache headers**: `Cache-Control: must-revalidate`

## Performance Characteristics

### Request Weight System
- **Weight calculation**: Based on data range size and complexity
- **Weight limit**: 320,000 units per request
- **Protection**: Prevents oversized queries that could impact performance

### Compression
- **Automatic negotiation**: Based on `Accept-Encoding` header
- **Multiple algorithms**: Brotli (best), Zstd, Gzip, Deflate
- **Transparent**: Applied automatically to all responses

### Logging
- **Colored output**: Green for 200, red for errors, gray for redirects
- **Request timing**: Latency measurement and display
- **Status tracking**: HTTP status codes with appropriate colors

## Dependencies

- `axum` - High-performance async web framework
- `tower-http` - HTTP middleware (compression, tracing, CORS)
- `brk_interface` - Data access and formatting layer
- `brk_mcp` - Model Context Protocol routes
- `quick_cache` - Fast in-memory caching
- `tokio` - Async runtime for networking

---

*This README was generated by Claude Code*
