use brk_types::{FeeRate, MempoolEntryInfo, Txid};
use rustc_hash::FxHashMap;

pub struct Fetched {
    /// Every txid currently in the mempool (from `getrawmempool false`).
    /// Used to derive the `live` set for removal classification.
    pub live_txids: Vec<Txid>,
    /// `MempoolEntryInfo` for newly-observed txids only (existing ones
    /// keep their first-sight entry on the live store).
    pub new_entries: Vec<MempoolEntryInfo>,
    pub new_txs: FxHashMap<Txid, bitcoin::Transaction>,
    /// Block 0 ordering from `getblocktemplate`. Bodies and stats have
    /// already been folded into `new_entries`/`new_txs` (or were already
    /// in the pool); the Rebuilder only needs the txid sequence to
    /// project Core's exact selection.
    pub gbt_txids: Vec<Txid>,
    pub min_fee: FeeRate,
}
