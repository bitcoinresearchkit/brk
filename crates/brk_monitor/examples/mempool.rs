use std::{thread, time::Duration};

use brk_error::Result;
use brk_monitor::Mempool;
use brk_rpc::{Auth, Client};

fn main() -> Result<()> {
    brk_logger::init(None)?;

    let bitcoin_dir = Client::default_bitcoin_path();
    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let mempool = Mempool::new(&client);

    // Start mempool sync in background thread
    let mempool_clone = mempool.clone();
    thread::spawn(move || {
        mempool_clone.start();
    });

    // Poll and display stats every 5 seconds
    loop {
        thread::sleep(Duration::from_secs(5));

        // Basic mempool info
        let info = mempool.get_info();
        let block_stats = mempool.get_block_stats();
        let total_fees: u64 = block_stats.iter().map(|s| u64::from(s.total_fee)).sum();
        println!("\n=== Mempool Info ===");
        println!("  Transactions: {}", info.count);
        println!("  Total vsize:  {} vB", info.vsize);
        println!(
            "  Total fees:   {:.4} BTC",
            total_fees as f64 / 100_000_000.0
        );

        // Fee recommendations (like mempool.space)
        let fees = mempool.get_fees();
        println!("\n=== Recommended Fees (sat/vB) ===");
        println!("  No Priority     {:.4}", f64::from(fees.economy_fee));
        println!("  Low Priority    {:.4}", f64::from(fees.hour_fee));
        println!("  Medium Priority {:.4}", f64::from(fees.half_hour_fee));
        println!("  High Priority   {:.4}", f64::from(fees.fastest_fee));

        // Projected blocks (like mempool.space)
        if !block_stats.is_empty() {
            println!("\n=== Projected Blocks ===");
            for (i, stats) in block_stats.iter().enumerate() {
                let total_fee_btc = u64::from(stats.total_fee) as f64 / 100_000_000.0;
                println!(
                    "  Block {}: ~{:.4} sat/vB, {:.4}-{:.4} sat/vB, {:.3} BTC, {} txs",
                    i + 1,
                    f64::from(stats.median_fee_rate()),
                    f64::from(stats.min_fee_rate()),
                    f64::from(stats.max_fee_rate()),
                    total_fee_btc,
                    stats.tx_count,
                );
            }
        }

        // Address tracking stats
        let addresses = mempool.get_addresses();
        println!("\n=== Address Tracking ===");
        println!("  Addresses with pending txs: {}", addresses.len());

        println!("\n----------------------------------------");
    }
}
