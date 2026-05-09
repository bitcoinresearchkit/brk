//! CPFP queries: dispatches between the live mempool path (handled by
//! `brk_mempool`) and the confirmed-tx path built here from indexer
//! and computer vecs.
//!
//! Confirmed clusters are materialized on demand by walking same-block
//! parent/child edges in `TxIndex` space (no `Transaction`
//! reconstruction, no `txid -> tx_index` lookup), then assembling the
//! wire shape directly. The seed's effective fee rate and the per-chunk
//! grouping both read precomputed `effective_fee_rate.tx_index`, which
//! carries the same chunk-rate semantics the live mempool produces.

use brk_error::{Error, OptionData, Result};
use brk_types::{
    CPFP_CHAIN_LIMIT, ChunkInput, CpfpCluster, CpfpClusterTx, CpfpClusterTxIndex, CpfpEntry,
    CpfpInfo, FeeRate, Height, Sats, TxInIndex, TxIndex, Txid, TxidPrefix, VSize, Weight,
    find_seed_chunk, linearize,
};
use rustc_hash::{FxBuildHasher, FxHashMap};
use smallvec::SmallVec;
use vecdb::{ReadableVec, VecIndex};

use crate::Query;

struct WalkResult {
    /// Cluster members in `[ancestors..., seed, descendants...]` order,
    /// each paired with its in-cluster parent edges resolved to the
    /// member's local index. The seed's local index is `ancestors.len()`.
    members: Vec<(TxIndex, SmallVec<[CpfpClusterTxIndex; 2]>)>,
    seed_local: CpfpClusterTxIndex,
}

struct Member {
    txid: Txid,
    fee: Sats,
    weight: Weight,
    vsize: VSize,
    rate: FeeRate,
    parents: SmallVec<[CpfpClusterTxIndex; 2]>,
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

    /// Effective fee rate for `txid` using the same chunk-rate semantics
    /// across paths:
    ///
    /// - Live mempool: snapshot's per-tx linearized `chunk_rate`. If
    ///   the tx is in the pool but not in the latest snapshot (e.g.
    ///   just added), falls back to the entry's simple `fee/vsize`.
    /// - Confirmed: precomputed `effective_fee_rate.tx_index`.
    /// - Graveyard-only RBF predecessor: linearized chunk rate
    ///   captured at burial.
    ///
    /// Returns `Error::UnknownTxid` for txids not seen in any of those.
    pub fn effective_fee_rate(&self, txid: &Txid) -> Result<FeeRate> {
        let prefix = TxidPrefix::from(txid);

        if let Some(mempool) = self.mempool()
            && let Some(rate) = mempool.live_effective_fee_rate(&prefix)
        {
            return Ok(rate);
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
            && let Some(rate) = mempool.graveyard_fee_rate(txid)
        {
            return Ok(rate);
        }

        Err(Error::UnknownTxid)
    }

    /// CPFP cluster for a confirmed tx: the connected component of
    /// same-block parent/child edges, walked on demand. Per-tx
    /// `effective_fee_rate.tx_index` provides each member's chunk rate.
    fn confirmed_cpfp(&self, txid: &Txid) -> Result<CpfpInfo> {
        let tx_index = self.resolve_tx_index(txid)?;
        let height = self.confirmed_status_height(tx_index)?;
        let WalkResult {
            members,
            seed_local,
        } = self.walk_same_block_cluster(tx_index, height)?;

        let resolved = self.resolve_members(&members)?;
        let sigops = self
            .indexer()
            .vecs
            .transactions
            .total_sigop_cost
            .collect_one(tx_index)
            .data()?;

        Ok(build_cpfp_info(&resolved, seed_local, sigops))
    }

    fn resolve_members(
        &self,
        members: &[(TxIndex, SmallVec<[CpfpClusterTxIndex; 2]>)],
    ) -> Result<Vec<Member>> {
        let indexer = self.indexer();
        let computer = self.computer();
        let mut base_size = indexer.vecs.transactions.base_size.cursor();
        let mut total_size = indexer.vecs.transactions.total_size.cursor();
        let mut fee_cursor = computer.transactions.fees.fee.tx_index.cursor();
        let mut rate_cursor = computer
            .transactions
            .fees
            .effective_fee_rate
            .tx_index
            .cursor();
        let txid_reader = indexer.vecs.transactions.txid.reader();

        members
            .iter()
            .map(|(tx_index, parents)| {
                let i = tx_index.to_usize();
                let weight =
                    Weight::from_sizes(*base_size.get(i).data()?, *total_size.get(i).data()?);
                let vsize = VSize::from(weight);
                Ok(Member {
                    txid: txid_reader.get(i),
                    fee: fee_cursor.get(i).data()?,
                    weight,
                    vsize,
                    rate: rate_cursor.get(i).data()?,
                    parents: parents.clone(),
                })
            })
            .collect()
    }

    /// BFS the seed's same-block ancestors (via `outpoint`) and
    /// descendants (via `spent.txin_index` -> `spending_tx`), capped
    /// at `CPFP_CHAIN_LIMIT` each side to match Core/mempool.space. Returns members
    /// laid out as `[ancestors..., seed, descendants...]` so the seed's
    /// local index is `ancestors.len()`.
    fn walk_same_block_cluster(&self, seed: TxIndex, height: Height) -> Result<WalkResult> {
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

        let mut visited: FxHashMap<TxIndex, ()> =
            FxHashMap::with_capacity_and_hasher(2 * CPFP_CHAIN_LIMIT + 1, FxBuildHasher);
        visited.insert(seed, ());

        // Ancestor BFS: each push records (tx_index, raw parent tx_indices)
        // so we can filter against final cluster membership at the end.
        let seed_inputs = walk_inputs(seed);
        let mut ancestors: Vec<(TxIndex, SmallVec<[TxIndex; 2]>)> = Vec::new();
        let mut stack: Vec<SmallVec<[TxIndex; 2]>> = vec![seed_inputs.clone()];
        'a: while let Some(parents) = stack.pop() {
            for parent in parents {
                if ancestors.len() >= CPFP_CHAIN_LIMIT {
                    break 'a;
                }
                if visited.insert(parent, ()).is_some() || !same_block(parent) {
                    continue;
                }
                let parent_inputs = walk_inputs(parent);
                ancestors.push((parent, parent_inputs.clone()));
                stack.push(parent_inputs);
            }
        }

        // Descendant BFS via spent outputs.
        let mut descendants: Vec<(TxIndex, SmallVec<[TxIndex; 2]>)> = Vec::new();
        let mut stack: Vec<TxIndex> = vec![seed];
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
                if visited.insert(child, ()).is_some() || !same_block(child) {
                    continue;
                }
                descendants.push((child, walk_inputs(child)));
                stack.push(child);
                if descendants.len() >= CPFP_CHAIN_LIMIT {
                    break 'd;
                }
            }
        }

        // Lay members out as [ancestors_reverse..., seed, descendants...]
        // so parents come before children when a single ancestor chain
        // walks back from seed. Reversing the BFS order is good enough
        // for wire output; chunk grouping doesn't depend on it.
        let ancestor_count = ancestors.len();
        let total = ancestor_count + 1 + descendants.len();
        let mut local_of: FxHashMap<TxIndex, CpfpClusterTxIndex> =
            FxHashMap::with_capacity_and_hasher(total, FxBuildHasher);
        let mut members: Vec<(TxIndex, SmallVec<[TxIndex; 2]>)> = Vec::with_capacity(total);

        for (tx, raw_parents) in ancestors.into_iter().rev() {
            local_of.insert(tx, CpfpClusterTxIndex::from(members.len() as u32));
            members.push((tx, raw_parents));
        }
        let seed_local = CpfpClusterTxIndex::from(members.len() as u32);
        local_of.insert(seed, seed_local);
        members.push((seed, seed_inputs));
        for (tx, raw_parents) in descendants {
            local_of.insert(tx, CpfpClusterTxIndex::from(members.len() as u32));
            members.push((tx, raw_parents));
        }

        let resolved: Vec<(TxIndex, SmallVec<[CpfpClusterTxIndex; 2]>)> = members
            .into_iter()
            .map(|(tx, raw_parents)| {
                let parents: SmallVec<[CpfpClusterTxIndex; 2]> = raw_parents
                    .iter()
                    .filter_map(|p| local_of.get(p).copied())
                    .collect();
                (tx, parents)
            })
            .collect();

        Ok(WalkResult {
            members: resolved,
            seed_local,
        })
    }
}

fn build_cpfp_info(
    members: &[Member],
    seed_local: CpfpClusterTxIndex,
    sigops: brk_types::SigOps,
) -> CpfpInfo {
    let seed_pos = u32::from(seed_local) as usize;
    let seed = &members[seed_pos];

    let ancestors: Vec<CpfpEntry> = members[..seed_pos]
        .iter()
        .map(|m| CpfpEntry {
            txid: m.txid,
            weight: m.weight,
            fee: m.fee,
        })
        .collect();
    let descendants: Vec<CpfpEntry> = members[seed_pos + 1..]
        .iter()
        .map(|m| CpfpEntry {
            txid: m.txid,
            weight: m.weight,
            fee: m.fee,
        })
        .collect();
    let best_descendant = descendants
        .iter()
        .max_by_key(|e| FeeRate::from((e.fee, e.weight)))
        .cloned();

    let (cluster, effective_fee_per_vsize) = if members.len() <= 1 {
        (None, seed.rate)
    } else {
        let inputs: Vec<ChunkInput<'_>> = members
            .iter()
            .map(|m| ChunkInput {
                fee: m.fee,
                vsize: m.vsize,
                parents: m.parents.as_slice(),
            })
            .collect();
        let chunks = linearize(&inputs);
        let (chunk_index, seed_rate) = find_seed_chunk(&chunks, seed_local, seed.rate);
        let cluster_txs: Vec<CpfpClusterTx> = members
            .iter()
            .map(|m| CpfpClusterTx {
                txid: m.txid,
                weight: m.weight,
                fee: m.fee,
                parents: m.parents.iter().copied().collect(),
            })
            .collect();
        (
            Some(CpfpCluster {
                txs: cluster_txs,
                chunks,
                chunk_index,
            }),
            seed_rate,
        )
    };

    CpfpInfo {
        ancestors,
        best_descendant,
        descendants,
        effective_fee_per_vsize,
        sigops,
        fee: seed.fee,
        vsize: seed.vsize,
        adjusted_vsize: sigops.adjust_vsize(seed.vsize),
        cluster,
    }
}
