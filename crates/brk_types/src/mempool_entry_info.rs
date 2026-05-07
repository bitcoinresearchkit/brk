use crate::{FeeRate, Sats, Timestamp, Txid, VSize, Weight};

/// Mempool entry info from Bitcoin Core's `getrawmempool true`.
#[derive(Debug, Clone)]
pub struct MempoolEntryInfo {
    pub txid: Txid,
    pub vsize: VSize,
    pub weight: Weight,
    pub fee: Sats,
    pub first_seen: Timestamp,
    pub ancestor_count: u64,
    pub ancestor_size: VSize,
    pub ancestor_fee: Sats,
    pub descendant_size: VSize,
    pub descendant_fee: Sats,
    /// Total fee of the cluster mempool chunk this tx belongs to.
    /// Present from Bitcoin Core 31+ (cluster mempool); absent on
    /// older Core, in which case rate-callers fall back to
    /// `max(ancestor_rate, descendant_pkg_rate)`.
    pub chunk_fee: Option<Sats>,
    pub chunk_weight: Option<Weight>,
    /// Parent txids in the mempool.
    pub depends: Vec<Txid>,
}

impl MempoolEntryInfo {
    /// Effective per-vbyte rate Core would mine this tx at. Uses the
    /// Core-31 `fees.chunk` / `chunkweight` chunk fields when present;
    /// otherwise falls back to `max(ancestor_rate, descendant_pkg_rate)`,
    /// which bounds the predictive error in deep clusters.
    pub fn chunk_rate(&self) -> FeeRate {
        if let (Some(chunk_fee), Some(chunk_weight)) = (self.chunk_fee, self.chunk_weight) {
            return FeeRate::from((chunk_fee, VSize::from(chunk_weight)));
        }
        let anc = FeeRate::from((self.ancestor_fee, self.ancestor_size));
        let desc = FeeRate::from((self.descendant_fee, self.descendant_size));
        anc.max(desc)
    }
}
