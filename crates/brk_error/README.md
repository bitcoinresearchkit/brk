# brk_error

Centralized error handling for the Bitcoin Research Kit that provides a unified error type and result type for consistent error propagation across all BRK crates. This crate consolidates errors from external dependencies and defines domain-specific error variants used throughout the BRK ecosystem.

## Error Types

### External Library Errors
- **IO**: Standard I/O operations (`std::io::Error`)
- **BitcoinRPC**: Bitcoin Core RPC client errors
- **Jiff**: Date/time parsing and manipulation errors
- **Fjall**: Key-value store errors
- **VecDB/SeqDB**: Vector database errors
- **Minreq**: HTTP client errors
- **SerdeJson**: JSON serialization/deserialization errors
- **ZeroCopy**: Memory layout conversion errors
- **SystemTime**: System time errors

### Domain-Specific Errors
- **WrongLength**: Invalid data length
- **WrongAddressType**: Unsupported Bitcoin address type
- **UnindexableDate**: Date outside indexable range (before 2009-01-03)
- **QuickCacheError**: Cache operation failures
- **Str/String**: Custom error messages

## Usage

```rust
use brk_error::{Error, Result};

fn process_bitcoin_data() -> Result<()> {
    // Operations that may fail with various error types
    let data = std::fs::read("blocks.dat")?;  // IO error
    let parsed = parse_data(&data)?;          // Custom error
    Ok(())
}

fn parse_data(data: &[u8]) -> Result<ParsedData> {
    if data.len() < 80 {
        return Err(Error::WrongLength);
    }
    // ... parsing logic
    Ok(parsed_data)
}
```

## Type Alias

The crate exports `Result<T, E = Error>` as the standard result type, allowing for concise error handling:

```rust
use brk_error::Result;

fn my_function() -> Result<String> {
    // Automatically uses brk_error::Error as the error type
    Ok("success".to_string())
}
```
