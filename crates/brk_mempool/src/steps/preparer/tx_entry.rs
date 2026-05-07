use brk_types::{FeeRate, MempoolEntryInfo, Sats, Timestamp, Txid, TxidPrefix, VSize, Weight};
use smallvec::SmallVec;

/// A mempool transaction entry. Carries the per-tx facts needed for
/// projection, plus the snapshot-time `chunk_rate` (Core's cluster-mempool
/// chunk fee rate, or the proxy fallback) used as the effective rate
/// for partitioning, fee tiers, and CPFP.
#[derive(Debug, Clone)]
pub struct TxEntry {
    pub txid: Txid,
    pub fee: Sats,
    pub vsize: VSize,
    pub weight: Weight,
    /// Serialized tx size in bytes (witness + non-witness).
    pub size: u64,
    pub depends: SmallVec<[TxidPrefix; 2]>,
    pub first_seen: Timestamp,
    /// BIP-125 explicit signaling: any input has sequence < 0xfffffffe.
    pub rbf: bool,
    /// Effective per-vbyte rate Core would mine this tx at. From
    /// `MempoolEntryInfo::chunk_rate()`: Core 31+ uses `fees.chunk /
    /// (chunkweight/4)`, older Core falls back to
    /// `max(ancestor_rate, descendant_pkg_rate)`.
    pub chunk_rate: FeeRate,
}

impl TxEntry {
    pub(super) fn new(info: &MempoolEntryInfo, size: u64, rbf: bool) -> Self {
        Self {
            txid: info.txid,
            fee: info.fee,
            vsize: info.vsize,
            weight: info.weight,
            size,
            depends: info.depends.iter().map(TxidPrefix::from).collect(),
            first_seen: info.first_seen,
            rbf,
            chunk_rate: info.chunk_rate(),
        }
    }

    #[inline]
    pub fn fee_rate(&self) -> FeeRate {
        FeeRate::from((self.fee, self.vsize))
    }

    #[inline]
    pub fn txid_prefix(&self) -> TxidPrefix {
        TxidPrefix::from(&self.txid)
    }
}
