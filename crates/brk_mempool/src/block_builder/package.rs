use brk_types::FeeRate;

use crate::types::TxIndex;

/// A CPFP package: transactions the linearizer decided to mine together
/// because a child pays for its parent.
///
/// `fee_rate` is the package's own rate (sum of fees / sum of vsizes),
/// i.e. what a miner collects per vsize when the package is mined.
/// Packages are produced by SFL in descending-`fee_rate` order within a
/// cluster and are atomic (all-or-nothing) at mining time.
///
/// `cluster_id` + `chunk_order` let the partitioner enforce intra-cluster
/// ordering when its look-ahead would otherwise pull a child chunk into
/// an earlier block than its parent chunk.
pub struct Package {
    /// Transactions in topological order (parents before children).
    pub txs: Vec<TxIndex>,
    pub vsize: u64,
    pub fee_rate: FeeRate,
    pub cluster_id: u32,
    pub chunk_order: u32,
}

impl Package {
    pub fn new(fee_rate: FeeRate, cluster_id: u32, chunk_order: u32) -> Self {
        Self {
            txs: Vec::new(),
            vsize: 0,
            fee_rate,
            cluster_id,
            chunk_order,
        }
    }

    pub fn add_tx(&mut self, tx_index: TxIndex, vsize: u64) {
        self.txs.push(tx_index);
        self.vsize += vsize;
    }
}
