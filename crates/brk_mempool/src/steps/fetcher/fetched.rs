use brk_rpc::{BlockTemplateTx, RawTx};
use brk_types::{FeeRate, MempoolEntryInfo, Txid};
use rustc_hash::FxHashMap;

pub struct Fetched {
    pub entries_info: Vec<MempoolEntryInfo>,
    pub new_raws: FxHashMap<Txid, RawTx>,
    pub gbt: Vec<BlockTemplateTx>,
    pub min_fee: FeeRate,
}
