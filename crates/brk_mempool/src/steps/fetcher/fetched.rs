use brk_rpc::MempoolState;
use brk_types::{MempoolEntryInfo, Txid};
use rustc_hash::FxHashMap;

pub struct Fetched {
    /// Passthrough fields from the batched RPC fetch: live txid set,
    /// fee floor, chain tip. `live_txids` is the union of
    /// `getrawmempool` and `getblocktemplate` (see [`super::Fetcher::fetch`]),
    /// so downstream sees a single coherent "live" view.
    pub state: MempoolState,
    /// `MempoolEntryInfo` for newly-observed txids only (existing ones
    /// keep their first-sight entry on the live store).
    pub new_entries: Vec<MempoolEntryInfo>,
    pub new_txs: FxHashMap<Txid, bitcoin::Transaction>,
    /// Block 0 ordering from `getblocktemplate`. Bodies and stats have
    /// already been folded into `new_entries`/`new_txs` (or were already
    /// in the pool). The Rebuilder only needs the txid sequence to
    /// project Core's exact selection.
    pub block_template_txids: Vec<Txid>,
}
