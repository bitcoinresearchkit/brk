# brk_interface

**Unified data query and formatting interface for Bitcoin datasets**

`brk_interface` provides a clean, unified API for accessing Bitcoin datasets from both indexer and computer components. It serves as the primary data access layer powering BRK's web API and MCP endpoints, offering flexible querying, pagination, and multiple output formats.

## What it provides

- **Unified Data Access**: Single interface to query both indexed blockchain data and computed analytics
- **Multiple Output Formats**: JSON, CSV, TSV, and Markdown table formatting
- **Flexible Pagination**: Range queries with positive/negative indexing and automatic pagination
- **Multi-dataset Queries**: Retrieve multiple datasets with the same time index in one call
- **Dynamic Search**: Intelligent ID matching with automatic fallbacks and normalization

## Key Features

### Query Interface
- **25 Time Indices**: From granular (Height, DateIndex) to aggregate (YearIndex, DecadeIndex)
- **Bitcoin-Specific Indices**: Address types (P2PKH, P2SH, P2TR), output types, epochs
- **Multi-dataset support**: Query multiple related datasets simultaneously
- **Intelligent ID resolution**: Flexible matching with automatic fallbacks

### Pagination and Ranges
- **Signed indexing**: Negative indices count from end (`-1` = latest, `-10` = last 10)
- **Range queries**: `from`/`to` parameters with optional `count` limit
- **Efficient pagination**: 1,000 items per page with proper start/end calculation

### Output Formatting
- **JSON**: Single values, arrays, or matrices with automatic structure detection
- **CSV/TSV**: Delimiter-separated values with headers for spreadsheet use
- **Markdown**: Tables formatted for documentation and display
- **Schema Support**: JSON Schema generation for API documentation

## Usage

### Basic Query Setup

```rust
use brk_interface::{Interface, Params, ParamsOpt, Index, Format};
use brk_indexer::Indexer;
use brk_computer::Computer;

// Load data sources
let indexer = Indexer::forced_import("./brk_data")?;
let computer = Computer::forced_import("./brk_data", &indexer, None)?;

// Create unified interface
let interface = Interface::build(&indexer, &computer);
```

### Single Dataset Queries

```rust
// Get latest block data
let params = Params {
    index: Index::Height,
    ids: vec!["date", "timestamp", "difficulty"].into(),
    rest: ParamsOpt::default()
        .set_from(-1)  // Latest block
        .set_format(Format::JSON),
};

let result = interface.search_and_format(params)?;
println!("{}", result);
```

### Range Queries

```rust
// Get price data for last 30 days
let params = Params {
    index: Index::DateIndex,
    ids: vec!["price_usd", "price_usd_high", "price_usd_low"].into(),
    rest: ParamsOpt::default()
        .set_from(-30)  // Last 30 days
        .set_count(30)
        .set_format(Format::CSV),
};

let csv_data = interface.search_and_format(params)?;
```

### Multiple Datasets

```rust
// Get comprehensive block statistics
let params = Params {
    index: Index::Height,
    ids: vec!["size", "weight", "tx_count", "fee_total"].into(),
    rest: ParamsOpt::default()
        .set_from(800_000)  // Starting from block 800,000
        .set_to(800_100)    // Up to block 800,100
        .set_format(Format::TSV),
};

let tsv_data = interface.search_and_format(params)?;
```

### Flexible ID Specification

```rust
// Different ways to specify dataset IDs
let params = Params {
    index: Index::DateIndex,
    ids: vec!["price_usd,volume_usd,market_cap"].into(),  // Comma-separated
    // OR: ids: vec!["price_usd volume_usd market_cap"].into(),  // Space-separated
    // OR: ids: vec!["price_usd", "volume_usd", "market_cap"].into(),  // Array
    rest: ParamsOpt::default()
        .set_format(Format::JSON),
};
```

### Advanced Queries with Pagination

```rust
// Query with custom pagination
let params = Params {
    index: Index::MonthIndex,
    ids: vec!["supply_total", "supply_active"].into(),
    rest: ParamsOpt::default()
        .set_from(0)        // From beginning
        .set_count(50)      // Max 50 results
        .set_format(Format::Markdown),
};

let markdown_table = interface.search_and_format(params)?;
```

## API Methods

### Core Query Methods

```rust
// Combined search and format
let result = interface.search_and_format(params)?;

// Separate search and format
let vecs = interface.search(params)?;
let formatted = interface.format(vecs, params.rest.format.unwrap_or_default())?;

// Get metadata
let current_height = interface.get_height();
let available_datasets = interface.get_vecids(None)?;  // Paginated
let available_indices = interface.get_indexes();
```

### Working with Results

```rust
use brk_interface::{Value, Output};

// Handle different output types
match interface.search_and_format(params)? {
    Output::Json(Value::Single(val)) => println!("Single value: {}", val),
    Output::Json(Value::List(arr)) => println!("Array with {} items", arr.len()),
    Output::Json(Value::Matrix(matrix)) => println!("Matrix: {}x{}", matrix.len(), matrix[0].len()),
    Output::Delimited(csv_data) => println!("CSV/TSV data:\n{}", csv_data),
    Output::Markdown(table) => println!("Markdown table:\n{}", table),
}
```

## Available Indices

### Time-based Indices
- `Height` - Block height (0, 1, 2, ...)
- `DateIndex` - Days since Bitcoin genesis
- `WeekIndex`, `MonthIndex`, `QuarterIndex`, `YearIndex`, `DecadeIndex`
- `HalvingEpoch`, `DifficultyEpoch` - Bitcoin-specific epochs

### Bitcoin-specific Indices
- Address types: `P2PKHAddressIndex`, `P2SHAddressIndex`, `P2WPKHAddressIndex`, etc.
- Output types: `OutputType` classifications
- Transaction types: `TxIndex`, `InputIndex`, `OutputIndex`

## Parameter Reference

```rust
pub struct Params {
    pub index: Index,           // Time dimension for query
    pub ids: MaybeIds,         // Dataset identifiers
    pub rest: ParamsOpt,       // Optional parameters
}

pub struct ParamsOpt {
    pub from: Option<i64>,     // Starting index (negative = from end)
    pub to: Option<i64>,       // Ending index (exclusive)
    pub count: Option<usize>,  // Maximum results
    pub format: Option<Format>, // Output format
}
```

## Output Formats

- **JSON**: Structured data (single values, arrays, matrices)
- **CSV**: Comma-separated values with headers
- **TSV**: Tab-separated values with headers
- **Markdown**: Formatted tables for documentation

## Performance Features

- **Zero-copy operations**: Uses references and lifetimes to avoid cloning
- **Memory mapping**: Leverages vecdb's memory-mapped storage
- **Lazy evaluation**: Only processes data when formatting is requested
- **Efficient pagination**: Smart bounds calculation and range queries

## Schema Support

The interface provides JSON Schema support for API documentation:

```rust
use schemars::schema_for;

let schema = schema_for!(Params);
// Use schema for API documentation
```

## Dependencies

- `brk_indexer` - Indexed blockchain data source
- `brk_computer` - Computed analytics data source
- `vecdb` - Vector database with collection traits
- `serde` - Serialization/deserialization support
- `schemars` - JSON Schema generation

---

*This README was generated by Claude Code*