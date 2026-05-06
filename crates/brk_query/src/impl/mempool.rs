use brk_error::{Error, Result};
use brk_mempool::{EntryPool, Mempool, TxEntry, TxGraveyard, TxRemoval, TxStore, TxTombstone};
use brk_types::{
    CheckedSub, MempoolBlock, MempoolInfo, MempoolRecentTx, OutputType, RbfResponse, RbfTx,
    RecommendedFees, ReplacementNode, Sats, Timestamp, Transaction, TxOut, TxOutIndex, Txid,
    TxidPrefix, TypeIndex,
};
use rustc_hash::FxHashSet;
use vecdb::VecIndex;

use crate::Query;

const RECENT_REPLACEMENTS_LIMIT: usize = 25;

impl Query {
    fn require_mempool(&self) -> Result<&Mempool> {
        self.mempool().ok_or(Error::MempoolNotAvailable)
    }

    pub fn mempool_info(&self) -> Result<MempoolInfo> {
        Ok(self.require_mempool()?.info())
    }

    pub fn mempool_txids(&self) -> Result<Vec<Txid>> {
        let txs = self.require_mempool()?.txs();
        Ok(txs.keys().cloned().collect())
    }

    pub fn recommended_fees(&self) -> Result<RecommendedFees> {
        self.require_mempool().map(|m| m.fees())
    }

    pub fn mempool_blocks(&self) -> Result<Vec<MempoolBlock>> {
        let mempool = self.require_mempool()?;

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
        let safe = self.safe_lengths();
        let tx_index_len = safe.tx_index;
        let txout_index_len = safe.txout_index;
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
            if prev_tx_index >= tx_index_len {
                return None;
            }
            let first_txout: TxOutIndex = first_txout_index_reader.get(prev_tx_index.to_usize());
            let txout = first_txout + vout;
            if txout >= txout_index_len {
                return None;
            }
            let txout_index = usize::from(txout);
            let output_type: OutputType = output_type_reader.get(txout_index);
            let type_index: TypeIndex = type_index_reader.get(txout_index);
            let value: Sats = value_reader.get(txout_index);
            let script_pubkey = addr_readers.script_pubkey(output_type, type_index);
            Some(TxOut::from((script_pubkey, value)))
        })
    }

    pub fn mempool_recent(&self) -> Result<Vec<MempoolRecentTx>> {
        Ok(self.require_mempool()?.txs().recent().to_vec())
    }

    /// RBF history for a tx, matching mempool.space's
    /// `GET /api/v1/tx/:txid/rbf`. Walks forward through the graveyard
    /// to find the latest known replacer (tree root), then recursively
    /// walks `predecessors_of` backward to build the tree. `replaces`
    /// is the requested tx's own direct predecessors.
    pub fn tx_rbf(&self, txid: &Txid) -> Result<RbfResponse> {
        let mempool = self.require_mempool()?;
        let txs = mempool.txs();
        let entries = mempool.entries();
        let graveyard = mempool.graveyard();

        let root_txid = Self::walk_to_replacement_root(&graveyard, *txid);

        let replaces_vec: Vec<Txid> = graveyard.predecessors_of(txid).map(|(p, _)| *p).collect();
        let replaces = (!replaces_vec.is_empty()).then_some(replaces_vec);

        let replacements = self.build_rbf_node(&root_txid, None, &txs, &entries, &graveyard);

        Ok(RbfResponse {
            replacements,
            replaces,
        })
    }

    /// Walk forward through `Replaced { by }` links to the terminal
    /// replacer of an RBF chain. Returns `txid` itself if it's already
    /// the root.
    fn walk_to_replacement_root(graveyard: &TxGraveyard, mut root: Txid) -> Txid {
        while let Some(TxRemoval::Replaced { by }) = graveyard.get(&root).map(TxTombstone::reason) {
            root = *by;
        }
        root
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
    ///
    /// `rate` matches mempool.space's `tx.effectiveFeePerVsize` via
    /// `Query::effective_fee_rate`, with a fall-back to the entry's
    /// simple `fee/vsize` when the rate lookup fails.
    fn build_rbf_node(
        &self,
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
                self.build_rbf_node(pred_txid, Some(entry.first_seen), txs, entries, graveyard)
            })
            .collect();

        let full_rbf = replaces.iter().any(|c| !c.tx.rbf || c.full_rbf);

        let interval = successor_time
            .and_then(|st| st.checked_sub(entry.first_seen))
            .map(|d| *d);

        let value: Sats = tx.output.iter().map(|o| o.value).sum();
        let mined = self.resolve_tx_index(txid).is_ok().then_some(true);
        let rate = self
            .effective_fee_rate(txid)
            .unwrap_or_else(|_| entry.fee_rate());

        Some(ReplacementNode {
            tx: RbfTx {
                txid: *txid,
                fee: entry.fee,
                vsize: entry.vsize,
                value,
                rate,
                time: entry.first_seen,
                rbf: entry.rbf,
                full_rbf: Some(full_rbf),
                mined,
            },
            time: entry.first_seen,
            full_rbf,
            interval,
            mined,
            replaces,
        })
    }

    /// Recent RBF replacements across the whole mempool, matching
    /// mempool.space's `GET /api/v1/replacements` and
    /// `GET /api/v1/fullrbf/replacements`. Each entry is a complete
    /// replacement tree rooted at the terminal replacer; same shape as
    /// `tx_rbf().replacements`. Ordered by most-recent replacement
    /// event first (matches mempool.space's reversed-`replacedBy`
    /// iteration) and capped at 25 entries. When `full_rbf_only` is
    /// true, only trees with at least one non-signaling predecessor
    /// are returned.
    pub fn recent_replacements(&self, full_rbf_only: bool) -> Result<Vec<ReplacementNode>> {
        let mempool = self.require_mempool()?;
        let txs = mempool.txs();
        let entries = mempool.entries();
        let graveyard = mempool.graveyard();

        // A predecessor's `by` may itself be replaced; walk the chain
        // forward to the terminal replacer for each tree, dedup so each
        // tree is emitted once at its first (most recent) sighting.
        let mut seen: FxHashSet<Txid> = FxHashSet::default();
        Ok(graveyard
            .replaced_iter_recent_first()
            .filter_map(|(_, by)| {
                let root = Self::walk_to_replacement_root(&graveyard, *by);
                seen.insert(root).then_some(root)
            })
            .filter_map(|root| self.build_rbf_node(&root, None, &txs, &entries, &graveyard))
            .filter(|node| !full_rbf_only || node.full_rbf)
            .take(RECENT_REPLACEMENTS_LIMIT)
            .collect())
    }

    /// `first_seen` Unix-second timestamps for each txid, matching
    /// mempool.space's `POST /api/v1/transaction-times`. Returns 0 for
    /// unknown txids, in input order.
    pub fn transaction_times(&self, txids: &[Txid]) -> Result<Vec<u64>> {
        let entries = self.require_mempool()?.entries();
        Ok(txids
            .iter()
            .map(|txid| {
                entries
                    .get(&TxidPrefix::from(txid))
                    .map_or(0, |e| u64::from(e.first_seen))
            })
            .collect())
    }
}
