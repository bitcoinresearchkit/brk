//! Comprehensive test that fetches all endpoints in the tree.
//!
//! This example demonstrates how to recursively traverse the metrics catalog tree
//! and fetch data from each endpoint. Run with: cargo run --example tree

use brk_client::BrkClient;
use brk_types::{Index, TreeNode};
use std::collections::BTreeSet;

/// A collected metric with its path and available indexes.
struct CollectedMetric {
    path: String,
    name: String,
    indexes: BTreeSet<Index>,
}

/// Recursively collect all metrics from the tree.
fn collect_metrics(node: &TreeNode, path: &str) -> Vec<CollectedMetric> {
    let mut metrics = Vec::new();

    match node {
        TreeNode::Branch(children) => {
            for (key, child) in children {
                let child_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };
                metrics.extend(collect_metrics(child, &child_path));
            }
        }
        TreeNode::Leaf(leaf) => {
            metrics.push(CollectedMetric {
                path: path.to_string(),
                name: leaf.name().to_string(),
                indexes: leaf.indexes().clone(),
            });
        }
    }

    metrics
}

fn main() -> brk_client::Result<()> {
    let client = BrkClient::new("http://localhost:3110");

    // Get the metrics catalog tree
    let tree = client.get_metrics_tree()?;

    // Recursively collect all metrics
    let metrics = collect_metrics(&tree, "");
    println!("\nFound {} metrics", metrics.len());

    let mut success = 0;
    let mut failed = 0;
    let mut errors: Vec<String> = Vec::new();

    for metric in &metrics {
        for index in &metric.indexes {
            let index_str = index.serialize_long();
            match client.get_metric(
                metric.name.as_str().into(),
                *index,
                None,
                None,
                Some("-3"),
                None,
            ) {
                Ok(response) => {
                    let count = response.json().data.len();
                    if count != 3 {
                        failed += 1;
                        let error_msg = format!(
                            "FAIL: {}.by.{} -> expected 3, got {}",
                            metric.path, index_str, count
                        );
                        errors.push(error_msg.clone());
                        println!("{}", error_msg);
                    } else {
                        success += 1;
                        println!("OK: {}.by.{} -> {} items", metric.path, index_str, count);
                    }
                }
                Err(e) => {
                    failed += 1;
                    let error_msg = format!("FAIL: {}.by.{} -> {}", metric.path, index_str, e);
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
