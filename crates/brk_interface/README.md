# brk_interface

Unified data query and formatting interface for Bitcoin datasets with intelligent search and multi-format output.

[![Crates.io](https://img.shields.io/crates/v/brk_interface.svg)](https://crates.io/crates/brk_interface)
[![Documentation](https://docs.rs/brk_interface/badge.svg)](https://docs.rs/brk_interface)

## Overview

This crate provides a high-level interface for querying and formatting data from BRK's indexer and computer components. It offers intelligent vector search with fuzzy matching, parameter validation, range queries, and multi-format output (JSON, CSV, TSV, Markdown) with efficient caching and pagination support.

**Key Features:**

- Unified query interface across indexer and computer data sources
- Intelligent search with fuzzy matching and helpful error messages
- Multi-format output: JSON, CSV, TSV, Markdown with proper formatting
- Range-based data queries with flexible from/to parameters
- Comprehensive pagination support for large datasets
- Schema validation with JSON Schema generation for API documentation
- Efficient caching system for error messages and repeated queries

**Target Use Cases:**

- REST API backends requiring flexible data queries
- Data export tools supporting multiple output formats
- Interactive applications with user-friendly error messaging
- Research platforms requiring structured data access

## Installation

```bash
cargo add brk_interface
```

## Quick Start

```rust
use brk_interface::{Interface, Params, Index};
use brk_indexer::Indexer;
use brk_computer::Computer;

// Initialize with indexer and computer
let indexer = Indexer::build(/* config */)?;
let computer = Computer::build(/* config */)?;
let interface = Interface::build(&indexer, &computer);

// Query data with parameters
let params = Params {
    index: Index::Height,
    ids: vec!["height-to-blockhash".to_string()].into(),
    from: Some(800000),
    to: Some(800100),
    format: Some(Format::JSON),
    ..Default::default()
};

// Search and format results
let output = interface.search_and_format(params)?;
println!("{}", output);
```

## API Overview

### Core Types

- **`Interface<'a>`**: Main query interface coordinating indexer and computer access
- **`Params`**: Query parameters including index, IDs, range, and formatting options
- **`Index`**: Enumeration of available data indexes (Height, Date, Address, etc.)
- **`Format`**: Output format specification (JSON, CSV, TSV, MD)
- **`Output`**: Formatted query results with multiple value types

### Key Methods

**`Interface::build(indexer: &Indexer, computer: &Computer) -> Self`**
Creates interface instance with references to data sources.

**`search(&self, params: &Params) -> Result<Vec<(String, &&dyn AnyCollectableVec)>>`**
Searches for vectors matching the query parameters with intelligent error handling.

**`format(&self, vecs: Vec<...>, params: &ParamsOpt) -> Result<Output>`**
Formats search results according to specified output format and range parameters.

**`search_and_format(&self, params: Params) -> Result<Output>`**
Combined search and formatting operation for single-call data retrieval.

### Query Parameters

**Core Parameters:**

- `index`: Data index to query (height, date, address, etc.)
- `ids`: Vector IDs to retrieve from the specified index
- `from`/`to`: Optional range filtering (inclusive start, exclusive end)
- `format`: Output format (defaults to JSON)

**Pagination Parameters:**

- `offset`: Number of entries to skip
- `limit`: Maximum entries to return

## Examples

### Basic Data Query

```rust
use brk_interface::{Interface, Params, Index, Format};

let interface = Interface::build(&indexer, &computer);

// Query block heights to hashes
let params = Params {
    index: Index::Height,
    ids: vec!["height-to-blockhash".to_string()].into(),
    from: Some(750000),
    to: Some(750010),
    format: Some(Format::JSON),
    ..Default::default()
};

match interface.search_and_format(params)? {
    Output::Json(value) => println!("{}", serde_json::to_string_pretty(&value)?),
    _ => unreachable!(),
}
```

### CSV Export with Range Query

```rust
use brk_interface::{Interface, Params, Index, Format};

// Export price data as CSV
let params = Params {
    index: Index::Date,
    ids: vec!["dateindex-to-price-close".to_string()].into(),
    from: Some(0),      // From genesis
    to: Some(5000),     // First ~13 years
    format: Some(Format::CSV),
    ..Default::default()
};

match interface.search_and_format(params)? {
    Output::CSV(csv_text) => {
        std::fs::write("bitcoin_prices.csv", csv_text)?;
        println!("Price data exported to bitcoin_prices.csv");
    },
    _ => unreachable!(),
}
```

### Multi-Vector Query with Markdown Table

```rust
use brk_interface::{Interface, Params, Index, Format};

// Query multiple vectors and format as table
let params = Params {
    index: Index::Height,
    ids: vec![
        "height-to-blockhash".to_string(),
        "height-to-timestamp".to_string(),
        "height-to-difficulty".to_string()
    ].into(),
    from: Some(800000),
    to: Some(800005),
    format: Some(Format::MD),
    ..Default::default()
};

match interface.search_and_format(params)? {
    Output::MD(table) => println!("{}", table),
    _ => unreachable!(),
}
```

### Intelligent Error Handling

```rust
use brk_interface::{Interface, Params, Index};

// Query with typo in vector ID
let params = Params {
    index: Index::Height,
    ids: vec!["height-to-blockhas".to_string()].into(), // Typo: "blockhas"
    ..Default::default()
};

// Interface provides helpful error with suggestions
match interface.search(&params) {
    Err(error) => {
        println!("{}", error);
        // Output: No vec named "height-to-blockhas" indexed by "height" found.
        //         Maybe you meant one of the following: ["height-to-blockhash"] ?
    },
    Ok(_) => unreachable!(),
}
```

## Architecture

### Data Source Integration

The interface acts as a bridge between:

- **Indexer**: Raw blockchain data vectors (blocks, transactions, addresses)
- **Computer**: Computed analytics vectors (prices, statistics, aggregations)
- **Unified Access**: Single query interface for both data sources

### Search Implementation

1. **Parameter Validation**: Validates index existence and parameter consistency
2. **Vector Resolution**: Maps vector IDs to actual data structures
3. **Fuzzy Matching**: Provides suggestions for mistyped vector names
4. **Error Caching**: Caches error messages to avoid repeated expensive operations

### Output Formatting

**JSON Output:**

- Single value: Direct value serialization
- List: Array of values
- Matrix: Array of arrays for multi-vector queries

**Tabular Output (CSV/TSV/MD):**

- Column headers from vector IDs
- Row-wise data iteration with proper escaping
- Markdown tables use `tabled` crate formatting

### Caching Strategy

- **Error Message Caching**: 1000-entry LRU cache for error messages
- **Search Result Caching**: Upstream caching in server/client layers
- **Static Data Caching**: Index and vector metadata cached during initialization

## Configuration

### Index Types

Available indexes include:

- `Height`: Block height-based indexing
- `Date`: Calendar date indexing
- `Address`: Bitcoin address indexing
- `Transaction`: Transaction hash indexing
- Custom indexes from computer analytics

### Format Options

- **JSON**: Structured data with nested objects/arrays
- **CSV**: Comma-separated values with proper escaping
- **TSV**: Tab-separated values for import compatibility
- **MD**: Markdown tables for documentation and reports

## Code Analysis Summary

**Main Structure**: `Interface` struct coordinating between `Indexer` and `Computer` data sources \
**Query System**: Parameter-driven search with `Params` struct supporting range queries and formatting options \
**Error Handling**: Intelligent fuzzy matching with cached error messages and helpful suggestions \
**Output Formats**: Multi-format support (JSON, CSV, TSV, Markdown) with proper data serialization \
**Caching**: `quick_cache` integration for error messages and expensive operations \
**Search Logic**: `nucleo-matcher` fuzzy search for user-friendly vector name resolution \
**Architecture**: Abstraction layer providing unified access to heterogeneous Bitcoin data sources

---

_This README was generated by Claude Code_
