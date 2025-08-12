# brk_interface

Data query and formatting interface that provides a unified API for accessing Bitcoin datasets from both the indexer and computer components with flexible output formats and pagination support. This crate serves as the primary data access layer for BRK's web API and MCP endpoints.

## Features

- **Unified data access**: Query indexed blockchain data and computed analytics
- **Multiple output formats**: JSON, CSV, TSV, and Markdown table formatting
- **Flexible pagination**: Range queries with positive/negative indexing support
- **Multi-dataset queries**: Retrieve multiple datasets with the same index
- **Dynamic search**: Find datasets by ID with automatic fallbacks
- **Schema generation**: JSON Schema support for API documentation

## Query Parameters

### Core Parameters

- **index**: Time frame for data retrieval (height, date, week, month, etc.)
- **ids**: Dataset identifiers (comma or space separated)
- **from**: Starting index (negative values count from end)
- **to**: Ending index (optional, exclusive)
- **count**: Maximum number of results to return
- **format**: Output format (json, csv, tsv, md)

## Usage

```rust
use brk_interface::{Interface, Params, ParamsOpt, Index, Format};
use brk_indexer::Indexer;
use brk_computer::Computer;
use std::path::Path;

fn main() -> brk_error::Result<()> {
    let outputs_dir = Path::new("./brk_data");

    // Load indexer and computer
    let indexer = Indexer::forced_import(outputs_dir)?;
    let computer = Computer::forced_import(outputs_dir, &indexer, None)?;

    // Create interface
    let interface = Interface::build(&indexer, &computer);

    // Query latest block data
    let params = Params {
        index: Index::Height,
        ids: vec!["date", "timestamp"].into(),
        rest: ParamsOpt::default()
            .set_from(-1)  // Latest block
            .set_format(Format::JSON),
    };

    let result = interface.search_and_format(params)?;
    println!("{}", result);

    // Query price data for last 10 blocks
    let params = Params {
        index: Index::Height,
        ids: vec!["price_usd"].into(),
        rest: ParamsOpt::default()
            .set_from(-10)
            .set_count(10)
            .set_format(Format::CSV),
    };

    let result = interface.search_and_format(params)?;
    println!("{}", result);

    Ok(())
}
```

## API Integration

The interface provides methods for different use cases:

- `search()`: Find datasets matching parameters
- `format()`: Format search results into specified output format
- `search_and_format()`: Combined search and format operation
- `get_indexes()`: List available time indices
- `get_vecids()`: List available dataset identifiers
- `get_height()`: Get current blockchain height
