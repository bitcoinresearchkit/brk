use super::{ClusterId, LocalIdx};

/// Locates a node within the cluster forest: which cluster it lives in,
/// and its `LocalIdx` inside that cluster.
#[derive(Debug, Clone, Copy)]
pub struct ClusterRef {
    pub cluster_id: ClusterId,
    pub local: LocalIdx,
}
