//! Basic example of using the BRK client.

use brk_client::{BrkClient, BrkClientOptions};

fn main() -> brk_client::Result<()> {
    // Create client with default options
    let client = BrkClient::new("http://localhost:3110");

    // Or with custom options
    let _client_with_options = BrkClient::with_options(BrkClientOptions {
        base_url: "http://localhost:3110".to_string(),
        timeout_secs: 60,
    });

    // Fetch price data using the typed tree API
    let price_close = client
        .tree()
        .price
        .usd
        .split
        .close
        .by
        .dateindex()
        .range(Some(-3), None)?;
    println!("Last 3 price close values: {:?}", price_close);

    // Fetch block data
    let block_count = client
        .tree()
        .blocks
        .count
        .block_count
        .sum
        .by
        .dateindex()
        .range(Some(-3), None)?;
    println!("Last 3 block count values: {:?}", block_count);

    // Fetch supply data
    //
    dbg!(
        client
            .tree()
            .supply
            .circulating
            .bitcoin
            .by
            .dateindex()
            .path()
    );
    let circulating = client
        .tree()
        .supply
        .circulating
        .bitcoin
        .by
        .dateindex()
        .range(Some(-3), None)?;
    println!("Last 3 circulating supply values: {:?}", circulating);

    // Using generic metric fetching
    let metricdata =
        client.get_metric_by_index("dateindex", "price_close", None, None, Some("-3"), None)?;
    println!("Generic fetch result count: {}", metricdata.data.len());

    Ok(())
}
