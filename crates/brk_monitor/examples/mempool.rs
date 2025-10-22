use std::{path::Path, thread, time::Duration};

use brk_error::Result;
use brk_monitor::Mempool;
use brk_rpc::{Auth, Client};

fn main() -> Result<()> {
    // Connect to Bitcoin Core
    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK/bitcoin");

    let client = Client::new(
        "http://localhost:8332",
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let mempool = Mempool::new(client);

    let mempool_clone = mempool.clone();
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

    // Ok(())
}
