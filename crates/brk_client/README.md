# brk_client

Rust client for the [Bitcoin Research Kit](https://github.com/bitcoinresearchkit/brk) API.

[crates.io](https://crates.io/crates/brk_client) | [docs.rs](https://docs.rs/brk_client)

## Installation

```toml
[dependencies]
brk_client = "0.1"
```

## Quick Start

```rust
use brk_client::{BrkClient, Index};

fn main() -> brk_client::Result<()> {
    let client = BrkClient::new("http://localhost:3110");

    // Blockchain data (mempool.space compatible)
    let block = client.get_block_by_height(800000)?;
    let tx = client.get_tx("a1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d")?;
    let address = client.get_address("bc1q...")?;

    // Metrics API - typed, chainable
    let prices = client.metrics()
        .price.usd.split.close
        .by.dateindex()
        .range(Some(-30), None)?; // Last 30 days

    // Generic metric fetching
    let data = client.get_metric(
        "price_close".into(),
        Index::DateIndex,
        Some(-30), None, None, None,
    )?;

    Ok(())
}
```

## Configuration

```rust
use brk_client::{BrkClient, BrkClientOptions};

let client = BrkClient::with_options(BrkClientOptions {
    base_url: "http://localhost:3110".to_string(),
    timeout_secs: 60,
});
```
