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
    // day1() returns DateMetricEndpointBuilder, so fetch() returns DateMetricData
    let price_close = client
        .metrics()
        .prices
        .usd
        .split
        .close
        .by
        .day1()
        .last(3)
        .fetch()?;
    println!("Last 3 price close values:");
    // iter_dates() returns Option (None for sub-daily indexes)
    for (date, value) in price_close.iter_dates().unwrap() {
        println!("  {}: {}", date, value);
    }
    // iter_timestamps() works for all date-based indexes including sub-daily
    for (ts, value) in price_close.iter_timestamps() {
        println!("  {}: {}", ts, value);
    }

    // Fetch block data with height index (non-date, returns MetricData)
    let block_count = client
        .metrics()
        .blocks
        .count
        .block_count
        .sum
        ._24h
        .by
        .day1()
        .last(3)
        .fetch()?;
    println!("Last 3 block count values:");
    for (date, value) in block_count.iter_dates().unwrap() {
        println!("  {}: {}", date, value);
    }

    // Fetch supply data as CSV
    dbg!(client.metrics().supply.circulating.btc.by.day1().path());
    let circulating = client
        .metrics()
        .supply
        .circulating
        .btc
        .by
        .day1()
        .last(3)
        .fetch_csv()?;
    println!("Last 3 circulating supply (CSV): {:?}", circulating);

    // Using dynamic metric fetching with date_metric() for date-based indexes
    let date_metric = client
        .date_metric(Metric::from("price_close"), Index::Day1)?
        .last(3)
        .fetch()?;
    println!("Dynamic date metric fetch:");
    for (date, value) in date_metric.iter_dates().unwrap() {
        println!("  {}: {}", date, value);
    }

    // Using generic metric fetching (returns FormatResponse)
    let metricdata = client.get_metric(
        Metric::from("price_close"),
        Index::Day1,
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
