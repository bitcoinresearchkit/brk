use brk_types::FeeRate;

use crate::types::{SelectedTx, TxIndex};

/// A CPFP package - transactions that must be included together.
///
/// When a child pays for its parent (CPFP), both must be in the same block.
/// The package fee rate is the combined rate of all transactions.
pub struct Package {
    /// Transactions in topological order (parents before children)
    pub txs: Vec<SelectedTx>,

    /// Combined vsize of all transactions
    pub vsize: u64,

    /// Package fee rate
    pub fee_rate: FeeRate,
}

impl Package {
    pub fn new(fee_rate: FeeRate) -> Self {
        Self {
            txs: Vec::new(),
            vsize: 0,
            fee_rate,
        }
    }

    pub fn add_tx(&mut self, tx_index: TxIndex, vsize: u64) {
        self.txs.push(SelectedTx {
            tx_index,
            effective_fee_rate: self.fee_rate,
        });
        self.vsize += vsize;
    }
}
