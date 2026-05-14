use brk_types::{FeeRate, Txid};

use crate::TxRemoval;

#[derive(Debug, Clone, Copy)]
pub struct TxRemoved {
    pub txid: Txid,
    pub reason: TxRemoval,
    /// Package-effective rate at burial. Same value the tx reported
    /// while alive - RBF predecessors keep their package rate, not a
    /// misleading isolated fee/vsize.
    pub chunk_rate: FeeRate,
}
