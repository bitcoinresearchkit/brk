use crate::{Sats, Txid};

/// Mempool entry info from Bitcoin Core's getrawmempool verbose
#[derive(Debug, Clone)]
pub struct MempoolEntryInfo {
    pub txid: Txid,
    pub vsize: u64,
    pub weight: u64,
    pub fee: Sats,
    pub ancestor_count: u64,
    pub ancestor_size: u64,
    pub ancestor_fee: Sats,
    /// Parent txids in the mempool
    pub depends: Vec<Txid>,
}
