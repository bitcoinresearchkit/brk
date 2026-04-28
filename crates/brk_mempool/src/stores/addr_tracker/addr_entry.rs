use brk_types::{AddrMempoolStats, Txid};
use rustc_hash::FxHashSet;

/// Per-address mempool record: rolling stats plus the set of live
/// txids that touch the address (used to maintain `tx_count`).
#[derive(Default)]
pub struct AddrEntry {
    pub stats: AddrMempoolStats,
    pub txids: FxHashSet<Txid>,
}
