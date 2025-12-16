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
    /// Increment stats for a newly added transaction.
    ///
    /// Fee must come from `MempoolEntryInfo` (Bitcoin Core) rather than `tx.fee`
    /// because `tx.fee` may be 0 for chained mempool transactions where prevouts
    /// cannot be looked up via `gettxout`.
    #[inline]
    pub fn add(&mut self, tx: &Transaction, fee: Sats) {
        self.count += 1;
        self.vsize += tx.vsize();
        self.total_fee += fee;
    }

    /// Decrement stats for a removed transaction.
    ///
    /// Fee must match the fee used when the transaction was added.
    #[inline]
    pub fn remove(&mut self, tx: &Transaction, fee: Sats) {
        self.count -= 1;
        self.vsize -= tx.vsize();
        self.total_fee -= fee;
    }
}
