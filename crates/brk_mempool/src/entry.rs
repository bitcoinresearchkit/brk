use brk_types::{FeeRate, Sats, Timestamp, Txid, TxidPrefix, VSize};
use smallvec::SmallVec;

/// A mempool transaction entry.
///
/// Stores only the data needed for fee estimation and block building.
/// Ancestor values are pre-computed by Bitcoin Core (correctly handling shared ancestors).
#[derive(Debug, Clone)]
pub struct Entry {
    pub txid: Txid,
    pub fee: Sats,
    pub vsize: VSize,
    /// Serialized tx size in bytes (witness + non-witness), from the raw tx.
    pub size: u64,
    /// Pre-computed ancestor fee (self + all ancestors, no double-counting)
    pub ancestor_fee: Sats,
    /// Pre-computed ancestor vsize (self + all ancestors, no double-counting)
    pub ancestor_vsize: VSize,
    /// Parent txid prefixes (most txs have 0-2 parents)
    pub depends: SmallVec<[TxidPrefix; 2]>,
    /// When this tx was first seen in the mempool
    pub first_seen: Timestamp,
}

impl Entry {
    #[inline]
    pub fn fee_rate(&self) -> FeeRate {
        FeeRate::from((self.fee, self.vsize))
    }

    /// Ancestor fee rate (package rate for CPFP).
    #[inline]
    pub fn ancestor_fee_rate(&self) -> FeeRate {
        FeeRate::from((self.ancestor_fee, self.ancestor_vsize))
    }

    /// Effective fee rate for display.
    #[inline]
    pub fn effective_fee_rate(&self) -> FeeRate {
        self.fee_rate().max(self.ancestor_fee_rate())
    }

    #[inline]
    pub fn txid_prefix(&self) -> TxidPrefix {
        TxidPrefix::from(&self.txid)
    }
}
