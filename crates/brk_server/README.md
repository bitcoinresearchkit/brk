# brk_server

HTTP server providing REST API access to Bitcoin analytics data. Serves computed datasets from brk_indexer and brk_computer with multiple output formats, caching, and optional web interfaces.

## Overview

**Core Features**:
- **REST API**: Vector-based data access with pagination and format options
- **Multiple Formats**: JSON, CSV, TSV, Markdown output
- **HTTP Caching**: ETag-based caching with compression (Brotli, Gzip, Zstd, Deflate)
- **Weight Limits**: Request size protection (max 320,000 weight units)
- **Web Interface**: Optional website serving

**Port**: Auto-assigns starting from 3110

## API Reference

### Vector Metadata

| Endpoint | Description |
|----------|-------------|
| `GET /api/vecs/index-count` | Total number of available indexes |
| `GET /api/vecs/id-count` | Total number of vector IDs |
| `GET /api/vecs/vec-count` | Total number of vectors (sum of all index/ID combinations) |
| `GET /api/vecs/indexes` | List of all available indexes |
| `GET /api/vecs/accepted-indexes` | Object mapping indexes to their accepted variants |
| `GET /api/vecs/ids?page=N` | Paginated list of vector IDs (1000 per page, default page=0) |
| `GET /api/vecs/index-to-ids?index=INDEX&page=N` | Vector IDs supporting given index |
| `GET /api/vecs/id-to-indexes?id=ID` | Indexes supported by given vector ID |

### Vector Data Access

#### Direct Access Pattern: `GET /api/vecs/{INDEX}-to-{ID}`

Access single vector with index-to-id pattern (dashes replaced with underscores internally).

**Query Parameters:**
- `from` (i64, optional): Inclusive start index. Negative values count from end (default: 0)
- `to` (i64, optional): Exclusive end index. Negative values count from end. Overrides `count`
- `count` (usize, optional): Number of values to retrieve
- `format` (string, optional): Output format - `json`, `csv`, `tsv`, `md` (default: `json`)

**Examples:**
```bash
# Latest 100 price closes
curl /api/vecs/date-to-close?from=-100

# First 50 values as CSV
curl /api/vecs/height-to-difficulty?count=50&format=csv

# Range from index 1000 to 1999
curl /api/vecs/date-to-volume?from=1000&to=2000
```

#### Multi-Vector Query: `GET /api/vecs/query`

Query multiple vectors simultaneously with flexible output formats.

**Required Parameters:**
- `index`: Vector index type
- `ids`: Comma or space-separated vector IDs

**Optional Parameters:**
- `from`, `to`, `count`, `format`: Same as direct access

**Response Types:**
- **Single value**: One vector, one result (e.g., `from=-1`)
- **Array**: One vector, multiple results
- **Matrix**: Multiple vectors (always matrix, even for single results)

**Examples:**
```bash
# Single vector, latest value
curl '/api/vecs/query?index=date&ids=close&from=-1'

# Multiple vectors, date range
curl '/api/vecs/query?index=date&ids=open,high,low,close&from=-30&format=csv'

# Complex multi-vector query
curl '/api/vecs/query?index=week&ids=price-usd,volume,difficulty&count=52'
```

### System Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /version` | Server version (JSON string) |
| `GET /health` | Health check with timestamp |
| `GET /api` | Redirects to this documentation |
| `GET /*` | Static file serving (when website enabled) |

### HTTP Features

**Caching:**
- ETag-based conditional requests (304 Not Modified)
- `Cache-Control: must-revalidate` headers
- In-memory cache with 50ms guard timeout

**Compression:**
- Brotli, Gzip, Zstd, Deflate support
- Automatic content encoding negotiation

**CORS:**
- Cross-origin requests enabled
- Appropriate headers for web client access

**Rate Limiting:**
- Request weight system (max 320,000 units)
- Weight calculated from data range size
