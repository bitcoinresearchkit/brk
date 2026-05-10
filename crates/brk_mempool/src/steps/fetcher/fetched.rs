use brk_rpc::BlockTemplateTx;
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
    pub gbt: Vec<BlockTemplateTx>,
    pub min_fee: FeeRate,
}
