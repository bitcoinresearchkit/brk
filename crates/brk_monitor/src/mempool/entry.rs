use brk_types::{FeeRate, Outpoint, Sats, Transaction, Txid, VSize};
use rustc_hash::FxHashSet;

/// A mempool transaction with its dependency metadata
#[derive(Debug, Clone)]
pub struct MempoolEntry {
    pub txid: Txid,
    pub fee: Sats,
    pub vsize: VSize,

    /// Outpoints this tx spends (inputs)
    pub spends: Vec<Outpoint>,

    /// Txids of unconfirmed ancestors (parents, grandparents, etc.)
    pub ancestors: FxHashSet<Txid>,

    /// Cumulative fee of this tx + all ancestors
    pub ancestor_fee: Sats,

    /// Cumulative vsize of this tx + all ancestors
    pub ancestor_vsize: VSize,
}

impl MempoolEntry {
    pub fn new(tx: &Transaction) -> Self {
        let txid = tx.txid.clone();
        let fee = tx.fee;
        let vsize = tx.vsize();

        let spends = tx
            .input
            .iter()
            .map(|txin| Outpoint::new(txin.txid.clone(), txin.vout))
            .collect();

        Self {
            txid,
            fee,
            vsize,
            spends,
            ancestors: FxHashSet::default(),
            ancestor_fee: fee,
            ancestor_vsize: vsize,
        }
    }

    /// Individual fee rate (without ancestors)
    #[inline]
    pub fn fee_rate(&self) -> FeeRate {
        FeeRate::from((self.fee, self.vsize))
    }

    /// Ancestor fee rate (fee + ancestors_fee) / (vsize + ancestors_vsize)
    /// This is the effective mining priority
    #[inline]
    pub fn ancestor_fee_rate(&self) -> FeeRate {
        FeeRate::from((self.ancestor_fee, self.ancestor_vsize))
    }
}
