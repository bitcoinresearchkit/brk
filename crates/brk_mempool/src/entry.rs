use brk_types::{FeeRate, MempoolEntryInfo, Sats, Txid, TxidPrefix, VSize};
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
    /// Pre-computed ancestor fee (self + all ancestors, no double-counting)
    pub ancestor_fee: Sats,
    /// Pre-computed ancestor vsize (self + all ancestors, no double-counting)
    pub ancestor_vsize: VSize,
    /// Parent txid prefixes (most txs have 0-2 parents)
    pub depends: SmallVec<[TxidPrefix; 2]>,
}

impl Entry {
    pub fn from_info(info: &MempoolEntryInfo) -> Self {
        Self {
            txid: info.txid.clone(),
            fee: info.fee,
            vsize: VSize::from(info.vsize),
            ancestor_fee: info.ancestor_fee,
            ancestor_vsize: VSize::from(info.ancestor_size),
            depends: info.depends.iter().map(TxidPrefix::from).collect(),
        }
    }

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
