use std::{thread, time::Duration};

use brk_error::Result;
use brk_mempool::Mempool;
use brk_rpc::{Auth, Client};

#[derive(Debug, Clone)]
struct MempoolStats {
    info_count: usize,
    tx_count: usize,
    unresolved_count: usize,
    addr_count: usize,
    outpoint_spend_count: usize,
    graveyard_tombstone_count: usize,
    graveyard_order_count: usize,
}

impl From<&Mempool> for MempoolStats {
    fn from(mempool: &Mempool) -> Self {
        Self {
            info_count: mempool.info().count,
            tx_count: mempool.tx_count(),
            unresolved_count: mempool.unresolved_count(),
            addr_count: mempool.addr_count(),
            outpoint_spend_count: mempool.outpoint_spend_count(),
            graveyard_tombstone_count: mempool.graveyard_tombstone_count(),
            graveyard_order_count: mempool.graveyard_order_count(),
        }
    }
}

fn main() -> Result<()> {
    brk_logger::init(None)?;

    let bitcoin_dir = Client::default_bitcoin_path();
    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let mempool = Mempool::new(&client);

    let mempool_clone = mempool.clone();
    thread::spawn(move || {
        mempool_clone.start();
    });

    loop {
        thread::sleep(Duration::from_secs(5));

        let stats = MempoolStats::from(&mempool);
        let snapshot = mempool.snapshot();

        let blocks_tx_total: usize = snapshot.blocks.iter().map(|b| b.len()).sum();

        println!(
            "info.count={} txs={} unresolved={} addrs={} outpoints={} \
             graveyard.tombstones={} graveyard.order={} \
             snap.txs.len={} snap.blocks={} snap.blocks_txs={} \
             rebuilds={} skip.clean={}",
            stats.info_count,
            stats.tx_count,
            stats.unresolved_count,
            stats.addr_count,
            stats.outpoint_spend_count,
            stats.graveyard_tombstone_count,
            stats.graveyard_order_count,
            snapshot.txs_len(),
            snapshot.blocks.len(),
            blocks_tx_total,
            mempool.rebuild_count(),
            mempool.skip_clean_count(),
        );
    }
}
