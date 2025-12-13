use schemars::JsonSchema;
use serde::Serialize;

use crate::{Sats, Transaction, VSize};

/// Mempool statistics
#[derive(Debug, Default, Clone, Serialize, JsonSchema)]
pub struct MempoolInfo {
    /// Number of transactions in the mempool
    pub count: usize,
    /// Total virtual size of all transactions in the mempool (vbytes)
    pub vsize: VSize,
    /// Total fees of all transactions in the mempool (satoshis)
    pub total_fee: Sats,
}

impl MempoolInfo {
    /// Increment stats for a newly added transaction
    #[inline]
    pub fn add(&mut self, tx: &Transaction) {
        self.count += 1;
        self.vsize += tx.vsize();
        self.total_fee += tx.fee;
    }

    /// Decrement stats for a removed transaction
    #[inline]
    pub fn remove(&mut self, tx: &Transaction) {
        self.count -= 1;
        self.vsize -= tx.vsize();
        self.total_fee -= tx.fee;
    }
}
