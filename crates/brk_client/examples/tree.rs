//! Comprehensive test that fetches all endpoints in the tree.
//!
//! This example demonstrates how to iterate over all metrics and fetch data
//! from each endpoint. Run with: cargo run --example test_all_endpoints

use brk_client::{BrkClient, Index};

fn main() -> brk_client::Result<()> {
    let client = BrkClient::new("http://localhost:3110");

    // Get all metrics from the tree
    let metrics = client.all_metrics();
    println!("\nFound {} metrics", metrics.len());

    let mut success = 0;
    let mut failed = 0;
    let mut errors: Vec<String> = Vec::new();

    for metric in &metrics {
        let name = metric.name();
        let indexes = metric.indexes();

        for index in indexes {
            let path = format!("/api/metric/{}/{}", name, index.serialize_long());
            match client.get::<serde_json::Value>(&format!("{}?to=-3", path)) {
                Ok(data) => {
                    let count = data
                        .get("data")
                        .and_then(|d| d.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0);
                    if count != 3 {
                        failed += 1;
                        let error_msg = format!(
                            "FAIL: {}.{} -> expected 3, got {}",
                            name,
                            index.serialize_long(),
                            count
                        );
                        errors.push(error_msg.clone());
                        println!("{}", error_msg);
                    } else {
                        success += 1;
                        println!("OK: {}.{} -> {} items", name, index.serialize_long(), count);
                    }
                }
                Err(e) => {
                    failed += 1;
                    let error_msg = format!("FAIL: {}.{} -> {}", name, index.serialize_long(), e);
                    errors.push(error_msg.clone());
                    println!("{}", error_msg);
                }
            }
        }
    }

    println!("\n=== Results ===");
    println!("Success: {}", success);
    println!("Failed: {}", failed);

    if !errors.is_empty() {
        println!("\nErrors:");
        for err in errors.iter().take(10) {
            println!("  {}", err);
        }
        if errors.len() > 10 {
            println!("  ... and {} more", errors.len() - 10);
        }
    }

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
