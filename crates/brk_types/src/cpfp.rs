use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{FeeRate, Sats, Txid, VSize, Weight};

/// Position of a transaction inside a `CpfpCluster.txs` array. Cluster-local,
/// has no meaning outside the enclosing cluster.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
    Default, Deref, Serialize, Deserialize, JsonSchema,
)]
#[serde(transparent)]
pub struct CpfpClusterTxIndex(u32);

impl From<u32> for CpfpClusterTxIndex {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<CpfpClusterTxIndex> for u32 {
    fn from(v: CpfpClusterTxIndex) -> Self {
        v.0
    }
}

/// CPFP (Child Pays For Parent) information for a transaction
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CpfpInfo {
    /// Ancestor transactions in the CPFP chain
    pub ancestors: Vec<CpfpEntry>,
    /// Best (highest fee rate) descendant, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_descendant: Option<CpfpEntry>,
    /// Descendant transactions in the CPFP chain
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub descendants: Vec<CpfpEntry>,
    /// Effective fee rate considering CPFP relationships (sat/vB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_fee_per_vsize: Option<FeeRate>,
    /// Total signature operation count for the seed tx
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigops: Option<u32>,
    /// Transaction fee (sats)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Sats>,
    /// Adjusted virtual size (accounting for sigops)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_vsize: Option<VSize>,
    /// Mempool cluster the seed belongs to: full tx list, SFL-linearized
    /// chunks, and the seed's chunk index. Only set for unconfirmed txs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<CpfpCluster>,
}

/// A transaction in a CPFP relationship
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CpfpEntry {
    /// Transaction ID
    pub txid: Txid,
    /// Transaction weight
    pub weight: Weight,
    /// Transaction fee (sats)
    pub fee: Sats,
}

/// CPFP cluster output for an unconfirmed tx: the connected component
/// the seed belongs to, plus its SFL linearization.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CpfpCluster {
    /// All txs in the cluster, in topological order (parents before children).
    pub txs: Vec<CpfpClusterTx>,
    /// SFL-emitted chunks ordered by descending feerate.
    pub chunks: Vec<CpfpClusterChunk>,
    /// Index into `chunks` of the chunk containing the seed tx.
    pub chunk_index: u32,
}

/// One entry in a `CpfpCluster.txs` array.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CpfpClusterTx {
    pub txid: Txid,
    pub fee: Sats,
    pub weight: Weight,
    /// In-cluster parents of this tx.
    pub parents: Vec<CpfpClusterTxIndex>,
}

/// One SFL chunk inside a `CpfpCluster`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CpfpClusterChunk {
    /// Txs in this chunk.
    pub txs: Vec<CpfpClusterTxIndex>,
    /// Combined feerate of the chunk (sat/vB).
    pub feerate: FeeRate,
}
