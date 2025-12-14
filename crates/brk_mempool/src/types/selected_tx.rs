use brk_types::FeeRate;

use super::TxIndex;

/// A transaction selected for a projected block.
#[derive(Debug, Clone, Copy)]
pub struct SelectedTx {
    /// Index into mempool entries
    pub tx_index: TxIndex,
    /// Fee rate at selection time (includes CPFP)
    pub effective_fee_rate: FeeRate,
}
