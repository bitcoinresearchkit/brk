//! Compares results from the bitcoincore-rpc and corepc backends.
//!
//! Run with:
//!   cargo run -p brk_rpc --example compare_backends --features corepc

use std::time::{Duration, Instant};

#[cfg(not(all(feature = "bitcoincore-rpc", feature = "corepc")))]
fn main() {
    eprintln!("This example requires both features: --features bitcoincore-rpc,corepc");
    std::process::exit(1);
}

#[cfg(all(feature = "bitcoincore-rpc", feature = "corepc"))]
fn main() {
    use brk_rpc::backend::{self, Auth};

    brk_logger::init(None).unwrap();

    let bitcoin_dir = brk_rpc::Client::default_bitcoin_path();
    let auth = Auth::CookieFile(bitcoin_dir.join(".cookie"));
    let url = brk_rpc::Client::default_url();

    let bc = backend::bitcoincore::ClientInner::new(url, auth.clone(), 10, Duration::from_secs(1))
        .expect("bitcoincore client");
    let cp = backend::corepc::ClientInner::new(url, auth, 10, Duration::from_secs(1))
        .expect("corepc client");

    println!("=== Comparing backends ===\n");

    // --- get_blockchain_info ---
    {
        let (t1, r1) = timed(|| bc.get_blockchain_info());
        let (t2, r2) = timed(|| cp.get_blockchain_info());
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_blockchain_info:");
        println!(
            "  bitcoincore: headers={} blocks={} ({t1:?})",
            r1.headers, r1.blocks
        );
        println!(
            "  corepc:      headers={} blocks={} ({t2:?})",
            r2.headers, r2.blocks
        );
        assert_eq!(r1.headers, r2.headers, "headers mismatch");
        assert_eq!(r1.blocks, r2.blocks, "blocks mismatch");
        println!("  MATCH\n");
    }

    // --- get_block_count ---
    {
        let (t1, r1) = timed(|| bc.get_block_count());
        let (t2, r2) = timed(|| cp.get_block_count());
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_block_count:");
        println!("  bitcoincore: {r1} ({t1:?})");
        println!("  corepc:      {r2} ({t2:?})");
        assert_eq!(r1, r2, "block count mismatch");
        println!("  MATCH\n");
    }

    // --- get_block_hash (height 0) ---
    let genesis_hash;
    {
        let (t1, r1) = timed(|| bc.get_block_hash(0));
        let (t2, r2) = timed(|| cp.get_block_hash(0));
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        genesis_hash = r1;
        println!("get_block_hash(0):");
        println!("  bitcoincore: {r1} ({t1:?})");
        println!("  corepc:      {r2} ({t2:?})");
        assert_eq!(r1, r2, "genesis hash mismatch");
        println!("  MATCH\n");
    }

    // --- get_block_header ---
    {
        let (t1, r1) = timed(|| bc.get_block_header(&genesis_hash));
        let (t2, r2) = timed(|| cp.get_block_header(&genesis_hash));
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_block_header(genesis):");
        println!("  bitcoincore: prev={} ({t1:?})", r1.prev_blockhash);
        println!("  corepc:      prev={} ({t2:?})", r2.prev_blockhash);
        assert_eq!(r1, r2, "header mismatch");
        println!("  MATCH\n");
    }

    // --- get_block_info ---
    {
        let (t1, r1) = timed(|| bc.get_block_info(&genesis_hash));
        let (t2, r2) = timed(|| cp.get_block_info(&genesis_hash));
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_block_info(genesis):");
        println!(
            "  bitcoincore: height={} confirmations={} ({t1:?})",
            r1.height, r1.confirmations
        );
        println!(
            "  corepc:      height={} confirmations={} ({t2:?})",
            r2.height, r2.confirmations
        );
        assert_eq!(r1.height, r2.height, "height mismatch");
        // confirmations can drift by 1 between calls
        assert!(
            (r1.confirmations - r2.confirmations).abs() <= 1,
            "confirmations mismatch: {} vs {}",
            r1.confirmations,
            r2.confirmations
        );
        println!("  MATCH\n");
    }

    // --- get_block_header_info ---
    {
        let (t1, r1) = timed(|| bc.get_block_header_info(&genesis_hash));
        let (t2, r2) = timed(|| cp.get_block_header_info(&genesis_hash));
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_block_header_info(genesis):");
        println!(
            "  bitcoincore: height={} prev={:?} ({t1:?})",
            r1.height, r1.previous_block_hash
        );
        println!(
            "  corepc:      height={} prev={:?} ({t2:?})",
            r2.height, r2.previous_block_hash
        );
        assert_eq!(r1.height, r2.height, "height mismatch");
        assert_eq!(
            r1.previous_block_hash, r2.previous_block_hash,
            "prev hash mismatch"
        );
        println!("  MATCH\n");
    }

    // --- get_block (genesis) ---
    {
        let (t1, r1) = timed(|| bc.get_block(&genesis_hash));
        let (t2, r2) = timed(|| cp.get_block(&genesis_hash));
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_block(genesis):");
        println!("  bitcoincore: txs={} ({t1:?})", r1.txdata.len());
        println!("  corepc:      txs={} ({t2:?})", r2.txdata.len());
        assert_eq!(r1, r2, "block mismatch");
        println!("  MATCH\n");
    }

    // --- get_raw_mempool ---
    {
        let (t1, r1) = timed(|| bc.get_raw_mempool());
        let (t2, r2) = timed(|| cp.get_raw_mempool());
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_raw_mempool:");
        println!("  bitcoincore: {} txs ({t1:?})", r1.len());
        println!("  corepc:      {} txs ({t2:?})", r2.len());
        // Mempool can change between calls, just check they're reasonable
        println!(
            "  {} (mempool is live, counts may differ slightly)\n",
            if r1.len() == r2.len() {
                "MATCH"
            } else {
                "CLOSE"
            }
        );
    }

    // --- get_raw_mempool_verbose ---
    {
        let (t1, r1) = timed(|| bc.get_raw_mempool_verbose());
        let (t2, r2) = timed(|| cp.get_raw_mempool_verbose());
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_raw_mempool_verbose:");
        println!("  bitcoincore: {} entries ({t1:?})", r1.len());
        println!("  corepc:      {} entries ({t2:?})", r2.len());

        // Compare a sample entry if both have data
        if let (Some((txid1, e1)), Some(_)) = (r1.first(), r2.first())
            && let Some((_, e2)) = r2.iter().find(|(t, _)| t == txid1)
        {
            println!("  sample txid {txid1}:");
            println!(
                "    bitcoincore: vsize={} fee={} ancestor_count={}",
                e1.vsize, e1.base_fee_sats, e1.ancestor_count
            );
            println!(
                "    corepc:      vsize={} fee={} ancestor_count={}",
                e2.vsize, e2.base_fee_sats, e2.ancestor_count
            );
            assert_eq!(e1.base_fee_sats, e2.base_fee_sats, "fee mismatch");
            assert_eq!(
                e1.ancestor_count, e2.ancestor_count,
                "ancestor_count mismatch"
            );
            println!("    MATCH");
        }
        println!();
    }

    // --- get_raw_transaction_hex (tx from block 1, genesis coinbase can't be retrieved) ---
    let block1_hash;
    {
        block1_hash = bc.get_block_hash(1).unwrap();
        let block = bc.get_block(&block1_hash).unwrap();
        let coinbase_txid = block.txdata[0].compute_txid();
        let (t1, r1) = timed(|| bc.get_raw_transaction_hex(&coinbase_txid, Some(&block1_hash)));
        let (t2, r2) = timed(|| cp.get_raw_transaction_hex(&coinbase_txid, Some(&block1_hash)));
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_raw_transaction_hex(block 1 coinbase):");
        println!("  bitcoincore: {}... ({t1:?})", &r1[..40.min(r1.len())]);
        println!("  corepc:      {}... ({t2:?})", &r2[..40.min(r2.len())]);
        assert_eq!(r1, r2, "raw tx hex mismatch");
        println!("  MATCH\n");
    }

    // --- get_tx_out (genesis coinbase, likely unspendable but test the call) ---
    {
        let block = bc.get_block(&genesis_hash).unwrap();
        let coinbase_txid = block.txdata[0].compute_txid();
        let (t1, r1) = timed(|| bc.get_tx_out(&coinbase_txid, 0, Some(false)));
        let (t2, r2) = timed(|| cp.get_tx_out(&coinbase_txid, 0, Some(false)));
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        println!("get_tx_out(genesis coinbase, vout=0):");
        match (&r1, &r2) {
            (Some(a), Some(b)) => {
                println!(
                    "  bitcoincore: coinbase={} value={:?} ({t1:?})",
                    a.coinbase, a.value
                );
                println!(
                    "  corepc:      coinbase={} value={:?} ({t2:?})",
                    b.coinbase, b.value
                );
                assert_eq!(a.coinbase, b.coinbase, "coinbase mismatch");
                assert_eq!(a.value, b.value, "value mismatch");
                assert_eq!(a.script_pub_key, b.script_pub_key, "script mismatch");
                println!("  MATCH");
            }
            (None, None) => {
                println!("  both: None (spent) ({t1:?} / {t2:?})");
                println!("  MATCH");
            }
            _ => {
                println!("  MISMATCH: bitcoincore={r1:?}, corepc={r2:?}");
                panic!("get_tx_out mismatch");
            }
        }
        println!();
    }

    println!("=== All checks passed ===");
}

fn timed<T>(f: impl FnOnce() -> T) -> (Duration, T) {
    let start = Instant::now();
    let result = f();
    (start.elapsed(), result)
}
