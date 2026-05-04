use crate::{Sats, Timestamp, Txid, VSize, Weight};

/// Mempool entry info from Bitcoin Core's getrawmempool verbose
#[derive(Debug, Clone)]
pub struct MempoolEntryInfo {
    pub txid: Txid,
    pub vsize: VSize,
    pub weight: Weight,
    pub fee: Sats,
    pub first_seen: Timestamp,
    pub ancestor_count: u64,
    pub ancestor_size: u64,
    pub ancestor_fee: Sats,
    /// Parent txids in the mempool
    pub depends: Vec<Txid>,
}
