//! Basic example of using the BRK client.

use brk_client::{BrkClient, BrkClientOptions};
use brk_types::{FormatResponse, Index, Metric};

fn main() -> brk_client::Result<()> {
    // Create client with default options
    let client = BrkClient::new("http://localhost:3110");

    // Or with custom options
    let _client_with_options = BrkClient::with_options(BrkClientOptions {
        base_url: "http://localhost:3110".to_string(),
        timeout_secs: 60,
    });

    // Fetch price data using the typed metrics API
    let price_close = client
        .metrics()
        .price
        .usd
        .split
        .close
        .by
        .dateindex()
        .from(-3)
        .json()?;
    println!("Last 3 price close values: {:?}", price_close);

    // Fetch block data
    let block_count = client
        .metrics()
        .blocks
        .count
        .block_count
        .sum
        .by
        .dateindex()
        .from(-3)
        .json()?;
    println!("Last 3 block count values: {:?}", block_count);

    // Fetch supply data
    //
    dbg!(
        client
            .metrics()
            .supply
            .circulating
            .bitcoin
            .by
            .dateindex()
            .path()
    );
    let circulating = client
        .metrics()
        .supply
        .circulating
        .bitcoin
        .by
        .dateindex()
        .from(-3)
        .csv()?;
    println!("Last 3 circulating supply values: {:?}", circulating);

    // Using generic metric fetching
    let metricdata = client.get_metric(
        Metric::from("price_close"),
        Index::DateIndex,
        Some(-3),
        None,
        None,
        None,
    )?;
    match metricdata {
        FormatResponse::Json(m) => {
            println!("Generic fetch result count: {}", m.data.len());
        }
        FormatResponse::Csv(_) => panic!(),
    };

    Ok(())
}
