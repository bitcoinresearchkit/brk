use std::{thread, time::Duration};

use brk_error::Result;
use brk_mempool::{Mempool, MempoolStats};
use brk_rpc::{Auth, Client};

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

        let cluster_nodes_total: usize = snapshot.clusters.iter().map(|c| c.nodes.len()).sum();
        let blocks_tx_total: usize = snapshot.blocks.iter().map(|b| b.len()).sum();
        let (skip_clean, skip_throttled) = mempool.skip_counts();

        println!(
            "info.count={} entries.slots={} entries.active={} entries.free={} \
             txs={} unresolved={} addrs={} outpoints={} \
             graveyard.tombstones={} graveyard.order={} \
             snap.clusters={} snap.cluster_nodes={} snap.cluster_of.len={} snap.cluster_of.active={} \
             snap.blocks={} snap.blocks_txs={} \
             rebuilds={} skip.clean={} skip.throttled={}",
            stats.info_count,
            stats.entry_slot_count,
            stats.entry_active_count,
            stats.entry_free_count,
            stats.tx_count,
            stats.unresolved_count,
            stats.addr_count,
            stats.outpoint_spend_count,
            stats.graveyard_tombstone_count,
            stats.graveyard_order_count,
            snapshot.clusters.len(),
            cluster_nodes_total,
            snapshot.cluster_of_len(),
            snapshot.cluster_of_active(),
            snapshot.blocks.len(),
            blocks_tx_total,
            mempool.rebuild_count(),
            skip_clean,
            skip_throttled,
        );
    }
}
