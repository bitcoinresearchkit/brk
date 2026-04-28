use brk_rpc::RawTx;
use brk_types::{MempoolEntryInfo, Txid};
use rustc_hash::FxHashMap;

pub struct Fetched {
    pub entries_info: Vec<MempoolEntryInfo>,
    pub new_raws: FxHashMap<Txid, RawTx>,
    pub parent_raws: FxHashMap<Txid, RawTx>,
}
