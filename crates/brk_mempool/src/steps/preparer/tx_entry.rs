use brk_types::{FeeRate, MempoolEntryInfo, Sats, Timestamp, Txid, TxidPrefix, VSize};
use smallvec::SmallVec;

/// A mempool transaction entry.
///
/// Stores only immutable per-tx facts. Ancestor aggregates are
/// deliberately not cached: they're derivable from the live
/// dependency graph, and any cached copy would go stale the moment
/// any ancestor confirms or is replaced.
#[derive(Debug, Clone)]
pub struct TxEntry {
    pub txid: Txid,
    pub fee: Sats,
    pub vsize: VSize,
    /// Serialized tx size in bytes (witness + non-witness), from the raw tx.
    pub size: u64,
    /// Parent txid prefixes (most txs have 0-2 parents).
    ///
    /// May reference parents no longer in the pool. Consumers resolve
    /// against the live pool and drop misses, so staleness here is
    /// self-healing.
    pub depends: SmallVec<[TxidPrefix; 2]>,
    pub first_seen: Timestamp,
    /// BIP-125 explicit signaling: any input has sequence < 0xfffffffe.
    pub rbf: bool,
}

impl TxEntry {
    pub(super) fn new(info: &MempoolEntryInfo, size: u64, rbf: bool, first_seen: Timestamp) -> Self {
        Self {
            txid: info.txid.clone(),
            fee: info.fee,
            vsize: VSize::from(info.vsize),
            size,
            depends: info.depends.iter().map(TxidPrefix::from).collect(),
            first_seen,
            rbf,
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
