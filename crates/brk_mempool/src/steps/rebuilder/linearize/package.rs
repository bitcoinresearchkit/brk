use brk_types::{FeeRate, VSize};
use smallvec::SmallVec;

use super::{LocalIdx, chunk::Chunk, cluster::Cluster};
use crate::stores::TxIndex;

/// A CPFP package: transactions mined together because a child pays
/// for its parent. Atomic (all-or-nothing) at mining time.
///
/// `fee_rate` is the package's combined rate (sum of fees / sum of
/// vsizes). SFL emits packages in descending-`fee_rate` order within
/// a cluster.
///
/// `cluster_id` + `chunk_order` let the partitioner enforce
/// intra-cluster ordering when its look-ahead would otherwise pull a
/// child chunk into an earlier block than its parent chunk.
pub struct Package {
    /// Transactions in topological order (parents before children).
    pub txs: Vec<TxIndex>,
    pub vsize: VSize,
    pub fee_rate: FeeRate,
    pub cluster_id: u32,
    pub chunk_order: u32,
}

impl Package {
    pub(super) fn singleton(cluster: &Cluster, cluster_id: u32) -> Self {
        let node = &cluster.nodes[0];
        let mut package = Self::empty(FeeRate::from((node.fee, node.vsize)), cluster_id, 0);
        package.add_tx(node.tx_index, node.vsize);
        package
    }

    /// Txs inside the package are ordered parents-first by `topo_rank`.
    pub(super) fn from_chunk(
        cluster: &Cluster,
        chunk: Chunk,
        cluster_id: u32,
        chunk_order: u32,
    ) -> Self {
        let mut package = Self::empty(chunk.fee_rate(), cluster_id, chunk_order);

        let mut ordered: SmallVec<[LocalIdx; 8]> = chunk.nodes.into_iter().collect();
        ordered.sort_by_key(|&local| cluster.topo_rank[local as usize]);

        for local in ordered {
            let node = &cluster.nodes[local as usize];
            package.add_tx(node.tx_index, node.vsize);
        }
        package
    }

    fn empty(fee_rate: FeeRate, cluster_id: u32, chunk_order: u32) -> Self {
        Self {
            txs: Vec::new(),
            vsize: VSize::default(),
            fee_rate,
            cluster_id,
            chunk_order,
        }
    }

    fn add_tx(&mut self, tx_index: TxIndex, vsize: VSize) {
        self.txs.push(tx_index);
        self.vsize += vsize;
    }
}
