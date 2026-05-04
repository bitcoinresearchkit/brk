use brk_types::{CpfpClusterTx, CpfpClusterTxIndex, CpfpEntry, Sats, Txid, VSize, Weight};
use smallvec::SmallVec;

use super::LocalIdx;

/// A node inside a `Cluster<I>`. The `id` carries whatever the caller
/// uses to refer back to the source tx: `brk_mempool::stores::TxIndex`
/// (live pool slot) on the mempool path, `brk_types::TxIndex` (global
/// indexer position) on the confirmed path. `Cluster::new` and the SFL
/// algorithm don't read it.
///
/// All fields are `pub` and callers construct directly with struct
/// literals; `parents` are always supplied at construction (no
/// post-init mutation pattern).
pub struct ClusterNode<I> {
    pub id: I,
    pub txid: Txid,
    pub fee: Sats,
    pub vsize: VSize,
    pub weight: Weight,
    /// Direct parents in the cluster. Caller-supplied.
    pub parents: SmallVec<[LocalIdx; 2]>,
}

impl<I> From<&ClusterNode<I>> for CpfpEntry {
    fn from(node: &ClusterNode<I>) -> Self {
        Self {
            txid: node.txid,
            weight: node.weight,
            fee: node.fee,
        }
    }
}

impl<I> From<&ClusterNode<I>> for CpfpClusterTx {
    fn from(node: &ClusterNode<I>) -> Self {
        Self {
            txid: node.txid,
            weight: node.weight,
            fee: node.fee,
            parents: node
                .parents
                .iter()
                .map(|&p| CpfpClusterTxIndex::from(p.inner()))
                .collect(),
        }
    }
}
