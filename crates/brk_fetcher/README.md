# brk_fetcher

**Bitcoin price data fetcher with multi-source fallback and retry logic**

`brk_fetcher` provides reliable Bitcoin price data retrieval from multiple sources including Binance, Kraken, and BRK instances. It offers both date-based and block height-based price queries with automatic fallback and retry mechanisms for robust data collection.

## What it provides

- **Multi-source fallback**: Automatic fallback between Kraken → Binance → BRK
- **Flexible querying**: Fetch prices by date or block height with timestamps
- **Retry logic**: Built-in retry mechanism with exponential backoff
- **Multiple timeframes**: 1-minute and 1-day interval support
- **HAR file import**: Import historical Binance chart data from browser

## Key Features

### Data Sources
- **Kraken API**: Primary source for OHLC data (1-day and 1-minute intervals)
- **Binance API**: Secondary source with additional historical data
- **BRK instance**: Fallback source for previously cached price data
- **HAR import**: Manual historical data import from browser sessions

### Query Methods
- **Date-based queries**: Get OHLC data for specific calendar dates
- **Height-based queries**: Get OHLC data for specific block heights with timestamps
- **Automatic aggregation**: Combines minute-level data for block intervals

### Reliability Features
- **Automatic fallback**: Tries sources in order until successful
- **Retry mechanism**: Up to 12 hours of retries with 60-second intervals
- **Cache clearing**: Automatic cache refresh on failures
- **Error handling**: Graceful degradation with detailed error messages

## Usage

### Basic Setup

```rust
use brk_fetcher::Fetcher;
use brk_structs::{Date, Height, Timestamp};

// Initialize fetcher with exchange APIs enabled
let mut fetcher = Fetcher::import(true, None)?;

// Initialize without exchange APIs (BRK-only mode)
let mut fetcher = Fetcher::import(false, None)?;

// Initialize with HAR file for historical Binance data
let har_path = Path::new("./binance.har");
let mut fetcher = Fetcher::import(true, Some(har_path))?;
```

### Date-based Price Queries

```rust
use brk_structs::Date;

// Fetch OHLC data for a specific date
let date = Date::new(2024, 12, 25);
let ohlc = fetcher.get_date(date)?;

println!("Bitcoin price on {}: ${:.2}", date, ohlc.close.dollars());
println!("Daily high: ${:.2}", ohlc.high.dollars());
println!("Daily low: ${:.2}", ohlc.low.dollars());
```

### Block Height-based Price Queries

```rust
use brk_structs::{Height, Timestamp};

// Fetch price at specific block height
let height = Height::new(900_000);
let timestamp = Timestamp::from_block_height(height);
let previous_timestamp = Some(Timestamp::from_block_height(Height::new(899_999)));

let ohlc = fetcher.get_height(height, timestamp, previous_timestamp)?;
println!("Bitcoin price at block {}: ${:.2}", height, ohlc.close.dollars());
```

### Working with OHLC Data

```rust
use brk_structs::OHLCCents;

// OHLC data is returned in cents for precision
let ohlc: OHLCCents = fetcher.get_date(date)?;

// Convert to dollars for display
println!("Open: ${:.2}", ohlc.open.dollars());
println!("High: ${:.2}", ohlc.high.dollars());
println!("Low: ${:.2}", ohlc.low.dollars());
println!("Close: ${:.2}", ohlc.close.dollars());

// Access raw cent values
println!("Close in cents: {}", ohlc.close.0);
```

### Using Individual Sources

```rust
use brk_fetcher::{Binance, Kraken, BRK};

// Use specific exchanges directly
let binance = Binance::init(None);
let kraken = Kraken::default();
let brk = BRK::default();

// Fetch from specific source
let binance_data = binance.get_from_1d(&date)?;
let kraken_data = kraken.get_from_1mn(timestamp, previous_timestamp)?;
let brk_data = brk.get_from_height(height)?;
```

### Error Handling and Retries

```rust
// The fetcher automatically retries on failures
match fetcher.get_date(date) {
    Ok(ohlc) => println!("Successfully fetched: ${:.2}", ohlc.close.dollars()),
    Err(e) => {
        // After all retries and sources exhausted
        eprintln!("Failed to fetch price data: {}", e);
    }
}

// Clear cache to force fresh data
fetcher.clear();
```

## Data Sources and Limitations

### Kraken API
- **1-day data**: Historical daily OHLC data
- **1-minute data**: Limited to last ~10 hours
- **Rate limits**: Subject to Kraken API restrictions

### Binance API  
- **1-day data**: Historical daily OHLC data
- **1-minute data**: Limited to last ~16 hours
- **HAR import**: Can extend historical coverage via browser data

### BRK Instance
- **Cached data**: Previously fetched price data
- **Offline capability**: Works without internet when data is cached
- **Height-based**: Optimized for block height queries

## HAR File Import

For historical data beyond API limits:

1. Visit [Binance BTC/USDT chart](https://www.binance.com/en/trade/BTC_USDT?type=spot)
2. Set chart to 1-minute interval
3. Open browser dev tools, go to Network tab
4. Filter by 'uiKlines'
5. Scroll chart to desired historical period
6. Export network requests as HAR file
7. Initialize fetcher with HAR path

## Fallback Strategy

The fetcher tries sources in this order:
1. **Kraken** - Primary source for most queries
2. **Binance** - Secondary source with extended coverage
3. **BRK** - Fallback for cached/computed prices

If all sources fail, it retries up to 12 hours with 60-second intervals.

## Performance and Reliability

- **Automatic retries**: Up to 720 attempts (12 hours) with 60-second delays
- **Cache management**: Clears cache on failures to force fresh data
- **Error logging**: Detailed failure reporting with recovery instructions
- **Graceful degradation**: Falls back through sources until successful

## Dependencies

- `brk_structs` - Bitcoin-aware type system (Date, Height, OHLC types)
- `brk_error` - Unified error handling
- `minreq` - HTTP client for API requests
- `serde_json` - JSON parsing for API responses
- `log` - Logging for retry and error reporting

---

*This README was generated by Claude Code*