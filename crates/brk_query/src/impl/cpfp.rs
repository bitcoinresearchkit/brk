//! CPFP queries: dispatches between the live mempool path (handled by
//! `brk_mempool`) and the confirmed-tx path built here from indexer
//! and computer vecs.
//!
//! Confirmed clusters are built on demand by walking the same-block
//! parent/child edges in `TxIndex` space (no `Transaction`
//! reconstruction, no `txid → tx_index` lookup), then handing the
//! resulting `brk_mempool::cluster::Cluster` to `Cluster::to_cpfp_info`
//! — the same wire converter the mempool path uses, so both produce
//! identical `CpfpInfo` shapes.

use brk_error::{Error, OptionData, Result};
use brk_mempool::cluster::{Cluster, ClusterNode, LocalIdx};
use brk_types::{CpfpInfo, FeeRate, Height, TxInIndex, TxIndex, Txid, TxidPrefix, VSize, Weight};
use rustc_hash::{FxBuildHasher, FxHashMap};
use smallvec::SmallVec;
use vecdb::{ReadableVec, VecIndex};

use crate::Query;

/// Cap matches Bitcoin Core's default mempool ancestor/descendant
/// chain limits and mempool.space's truncation.
const MAX: usize = 25;

struct WalkResult {
    /// Cluster members in build order (`[seed, ancestors..., descendants...]`),
    /// each paired with its in-cluster parent edges already resolved to
    /// `LocalIdx`. Vec position equals the node's `LocalIdx`.
    nodes: Vec<(TxIndex, SmallVec<[LocalIdx; 2]>)>,
    /// Pre-permutation `LocalIdx` of the seed. Equals `ancestor_count`
    /// because all of seed's in-cluster ancestors topo-sort before it
    /// and only ancestors do, so after `Cluster::new` permutes nodes
    /// into topological order seed lands at this exact position.
    seed_local: LocalIdx,
}

impl Query {
    /// CPFP cluster for `txid`. Returns the mempool cluster when the
    /// txid is unconfirmed; otherwise reconstructs the confirmed
    /// same-block cluster from indexer state. Works even when the
    /// mempool feature is off.
    pub fn cpfp(&self, txid: &Txid) -> Result<CpfpInfo> {
        let prefix = TxidPrefix::from(txid);
        if let Some(info) = self.mempool().and_then(|m| m.cpfp_info(&prefix)) {
            return Ok(info);
        }
        self.confirmed_cpfp(txid)
    }

    /// Effective fee rate for `txid` using the same SFL chunk-rate
    /// semantics across paths:
    ///
    /// - Live mempool: snapshot `cluster_of` lookup → seed's chunk rate.
    ///   If the tx is in the pool but not in the latest snapshot (e.g.
    ///   just added), falls back to the entry's simple `fee/vsize`.
    /// - Confirmed: precomputed `effective_fee_rate.tx_index` (the same
    ///   SFL chunk rate, computed at index time).
    /// - Graveyard-only RBF predecessor: simple `fee/vsize` snapshotted
    ///   at burial.
    ///
    /// Returns `Error::UnknownTxid` for txids not seen in any of those.
    pub fn effective_fee_rate(&self, txid: &Txid) -> Result<FeeRate> {
        let prefix = TxidPrefix::from(txid);

        if let Some(mempool) = self.mempool() {
            let entries = mempool.entries();
            if let Some(seed_idx) = entries.idx_of(&prefix)
                && let Some(rate) = mempool.snapshot().chunk_rate_of(seed_idx)
            {
                return Ok(rate);
            }
            if let Some(entry) = entries.get(&prefix) {
                return Ok(entry.fee_rate());
            }
        }

        if let Ok(idx) = self.resolve_tx_index(txid)
            && let Some(rate) = self
                .computer()
                .transactions
                .fees
                .effective_fee_rate
                .tx_index
                .collect_one(idx)
        {
            return Ok(rate);
        }

        if let Some(mempool) = self.mempool()
            && let Some(tomb) = mempool.graveyard().get(txid)
        {
            return Ok(tomb.entry.fee_rate());
        }

        Err(Error::UnknownTxid)
    }

    /// CPFP cluster for a confirmed tx: the connected component of
    /// same-block parent/child edges, walked on demand. SFL runs on
    /// the result so `effectiveFeePerVsize` matches the live path's
    /// chunk-rate semantics.
    fn confirmed_cpfp(&self, txid: &Txid) -> Result<CpfpInfo> {
        let tx_index = self.resolve_tx_index(txid)?;
        let height = self.confirmed_status_height(tx_index)?;
        let (cluster, seed_local) = self.build_confirmed_cluster(tx_index, height)?;
        let sigops = self
            .indexer()
            .vecs
            .transactions
            .total_sigop_cost
            .collect_one(tx_index)
            .data()?;
        Ok(cluster.to_cpfp_info(seed_local, sigops))
    }

    /// Walk the seed's same-block parent/child edges, materialize each
    /// member's `(txid, weight, fee)` from indexer/computer cursors,
    /// and build a `Cluster<TxIndex>`. The seed's `LocalIdx` comes
    /// straight from the walk (`ancestor_count`), since `Cluster::new`
    /// preserves the "ancestors before seed before descendants" ordering
    /// that defines that index.
    fn build_confirmed_cluster(
        &self,
        seed: TxIndex,
        height: Height,
    ) -> Result<(Cluster<TxIndex>, LocalIdx)> {
        let indexer = self.indexer();
        let computer = self.computer();
        let safe = self.safe_lengths();
        let first_tx_index_vec = &indexer.vecs.transactions.first_tx_index;
        let block_first = first_tx_index_vec.collect_one(height).data()?;
        let next_height = height.incremented();
        let block_end = if next_height < safe.height {
            first_tx_index_vec.collect_one(next_height).data()?
        } else {
            safe.tx_index
        };
        let same_block = |idx: TxIndex| idx >= block_first && idx < block_end;

        let WalkResult { nodes, seed_local } = self.walk_same_block_edges(seed, same_block);

        let mut base_size = indexer.vecs.transactions.base_size.cursor();
        let mut total_size = indexer.vecs.transactions.total_size.cursor();
        let mut fee_cursor = computer.transactions.fees.fee.tx_index.cursor();
        let txid_reader = indexer.vecs.transactions.txid.reader();

        let cluster_nodes: Vec<ClusterNode<TxIndex>> = nodes
            .into_iter()
            .map(|(tx_index, parents)| {
                let i = tx_index.to_usize();
                let weight =
                    Weight::from_sizes(*base_size.get(i).data()?, *total_size.get(i).data()?);
                Ok(ClusterNode {
                    id: tx_index,
                    txid: txid_reader.get(i),
                    fee: fee_cursor.get(i).data()?,
                    vsize: VSize::from(weight),
                    weight,
                    parents,
                })
            })
            .collect::<Result<_>>()?;

        Ok((Cluster::new(cluster_nodes), seed_local))
    }

    /// BFS the seed's same-block ancestors (via `outpoint`) and
    /// descendants (via `spent.txin_index` → `spending_tx`), capped
    /// at `MAX` each side to match Core/mempool.space. Each node is
    /// pushed in build order with its full parent-outpoint list, then
    /// at end of walk those lists are filtered against the membership
    /// map to keep only in-cluster parents (resolved to `LocalIdx`).
    fn walk_same_block_edges(
        &self,
        seed: TxIndex,
        same_block: impl Fn(TxIndex) -> bool,
    ) -> WalkResult {
        let indexer = self.indexer();
        let computer = self.computer();
        let mut first_txin = indexer.vecs.transactions.first_txin_index.cursor();
        let mut first_txout = indexer.vecs.transactions.first_txout_index.cursor();
        let mut outpoint = indexer.vecs.inputs.outpoint.cursor();
        let mut spent = computer.outputs.spent.txin_index.cursor();
        let mut spending_tx = indexer.vecs.inputs.tx_index.cursor();

        let mut walk_inputs = |tx: TxIndex| -> SmallVec<[TxIndex; 2]> {
            let mut out: SmallVec<[TxIndex; 2]> = SmallVec::new();
            let Ok(start) = first_txin.get(tx.to_usize()).data() else {
                return out;
            };
            let Ok(end) = first_txin.get(tx.to_usize() + 1).data() else {
                return out;
            };
            for i in usize::from(start)..usize::from(end) {
                let Ok(op) = outpoint.get(i).data() else {
                    continue;
                };
                if op.is_coinbase() {
                    continue;
                }
                out.push(op.tx_index());
            }
            out
        };

        let mut raw: Vec<(TxIndex, SmallVec<[TxIndex; 2]>)> = Vec::with_capacity(2 * MAX + 1);
        let mut local_of: FxHashMap<TxIndex, LocalIdx> =
            FxHashMap::with_capacity_and_hasher(2 * MAX + 1, FxBuildHasher);
        raw.push((seed, walk_inputs(seed)));
        local_of.insert(seed, LocalIdx::ZERO);

        // Ancestor BFS. Stack holds indices into `raw`; each pop reads
        // that node's already-recorded parents and explores any same-block
        // ones we haven't visited yet. `walk_inputs` runs at push time so
        // parents are ready for the post-walk filter.
        let mut stack: Vec<usize> = vec![0];
        let mut ancestor_count: usize = 0;
        'a: while let Some(idx) = stack.pop() {
            let parents = raw[idx].1.clone();
            for parent in parents {
                if ancestor_count >= MAX {
                    break 'a;
                }
                if local_of.contains_key(&parent) || !same_block(parent) {
                    continue;
                }
                let new_idx = raw.len();
                raw.push((parent, walk_inputs(parent)));
                local_of.insert(parent, LocalIdx::from(new_idx));
                stack.push(new_idx);
                ancestor_count += 1;
            }
        }

        // Descendant BFS. Stack holds tx_indices since we look up each
        // tx's txouts via `first_txout`/`spent`/`spending_tx`. `local_of`
        // already contains the seed and every ancestor, so they're
        // skipped by the membership check.
        let mut stack: Vec<TxIndex> = vec![seed];
        let mut descendant_count = 0;
        'd: while let Some(cur) = stack.pop() {
            let Ok(start) = first_txout.get(cur.to_usize()).data() else {
                continue;
            };
            let Ok(end) = first_txout.get(cur.to_usize() + 1).data() else {
                continue;
            };
            for i in usize::from(start)..usize::from(end) {
                let Ok(txin_idx) = spent.get(i).data() else {
                    continue;
                };
                if txin_idx == TxInIndex::UNSPENT {
                    continue;
                }
                let Ok(child) = spending_tx.get(usize::from(txin_idx)).data() else {
                    continue;
                };
                if local_of.contains_key(&child) || !same_block(child) {
                    continue;
                }
                let new_idx = raw.len();
                raw.push((child, walk_inputs(child)));
                local_of.insert(child, LocalIdx::from(new_idx));
                stack.push(child);
                descendant_count += 1;
                if descendant_count >= MAX {
                    break 'd;
                }
            }
        }

        // Filter each node's full input list against `local_of` to keep
        // only in-cluster parents, resolved to their `LocalIdx`.
        let nodes: Vec<(TxIndex, SmallVec<[LocalIdx; 2]>)> = raw
            .into_iter()
            .map(|(tx_index, full_inputs)| {
                let parents: SmallVec<[LocalIdx; 2]> = full_inputs
                    .iter()
                    .filter_map(|p| local_of.get(p).copied())
                    .collect();
                (tx_index, parents)
            })
            .collect();

        // Seed's pre-permutation index is 0; after `Cluster::new` topo-sorts
        // it lands at `ancestor_count` (all in-cluster ancestors come first,
        // and only ancestors do).
        WalkResult {
            nodes,
            seed_local: LocalIdx::from(ancestor_count),
        }
    }
}
