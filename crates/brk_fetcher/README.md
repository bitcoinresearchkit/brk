# brk_fetcher

Bitcoin price fetcher that retrieves historical OHLC (Open, High, Low, Close) data by date or block height from multiple sources including Binance, Kraken, and the main BRK instance. This crate provides a unified interface with automatic fallback between exchanges and retry logic for reliable price data collection.

## Features

- **Multiple data sources**: Binance, Kraken APIs, and BRK instance
- **Flexible queries**: Fetch prices by date or block height with timestamp
- **Automatic fallback**: Tries sources in order (Kraken → Binance → BRK)
- **Retry logic**: Built-in retry mechanism
- **Time resolution**: 1-minute and 1-day interval support
- **HAR file import**: Import Binance chart data from browser for historical prices

## Usage

```rust
use brk_fetcher::Fetcher;
use brk_structs::{Date, Height};

fn main() -> brk_error::Result<()> {
    // Initialize fetcher with exchange APIs enabled
    let mut fetcher = Fetcher::import(true, None)?;

    // Fetch price by date
    let price = fetcher.get_date(Date::new(2025, 1, 15))?;
    println!("Price on 2025-01-15: ${}", price.close.dollars());

    // Fetch price by block height
    let price = fetcher.get_height(
        Height::new(900_000),
        timestamp,
        previous_timestamp,
    )?;
    println!("Price at block 900,000: ${}", price.close.dollars());

    Ok(())
}
```

## Individual Sources

Each exchange can be used independently:

```rust
use brk_fetcher::{Binance, Kraken, BRK};

// Fetch from specific exchanges
let binance_data = Binance::fetch_1d()?;
let kraken_data = Kraken::fetch_1mn()?;
let brk_data = BRK::default().get_from_height(Height::new(800_000))?;
```

## Limitations

- **1-minute data**: Limited to last 16 hours (Binance) or 10 hours (Kraken)
- **Network dependent**: Requires internet connection for exchange APIs
- **Rate limits**: Subject to exchange API rate limiting
