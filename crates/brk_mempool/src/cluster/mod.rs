//! Cluster primitive shared by the live mempool snapshot rebuilder
//! and the per-request CPFP path. A `Cluster` is a connected component
//! of the mempool dependency graph, locally re-indexed in topological
//! order and SFL-linearized into chunks ordered by descending feerate.
//!
//! Callers supply `ClusterNode`s with parent edges only; `Cluster::new`
//! permutes them into Kahn topological order (so `LocalIdx == position
//! in `nodes` == topological position`), then runs SFL.

mod chunk;
mod chunk_id;
mod cluster_id;
mod cluster_node;
mod cluster_ref;
mod local_idx;
mod sfl;

pub use chunk::Chunk;
pub use chunk_id::ChunkId;
pub use cluster_id::ClusterId;
pub use cluster_node::ClusterNode;
pub use cluster_ref::ClusterRef;
pub use local_idx::LocalIdx;

use smallvec::SmallVec;
use tracing::warn;

/// A connected component of the mempool graph, stored in topological
/// order (parents before children) and SFL-linearized into chunks.
///
/// `I` is the caller's identifier for each node: `brk_mempool::stores::TxIndex`
/// (live pool slot) on the mempool path, `brk_types::TxIndex` (global indexer
/// position) on the confirmed path. The SFL algorithm doesn't touch it; only
/// consumers that need to map a `LocalIdx` back to source-tx state read it.
///
/// Because nodes are stored topologically, every `LocalIdx` is also
/// its topological position: parent edges always point to lower
/// indices, and a forward iteration over `nodes` is a valid topo
/// sweep.
pub struct Cluster<I> {
    pub nodes: Vec<ClusterNode<I>>,
    /// SFL-emitted chunks, ordered by descending feerate.
    pub chunks: Vec<Chunk>,
    /// `node_to_chunk[local]` is the `ChunkId` that contains the node.
    pub node_to_chunk: Vec<ChunkId>,
}

impl<I> Cluster<I> {
    pub fn new(nodes: Vec<ClusterNode<I>>) -> Self {
        let nodes = Self::permute_to_topo_order(nodes);
        let (chunks, node_to_chunk) = if nodes.len() < sfl::BITMASK_LIMIT {
            let chunk_masks = sfl::linearize(&nodes);
            Self::materialize_chunks(&chunk_masks, nodes.len())
        } else {
            // Bitcoin Core 30+ caps clusters at 100, but pre-BIP431 nodes
            // (or relay-policy edge cases) can produce larger connected
            // components. Fall back to a trivial linearization so the
            // mempool loop survives instead of panicking.
            warn!(
                "cluster size {} >= u128 capacity, using trivial linearization",
                nodes.len()
            );
            Self::trivial_chunks(&nodes)
        };
        Self {
            nodes,
            chunks,
            node_to_chunk,
        }
    }

    /// Fallback linearization for oversized clusters: emit each node as
    /// its own chunk (topo-ordered), then run the same stack-merge as
    /// `sfl::canonicalize` to restore the non-increasing fee_rate
    /// invariant the partitioner relies on. Suboptimal partitioning
    /// for that one cluster, but topology is preserved (merges only
    /// join consecutive topo-ordered runs).
    fn trivial_chunks(nodes: &[ClusterNode<I>]) -> (Vec<Chunk>, Vec<ChunkId>) {
        let mut out: Vec<Chunk> = Vec::with_capacity(nodes.len());
        for (i, node) in nodes.iter().enumerate() {
            let mut txs: SmallVec<[LocalIdx; 4]> = SmallVec::new();
            txs.push(LocalIdx::from(i));
            let mut cur = Chunk {
                txs,
                fee: node.fee,
                vsize: node.vsize,
            };
            while let Some(top) = out.last() {
                if cur.fee_rate() <= top.fee_rate() {
                    break;
                }
                let prev = out.pop().unwrap();
                let mut merged_txs = prev.txs;
                merged_txs.extend(cur.txs.iter().copied());
                cur = Chunk {
                    txs: merged_txs,
                    fee: prev.fee + cur.fee,
                    vsize: prev.vsize + cur.vsize,
                };
            }
            out.push(cur);
        }
        let mut node_to_chunk = vec![ChunkId::ZERO; nodes.len()];
        for (cid, chunk) in out.iter().enumerate() {
            let chunk_id = ChunkId::from(cid);
            for &local in &chunk.txs {
                node_to_chunk[local.as_usize()] = chunk_id;
            }
        }
        (out, node_to_chunk)
    }

    /// O(1) chunk lookup for a node.
    #[inline]
    pub fn chunk_of(&self, local: LocalIdx) -> &Chunk {
        &self.chunks[self.node_to_chunk[local.as_usize()].as_usize()]
    }

    /// Reorder `nodes` into Kahn topological order and remap every
    /// parent edge into the new index space. Single pass: build the
    /// child adjacency and in-degrees, then Kahn-pop directly into the
    /// output Vec while remapping each node's parents through the
    /// `new_pos[old] -> new` map populated as we pop. Post-condition:
    /// for every `i`, every parent of `nodes[i]` has a `LocalIdx`
    /// strictly less than `i`.
    fn permute_to_topo_order(mut nodes: Vec<ClusterNode<I>>) -> Vec<ClusterNode<I>> {
        let n = nodes.len();
        let mut children: Vec<SmallVec<[LocalIdx; 2]>> = (0..n).map(|_| SmallVec::new()).collect();
        let mut indegree: Vec<u32> = vec![0; n];
        for (i, node) in nodes.iter().enumerate() {
            indegree[i] = node.parents.len() as u32;
            for &p in &node.parents {
                children[p.as_usize()].push(LocalIdx::from(i));
            }
        }

        // Sources (in-degree 0) seed the queue. We hold them as `LocalIdx`
        // pointing at the *old* slot; `out` drains nodes out as it pops.
        let mut queue: Vec<LocalIdx> = (0..n)
            .filter(|&i| indegree[i] == 0)
            .map(LocalIdx::from)
            .collect();
        let mut new_pos = vec![LocalIdx::ZERO; n];
        let mut out: Vec<ClusterNode<I>> = Vec::with_capacity(n);
        let mut taken: Vec<Option<ClusterNode<I>>> = nodes.drain(..).map(Some).collect();

        let mut head = 0;
        while head < queue.len() {
            let v = queue[head];
            head += 1;
            new_pos[v.as_usize()] = LocalIdx::from(out.len());
            let mut node = taken[v.as_usize()].take().unwrap();
            for p in node.parents.iter_mut() {
                *p = new_pos[p.as_usize()];
            }
            out.push(node);
            for &c in &children[v.as_usize()] {
                indegree[c.as_usize()] -= 1;
                if indegree[c.as_usize()] == 0 {
                    queue.push(c);
                }
            }
        }

        debug_assert_eq!(out.len(), n, "cluster contained a cycle");
        out
    }

    /// Convert SFL's raw bit-masks into final `Chunk`s with topo-ordered
    /// `txs` and a `tx → ChunkId` reverse map. Bit iteration via
    /// `trailing_zeros` visits each chunk's bits in ascending order, and
    /// nodes are stored in topo order (`LocalIdx == position`), so each
    /// pushed `LocalIdx` lands parents-first in `chunk.txs`.
    fn materialize_chunks(chunk_masks: &[sfl::ChunkMask], n: usize) -> (Vec<Chunk>, Vec<ChunkId>) {
        let mut chunks: Vec<Chunk> = Vec::with_capacity(chunk_masks.len());
        let mut node_to_chunk = vec![ChunkId::ZERO; n];
        for (cid, cm) in chunk_masks.iter().enumerate() {
            let chunk_id = ChunkId::from(cid);
            let mut chunk = Chunk {
                txs: SmallVec::new(),
                fee: cm.fee,
                vsize: cm.vsize,
            };
            let mut bits = cm.mask;
            while bits != 0 {
                let i = bits.trailing_zeros() as usize;
                node_to_chunk[i] = chunk_id;
                chunk.txs.push(LocalIdx::from(i));
                bits &= bits - 1;
            }
            chunks.push(chunk);
        }
        (chunks, node_to_chunk)
    }
}
