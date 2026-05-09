use crate::Query;
use brk_error::{Error, Result};
use brk_mempool::{Mempool, PrevoutResolver, RbfForTx, RbfNode};
use brk_types::{
    BlockTemplate, BlockTemplateDiff, CheckedSub, FeeRate, MempoolBlock, MempoolInfo,
    MempoolRecentTx, NextBlockHash, OutputType, RbfResponse, RbfTx, RecommendedFees,
    ReplacementNode, Sats, Timestamp, TxOut, TxOutIndex, Txid, TxidPrefix, TypeIndex,
};

const RECENT_REPLACEMENTS_LIMIT: usize = 25;

impl Query {
    fn require_mempool(&self) -> Result<&Mempool> {
        self.mempool().ok_or(Error::MempoolNotAvailable)
    }

    pub fn mempool_info(&self) -> Result<MempoolInfo> {
        Ok(self.require_mempool()?.info())
    }

    pub fn mempool_txids(&self) -> Result<Vec<Txid>> {
        Ok(self.require_mempool()?.txids())
    }

    pub fn recommended_fees(&self) -> Result<RecommendedFees> {
        self.require_mempool().map(|m| m.fees())
    }

    pub fn mempool_blocks(&self) -> Result<Vec<MempoolBlock>> {
        let mempool = self.require_mempool()?;
        Ok(mempool.block_stats().iter().map(MempoolBlock::from).collect())
    }

    /// Indexer-backed resolver for confirmed-parent prevouts. Pass
    /// the returned closure to `Mempool::start_with` /
    /// `Mempool::update_with`; the mempool driver calls it post-apply
    /// for every still-unfilled `prevout == None` input.
    ///
    /// Reads go through `read_once` rather than a captured
    /// `VecReader`: `VecReader::stored_len` is snapshotted at
    /// construction, so a long-lived reader paired with fresh
    /// `safe_lengths` would let `safe.tx_index` / `safe.txout_index`
    /// advance past the reader's frozen length and panic in
    /// `reader.get()`. `read_once` rebinds against the current vec
    /// length per call and lets newly indexed parents become
    /// resolvable on the next cycle.
    pub fn indexer_prevout_resolver(&self) -> PrevoutResolver {
        let query = self.clone();
        let indexer = self.0.indexer;

        Box::new(move |prev_txid, vout| {
            let safe = query.safe_lengths();
            let prev_tx_index = indexer
                .stores
                .txid_prefix_to_tx_index
                .get(&TxidPrefix::from(prev_txid))
                .ok()??
                .into_owned();
            if prev_tx_index >= safe.tx_index {
                return None;
            }
            let first_txout: TxOutIndex = indexer
                .vecs
                .transactions
                .first_txout_index
                .read_once(prev_tx_index)
                .ok()?;
            let txout = first_txout + vout;
            if txout >= safe.txout_index {
                return None;
            }
            let output_type: OutputType = indexer.vecs.outputs.output_type.read_once(txout).ok()?;
            let type_index: TypeIndex = indexer.vecs.outputs.type_index.read_once(txout).ok()?;
            let value: Sats = indexer.vecs.outputs.value.read_once(txout).ok()?;
            let script_pubkey = indexer
                .vecs
                .addrs
                .addr_readers()
                .script_pubkey(output_type, type_index);
            Some(TxOut::from((script_pubkey, value)))
        })
    }

    pub fn mempool_recent(&self) -> Result<Vec<MempoolRecentTx>> {
        Ok(self.require_mempool()?.recent_txs())
    }

    /// RBF history for a tx. Matches mempool.space's
    /// `GET /api/v1/tx/:txid/rbf`. Mempool builds the owned tree under
    /// one read-lock window; this then layers on `mined` + effective
    /// fee rate from the indexer/computer.
    pub fn tx_rbf(&self, txid: &Txid) -> Result<RbfResponse> {
        let RbfForTx { root, replaces } = self.require_mempool()?.rbf_for_tx(txid);
        let replacements = root.map(|n| self.enrich_rbf_node(n, None));
        let replaces = (!replaces.is_empty()).then_some(replaces);
        Ok(RbfResponse {
            replacements,
            replaces,
        })
    }

    /// Recent RBF replacements. Matches mempool.space's
    /// `GET /api/v1/replacements` and `GET /api/v1/fullrbf/replacements`.
    /// Most-recent first, capped at 25. `full_rbf_only` keeps only
    /// trees with at least one non-signaling predecessor.
    pub fn recent_replacements(&self, full_rbf_only: bool) -> Result<Vec<ReplacementNode>> {
        Ok(self
            .require_mempool()?
            .recent_rbf_trees(full_rbf_only, RECENT_REPLACEMENTS_LIMIT)
            .into_iter()
            .map(|n| self.enrich_rbf_node(n, None))
            .collect())
    }

    /// Layer `mined` + effective fee rate onto an `RbfNode` tree.
    /// Must run after the mempool lock has dropped (effective_fee_rate
    /// re-enters Mempool).
    fn enrich_rbf_node(&self, node: RbfNode, successor_time: Option<Timestamp>) -> ReplacementNode {
        let interval = successor_time
            .and_then(|st| st.checked_sub(node.first_seen))
            .map(|d| *d);
        let mined = self.resolve_tx_index(&node.txid).is_ok().then_some(true);
        let rate = self
            .effective_fee_rate(&node.txid)
            .unwrap_or_else(|_| FeeRate::from((node.fee, node.vsize)));
        let first_seen = node.first_seen;
        let replaces = node
            .replaces
            .into_iter()
            .map(|child| self.enrich_rbf_node(child, Some(first_seen)))
            .collect();
        ReplacementNode {
            tx: RbfTx {
                txid: node.txid,
                fee: node.fee,
                vsize: node.vsize,
                value: node.value,
                rate,
                time: first_seen,
                rbf: node.rbf,
                full_rbf: Some(node.full_rbf),
            },
            time: first_seen,
            full_rbf: node.full_rbf,
            interval,
            mined,
            replaces,
        }
    }

    /// `first_seen` Unix-second timestamps. Matches mempool.space's
    /// `POST /api/v1/transaction-times`. Returns 0 for unknowns.
    pub fn transaction_times(&self, txids: &[Txid]) -> Result<Vec<u64>> {
        Ok(self.require_mempool()?.transaction_times(txids))
    }

    /// Content hash of the projected next block. Same value as the
    /// mempool ETag. Polling lets monitors detect a stalled sync.
    pub fn mempool_hash(&self) -> Result<NextBlockHash> {
        Ok(self.require_mempool()?.next_block_hash())
    }

    /// Full projected next block (Core's `getblocktemplate` selection)
    /// with stats and full tx bodies in GBT order.
    pub fn block_template(&self) -> Result<BlockTemplate> {
        Ok(self.require_mempool()?.block_template())
    }

    /// Delta of the projected next block since `since`. `NotFound`
    /// when `since` has aged out (client should fall back to
    /// `block_template`).
    pub fn block_template_diff(&self, since: NextBlockHash) -> Result<BlockTemplateDiff> {
        self.require_mempool()?
            .block_template_diff(since)
            .ok_or_else(|| Error::NotFound(format!("unknown since hash: {since}")))
    }
}
