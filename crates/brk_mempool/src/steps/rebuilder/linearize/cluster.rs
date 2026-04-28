use super::{ClusterNode, LocalIdx};

/// A connected component of the mempool graph, re-indexed locally.
pub(crate) struct Cluster {
    pub(crate) nodes: Vec<ClusterNode>,
    /// Used during chunk emission to print txs parents-first.
    pub(crate) topo_rank: Vec<u32>,
}

impl Cluster {
    pub(crate) fn new(nodes: Vec<ClusterNode>) -> Self {
        let topo_rank = Self::kahn_topo_rank(&nodes);
        Self { nodes, topo_rank }
    }

    fn kahn_topo_rank(nodes: &[ClusterNode]) -> Vec<u32> {
        let n = nodes.len();
        let mut indegree: Vec<u32> = nodes.iter().map(|n| n.parents.len() as u32).collect();
        let mut ready: Vec<LocalIdx> = (0..n as LocalIdx)
            .filter(|&i| indegree[i as usize] == 0)
            .collect();

        let mut rank: Vec<u32> = vec![0; n];
        let mut position: u32 = 0;
        let mut head = 0;

        while head < ready.len() {
            let v = ready[head];
            head += 1;
            rank[v as usize] = position;
            position += 1;
            for &c in &nodes[v as usize].children {
                indegree[c as usize] -= 1;
                if indegree[c as usize] == 0 {
                    ready.push(c);
                }
            }
        }

        debug_assert_eq!(position as usize, n, "cluster contained a cycle");
        rank
    }
}
