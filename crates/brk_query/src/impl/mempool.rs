use brk_error::{Error, Result};
use brk_mempool::{Mempool, RbfForTx, RbfNode};
use brk_types::{
    CheckedSub, FeeRate, MempoolBlock, MempoolInfo, MempoolRecentTx, OutputType, RbfResponse, RbfTx,
    RecommendedFees, ReplacementNode, Sats, Timestamp, TxOut, TxOutIndex, Txid, TxidPrefix,
    TypeIndex,
};
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
        Ok(self.require_mempool()?.txids())
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
    /// indexer. Driver calls this once per cycle right after
    /// `mempool.update()`. Returns true if at least one was filled.
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
    fn enrich_rbf_node(
        &self,
        node: RbfNode,
        successor_time: Option<Timestamp>,
    ) -> ReplacementNode {
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
                mined,
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
    pub fn mempool_hash(&self) -> Result<u64> {
        Ok(self.require_mempool()?.next_block_hash())
    }
}
