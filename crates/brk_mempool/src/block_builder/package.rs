use brk_types::FeeRate;

use crate::types::TxIndex;

/// A CPFP package: transactions the selector decided to mine together
/// because a child pays for its parent.
///
/// Carries two rates:
/// - `fee_rate` is the package's own rate (sum of fees / sum of vsizes),
///   i.e. what a miner collects per vsize when the package is mined.
///   Used for per-tx fee stats and user-facing recommendations.
/// - `placement_rate` is the key the partitioner sorts by. It's the own
///   rate clamped below by the `placement_rate` of any ancestor packages,
///   so that sorting packages by this rate descending keeps dependent
///   packages in topological order even when a child's own rate exceeds
///   its parent's (possible in branching CPFP).
pub struct Package {
    /// Transactions in topological order (parents before children).
    pub txs: Vec<TxIndex>,
    pub vsize: u64,
    pub fee_rate: FeeRate,
    pub placement_rate: FeeRate,
}

impl Package {
    pub fn new(fee_rate: FeeRate) -> Self {
        Self {
            txs: Vec::new(),
            vsize: 0,
            fee_rate,
            placement_rate: fee_rate,
        }
    }

    pub fn add_tx(&mut self, tx_index: TxIndex, vsize: u64) {
        self.txs.push(tx_index);
        self.vsize += vsize;
    }
}
