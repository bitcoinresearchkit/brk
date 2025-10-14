use std::{path::Path, sync::Arc, thread, time::Duration};

use brk_monitor::Mempool;

fn main() {
    // Connect to Bitcoin Core
    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK/bitcoin");

    let rpc = Box::leak(Box::new(
        bitcoincore_rpc::Client::new(
            "http://localhost:8332",
            bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
        )
        .unwrap(),
    ));

    let mempool = Arc::new(Mempool::new(rpc));

    // Spawn monitoring thread
    let mempool_clone = Arc::clone(&mempool);
    thread::spawn(move || {
        mempool_clone.start();
    });

    // Access from main thread
    loop {
        thread::sleep(Duration::from_secs(5));
        let txs = mempool.get_txs();
        println!("mempool_tx_count: {}", txs.len());
        let addresses = mempool.get_addresses();
        println!("mempool_address_count: {}", addresses.len());
    }
}
