use std::cmp::Ordering;

use brk_error::{Error, Result};
use brk_mempool::{Entry, EntryPool, Removal, Tombstone, TxGraveyard, TxStore};
use brk_types::{
    CheckedSub, CpfpEntry, CpfpInfo, FeeRate, MempoolBlock, MempoolInfo, MempoolRecentTx,
    OutputType, RbfResponse, RbfTx, RecommendedFees, ReplacementNode, Sats, Timestamp, Transaction,
    TxOut, TxOutIndex, Txid, TxidPrefix, TypeIndex, VSize, Weight,
};
use rustc_hash::FxHashSet;
use vecdb::VecIndex;

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
        let entries = mempool.entries();
        let prefix = TxidPrefix::from(txid);

        let Some(entry) = entries.get(&prefix) else {
            return Ok(CpfpInfo::default());
        };

        // Ancestor walk doubles as package-rate aggregation. Stale
        // `depends` entries pointing at mined/evicted txs are silently
        // dropped via the live `entries.get` probe, so the aggregates
        // reflect only in-pool ancestors.
        let mut ancestors = Vec::new();
        let mut visited: FxHashSet<TxidPrefix> = FxHashSet::default();
        let mut package_fee = u64::from(entry.fee);
        let mut package_vsize = u64::from(entry.vsize);
        let mut stack: Vec<TxidPrefix> = entry.depends.to_vec();
        while let Some(p) = stack.pop() {
            if !visited.insert(p) {
                continue;
            }
            if let Some(anc) = entries.get(&p) {
                package_fee += u64::from(anc.fee);
                package_vsize += u64::from(anc.vsize);
                ancestors.push(CpfpEntry {
                    txid: anc.txid.clone(),
                    weight: Weight::from(anc.vsize),
                    fee: anc.fee,
                });
                stack.extend(anc.depends.iter().cloned());
            }
        }

        let mut descendants = Vec::new();
        for child_prefix in entries.children(&prefix) {
            if let Some(e) = entries.get(&child_prefix) {
                descendants.push(CpfpEntry {
                    txid: e.txid.clone(),
                    weight: Weight::from(e.vsize),
                    fee: e.fee,
                });
            }
        }

        let self_rate = entry.fee_rate();
        let package_rate = FeeRate::from((Sats::from(package_fee), VSize::from(package_vsize)));
        let effective_fee_per_vsize = if package_rate > self_rate {
            package_rate
        } else {
            self_rate
        };

        let best_descendant = descendants
            .iter()
            .max_by(|a, b| {
                FeeRate::from((a.fee, a.weight))
                    .partial_cmp(&FeeRate::from((b.fee, b.weight)))
                    .unwrap_or(Ordering::Equal)
            })
            .cloned();

        Ok(CpfpInfo {
            ancestors,
            best_descendant,
            descendants,
            effective_fee_per_vsize: Some(effective_fee_per_vsize),
            fee: Some(entry.fee),
            adjusted_vsize: Some(entry.vsize),
        })
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
        while let Some(Removal::Replaced { by }) = graveyard.get(&root_txid).map(Tombstone::reason)
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
    ) -> Option<(&'a Transaction, &'a Entry)> {
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
