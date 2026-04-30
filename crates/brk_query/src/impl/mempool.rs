use brk_error::{Error, OptionData, Result};
use brk_mempool::{EntryPool, TxEntry, TxGraveyard, TxRemoval, TxStore, TxTombstone};
use brk_types::{
    CheckedSub, CpfpEntry, CpfpInfo, FeeRate, MempoolBlock, MempoolInfo, MempoolRecentTx,
    OutputType, RbfResponse, RbfTx, RecommendedFees, ReplacementNode, Sats, Timestamp, Transaction,
    TxIndex, TxInIndex, TxOut, TxOutIndex, Txid, TxidPrefix, TypeIndex, VSize, Weight,
};
use rustc_hash::FxHashSet;
use vecdb::{AnyVec, ReadableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn mempool_info(&self) -> Result<MempoolInfo> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        Ok(mempool.info())
    }

    pub fn mempool_txids(&self) -> Result<Vec<Txid>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let txs = mempool.txs();
        Ok(txs.keys().cloned().collect())
    }

    pub fn recommended_fees(&self) -> Result<RecommendedFees> {
        self.mempool()
            .map(|mempool| mempool.fees())
            .ok_or(Error::MempoolNotAvailable)
    }

    pub fn mempool_blocks(&self) -> Result<Vec<MempoolBlock>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;

        let block_stats = mempool.block_stats();

        let blocks = block_stats
            .into_iter()
            .map(|stats| {
                MempoolBlock::new(
                    stats.tx_count,
                    stats.total_size,
                    stats.total_vsize,
                    stats.total_fee,
                    stats.fee_range,
                )
            })
            .collect();

        Ok(blocks)
    }

    /// Fill any `prevout == None` inputs on live mempool txs from the
    /// indexer, mutating them in place. Cheap when the unresolved set
    /// is empty (the steady-state with `-txindex` on); otherwise resolves
    /// each missing prevout via the same lookup chain used for confirmed
    /// txs: `txid → tx_index → first_txout_index + vout → output_type
    /// / type_index / value → script_pubkey`.
    ///
    /// Driver calls this once per cycle, right after `mempool.update()`.
    /// Returns true if at least one prevout was filled.
    pub fn fill_mempool_prevouts(&self) -> bool {
        let Some(mempool) = self.mempool() else {
            return false;
        };

        let indexer = self.indexer();
        let stores = &indexer.stores;
        let first_txout_index_reader = indexer.vecs.transactions.first_txout_index.reader();
        let output_type_reader = indexer.vecs.outputs.output_type.reader();
        let type_index_reader = indexer.vecs.outputs.type_index.reader();
        let value_reader = indexer.vecs.outputs.value.reader();
        let addr_readers = indexer.vecs.addrs.addr_readers();

        mempool.fill_prevouts(|prev_txid, vout| {
            let prev_tx_index = stores
                .txid_prefix_to_tx_index
                .get(&TxidPrefix::from(prev_txid))
                .ok()??
                .into_owned();
            let first_txout: TxOutIndex = first_txout_index_reader.get(prev_tx_index.to_usize());
            let txout_index = usize::from(first_txout + vout);
            let output_type: OutputType = output_type_reader.get(txout_index);
            let type_index: TypeIndex = type_index_reader.get(txout_index);
            let value: Sats = value_reader.get(txout_index);
            let script_pubkey = addr_readers.script_pubkey(output_type, type_index);
            Some(TxOut::from((script_pubkey, value)))
        })
    }

    pub fn mempool_recent(&self) -> Result<Vec<MempoolRecentTx>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        Ok(mempool.txs().recent().to_vec())
    }

    pub fn cpfp(&self, txid: &Txid) -> Result<CpfpInfo> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let prefix = TxidPrefix::from(txid);
        Ok(mempool
            .cpfp_info(&prefix)
            .unwrap_or_else(|| self.confirmed_cpfp(txid)))
    }

    /// CPFP cluster for a confirmed tx: the connected component of
    /// same-block parent/child edges, reconstructed by BFS on demand.
    /// Walks entirely in `TxIndex` space using direct vec reads (height,
    /// weight, fee) - skips full `Transaction` reconstruction and avoids
    /// `txid -> tx_index` lookups by reading `OutPoint`'s packed
    /// `tx_index` directly. Capped at 25 each side to match Bitcoin
    /// Core's default mempool chain limits and mempool.space's own
    /// truncation. `effectiveFeePerVsize` is the simple package rate;
    /// mempool's `calculateGoodBlockCpfp` chunk-rate algorithm is not
    /// ported.
    fn confirmed_cpfp(&self, txid: &Txid) -> CpfpInfo {
        const MAX: usize = 25;
        let Ok(seed_idx) = self.resolve_tx_index(txid) else {
            return CpfpInfo::default();
        };
        let Ok(seed_height) = self.confirmed_status_height(seed_idx) else {
            return CpfpInfo::default();
        };

        let indexer = self.indexer();
        let computer = self.computer();
        // Block's tx_index range. Reduces the per-neighbor height check to a
        // pair of integer compares (vs `tx_heights.get_shared` which acquires
        // a read lock and walks a `RangeMap`).
        let Ok(block_first) = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_one(seed_height)
            .data()
        else {
            return CpfpInfo::default();
        };
        let block_end = indexer
            .vecs
            .transactions
            .first_tx_index
            .collect_one(seed_height.incremented())
            .unwrap_or_else(|| TxIndex::from(indexer.vecs.transactions.txid.len()));
        let same_block = |idx: TxIndex| idx >= block_first && idx < block_end;

        let mut first_txin = indexer.vecs.transactions.first_txin_index.cursor();
        let mut first_txout = indexer.vecs.transactions.first_txout_index.cursor();
        let mut outpoint = indexer.vecs.inputs.outpoint.cursor();
        let mut spent = computer.outputs.spent.txin_index.cursor();
        let mut spending_tx = indexer.vecs.inputs.tx_index.cursor();

        let mut visited: FxHashSet<TxIndex> = FxHashSet::with_capacity_and_hasher(
            2 * MAX + 1,
            Default::default(),
        );
        visited.insert(seed_idx);

        let mut ancestor_idxs: Vec<TxIndex> = Vec::with_capacity(MAX);
        let mut queue: Vec<TxIndex> = vec![seed_idx];
        'a: while let Some(cur) = queue.pop() {
            let Ok(start) = first_txin.get(cur.to_usize()).data() else { continue };
            let Ok(end) = first_txin.get(cur.to_usize() + 1).data() else { continue };
            for i in usize::from(start)..usize::from(end) {
                let Ok(op) = outpoint.get(i).data() else { continue };
                if op.is_coinbase() {
                    continue;
                }
                let parent = op.tx_index();
                if !visited.insert(parent) || !same_block(parent) {
                    continue;
                }
                ancestor_idxs.push(parent);
                queue.push(parent);
                if ancestor_idxs.len() >= MAX {
                    break 'a;
                }
            }
        }

        let mut descendant_idxs: Vec<TxIndex> = Vec::with_capacity(MAX);
        let mut queue: Vec<TxIndex> = vec![seed_idx];
        'd: while let Some(cur) = queue.pop() {
            let Ok(start) = first_txout.get(cur.to_usize()).data() else { continue };
            let Ok(end) = first_txout.get(cur.to_usize() + 1).data() else { continue };
            for i in usize::from(start)..usize::from(end) {
                let Ok(txin_idx) = spent.get(i).data() else { continue };
                if txin_idx == TxInIndex::UNSPENT {
                    continue;
                }
                let Ok(child) = spending_tx.get(usize::from(txin_idx)).data() else { continue };
                if !visited.insert(child) || !same_block(child) {
                    continue;
                }
                descendant_idxs.push(child);
                queue.push(child);
                if descendant_idxs.len() >= MAX {
                    break 'd;
                }
            }
        }

        // Phase 2: bulk-fetch (weight, fee) for seed + cluster, cursors opened
        // once and reads issued in tx_index order for sequential page locality.
        let mut all = Vec::with_capacity(1 + ancestor_idxs.len() + descendant_idxs.len());
        all.push(seed_idx);
        all.extend(&ancestor_idxs);
        all.extend(&descendant_idxs);
        let Ok(weights_fees) = self.txs_weight_fee(&all) else {
            return CpfpInfo::default();
        };

        let txid_reader = indexer.vecs.transactions.txid.reader();
        let entry_at = |i: usize, idx: TxIndex| {
            let (weight, fee) = weights_fees[i];
            CpfpEntry {
                txid: txid_reader.get(idx.to_usize()),
                weight,
                fee,
            }
        };
        let (seed_weight, seed_fee) = weights_fees[0];
        let seed_vsize = VSize::from(seed_weight);
        let ancestors: Vec<CpfpEntry> = ancestor_idxs
            .iter()
            .enumerate()
            .map(|(k, &idx)| entry_at(1 + k, idx))
            .collect();
        let descendants: Vec<CpfpEntry> = descendant_idxs
            .iter()
            .enumerate()
            .map(|(k, &idx)| entry_at(1 + ancestor_idxs.len() + k, idx))
            .collect();

        let (sum_fee, sum_vsize) = ancestors
            .iter()
            .chain(descendants.iter())
            .fold((u64::from(seed_fee), u64::from(seed_vsize)), |(f, v), e| {
                (f + u64::from(e.fee), v + u64::from(VSize::from(e.weight)))
            });
        let package_rate = FeeRate::from((Sats::from(sum_fee), VSize::from(sum_vsize)));
        let effective = FeeRate::from((seed_fee, seed_vsize)).max(package_rate);

        let best_descendant = descendants
            .iter()
            .max_by_key(|e| FeeRate::from((e.fee, e.weight)))
            .cloned();

        CpfpInfo {
            ancestors,
            best_descendant,
            descendants,
            effective_fee_per_vsize: Some(effective),
            fee: Some(seed_fee),
            adjusted_vsize: Some(seed_vsize),
        }
    }

    /// Bulk read `(weight, fee)` for many tx_indexes. Cursors opened once;
    /// reads issued in ascending `tx_index` order for sequential I/O,
    /// results returned in the caller's order.
    fn txs_weight_fee(&self, idxs: &[TxIndex]) -> Result<Vec<(Weight, Sats)>> {
        if idxs.is_empty() {
            return Ok(vec![]);
        }
        let indexer = self.indexer();
        let computer = self.computer();
        let mut base_size = indexer.vecs.transactions.base_size.cursor();
        let mut total_size = indexer.vecs.transactions.total_size.cursor();
        let mut fee_cursor = computer.transactions.fees.fee.tx_index.cursor();

        let mut order: Vec<usize> = (0..idxs.len()).collect();
        order.sort_unstable_by_key(|&i| idxs[i]);

        let mut out = vec![(Weight::default(), Sats::ZERO); idxs.len()];
        for &pos in &order {
            let i = idxs[pos].to_usize();
            let bs = base_size.get(i).data()?;
            let ts = total_size.get(i).data()?;
            let f = fee_cursor.get(i).data()?;
            out[pos] = (Weight::from_sizes(*bs, *ts), f);
        }
        Ok(out)
    }

    /// RBF history for a tx, matching mempool.space's
    /// `GET /api/v1/tx/:txid/rbf`. Walks forward through the graveyard
    /// to find the latest known replacer (tree root), then recursively
    /// walks `predecessors_of` backward to build the tree. `replaces`
    /// is the requested tx's own direct predecessors.
    pub fn tx_rbf(&self, txid: &Txid) -> Result<RbfResponse> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let txs = mempool.txs();
        let entries = mempool.entries();
        let graveyard = mempool.graveyard();

        let mut root_txid = txid.clone();
        while let Some(TxRemoval::Replaced { by }) =
            graveyard.get(&root_txid).map(TxTombstone::reason)
        {
            root_txid = by.clone();
        }

        let replaces_vec: Vec<Txid> = graveyard
            .predecessors_of(txid)
            .map(|(p, _)| p.clone())
            .collect();
        let replaces = (!replaces_vec.is_empty()).then_some(replaces_vec);

        let replacements =
            Self::build_rbf_node(&root_txid, None, &txs, &entries, &graveyard).map(|mut node| {
                node.tx.full_rbf = Some(node.full_rbf);
                node.interval = None;
                node
            });

        Ok(RbfResponse {
            replacements,
            replaces,
        })
    }

    /// Resolve a txid to the data we need for an `RbfTx`. The live
    /// pool takes priority; the graveyard is the fallback. Returns
    /// `None` if the tx has no known data in either.
    fn resolve_rbf_node_data<'a>(
        txid: &Txid,
        txs: &'a TxStore,
        entries: &'a EntryPool,
        graveyard: &'a TxGraveyard,
    ) -> Option<(&'a Transaction, &'a TxEntry)> {
        if let (Some(tx), Some(entry)) = (txs.get(txid), entries.get(&TxidPrefix::from(txid))) {
            return Some((tx, entry));
        }
        graveyard.get(txid).map(|tomb| (&tomb.tx, &tomb.entry))
    }

    /// Recursively build an RBF tree node rooted at `txid`.
    /// Predecessors are always in the graveyard (that's where
    /// `Removal::Replaced` lives), so the recursion only needs the
    /// graveyard; the live pool is consulted for the root.
    fn build_rbf_node(
        txid: &Txid,
        successor_time: Option<Timestamp>,
        txs: &TxStore,
        entries: &EntryPool,
        graveyard: &TxGraveyard,
    ) -> Option<ReplacementNode> {
        let (tx, entry) = Self::resolve_rbf_node_data(txid, txs, entries, graveyard)?;

        let replaces: Vec<ReplacementNode> = graveyard
            .predecessors_of(txid)
            .filter_map(|(pred_txid, _)| {
                Self::build_rbf_node(pred_txid, Some(entry.first_seen), txs, entries, graveyard)
            })
            .collect();

        let full_rbf = replaces.iter().any(|c| !c.tx.rbf || c.full_rbf);

        let interval = successor_time
            .and_then(|st| st.checked_sub(entry.first_seen))
            .map(|d| usize::from(d) as u32);

        let value = Sats::from(tx.output.iter().map(|o| u64::from(o.value)).sum::<u64>());

        Some(ReplacementNode {
            tx: RbfTx {
                txid: txid.clone(),
                fee: entry.fee,
                vsize: entry.vsize,
                value,
                rate: entry.fee_rate(),
                time: entry.first_seen,
                rbf: entry.rbf,
                full_rbf: None,
            },
            time: entry.first_seen,
            full_rbf,
            interval,
            replaces,
        })
    }

    /// Recent RBF replacements across the whole mempool, matching
    /// mempool.space's `GET /api/v1/replacements` and
    /// `GET /api/v1/fullrbf/replacements`. Each entry is a complete
    /// replacement tree rooted at the latest replacer; same shape as
    /// `tx_rbf().replacements`. Sorted most-recent-first by root
    /// `time`. When `full_rbf_only` is true, only trees with at least
    /// one non-signaling predecessor are returned.
    pub fn recent_replacements(&self, full_rbf_only: bool) -> Result<Vec<ReplacementNode>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let txs = mempool.txs();
        let entries = mempool.entries();
        let graveyard = mempool.graveyard();

        // Collect every distinct tree-root replacer. A predecessor's
        // `by` may itself have been replaced; walk forward through
        // chained Replaced tombstones until reaching a tx that's no
        // longer flagged as replaced (live, Vanished, or unknown).
        let mut roots: FxHashSet<Txid> = FxHashSet::default();
        for (_, by) in graveyard.replaced_iter() {
            let mut root = by.clone();
            while let Some(TxRemoval::Replaced { by: next }) =
                graveyard.get(&root).map(TxTombstone::reason)
            {
                root = next.clone();
            }
            roots.insert(root);
        }

        let mut trees: Vec<ReplacementNode> = roots
            .iter()
            .filter_map(|root| {
                Self::build_rbf_node(root, None, &txs, &entries, &graveyard).map(|mut node| {
                    node.tx.full_rbf = Some(node.full_rbf);
                    node.interval = None;
                    node
                })
            })
            .filter(|node| !full_rbf_only || node.full_rbf)
            .collect();

        trees.sort_by(|a, b| b.time.cmp(&a.time));
        Ok(trees)
    }

    pub fn transaction_times(&self, txids: &[Txid]) -> Result<Vec<u64>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let entries = mempool.entries();
        Ok(txids
            .iter()
            .map(|txid| {
                entries
                    .get(&TxidPrefix::from(txid))
                    .map(|e| usize::from(e.first_seen) as u64)
                    .unwrap_or(0)
            })
            .collect())
    }
}
