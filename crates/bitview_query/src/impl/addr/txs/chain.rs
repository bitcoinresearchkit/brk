use brk_error::Result;
use brk_types::{Addr, BlockHash, Height, OutputType, Transaction, TxIndex, Txid, TypeIndex};

use crate::Query;

/// A confirmed address transaction page resolved against one best-chain view.
#[derive(Debug)]
pub struct ResolvedAddrChainTxs {
    txindices: Vec<TxIndex>,
    anchor_height: Option<Height>,
    activity_anchor: BlockHash,
}

impl ResolvedAddrChainTxs {
    /// The newest block represented by this page, when the page is non-empty.
    #[inline]
    pub fn block_hash(&self) -> Option<BlockHash> {
        self.anchor_height.map(|_| self.activity_anchor)
    }

    /// Latest relevant block, or the resolved tip while the page is empty.
    #[inline]
    pub const fn activity_anchor(&self) -> BlockHash {
        self.activity_anchor
    }
}

impl Query {
    pub fn addr_txs_chain(
        &self,
        addr: &Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Transaction>> {
        let _guard = self.read_publication()?;
        let txindices = self.addr_txindices(addr, after_txid, limit)?;
        let guard = self.pin_safe_lengths()?;
        self.transactions_at_indices(&txindices, &guard)
    }

    /// Resolve an address page once before body loading.
    pub fn resolve_addr_chain_txs(
        &self,
        addr: &Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<ResolvedAddrChainTxs> {
        let _guard = self.read_publication()?;
        let (output_type, type_index) = self.resolve_addr(addr)?;
        self.resolve_addr_chain_txs_for(output_type, type_index, after_txid, limit)
    }

    /// Load a previously resolved page after confirming its chain anchor survived.
    pub fn addr_txs_chain_resolved(
        &self,
        resolved: ResolvedAddrChainTxs,
    ) -> Result<Vec<Transaction>> {
        let _guard = self.read_publication()?;
        self.addr_txs_chain_at(resolved)
    }

    pub fn addr_txids(
        &self,
        addr: Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Txid>> {
        let _guard = self.read_publication()?;
        let txindices = self.addr_txindices(&addr, after_txid, limit)?;
        let txid_reader = self.indexer().vecs().transactions.txid.reader();
        Ok(txindices
            .into_iter()
            .map(|tx_index| txid_reader.get(tx_index))
            .collect())
    }

    fn addr_txindices(
        &self,
        addr: &Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<TxIndex>> {
        let (output_type, type_index) = self.resolve_addr(addr)?;
        self.addr_txindices_for(output_type, type_index, after_txid, limit)
    }

    fn addr_txindices_for(
        &self,
        output_type: OutputType,
        type_index: TypeIndex,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<TxIndex>> {
        let stores = self.indexer().stores();
        let tx_index_len = self.safe_lengths().tx_index;

        let before = after_txid
            .as_ref()
            .map(|txid| self.resolve_tx_index_bounded(txid))
            .transpose()?
            .unwrap_or(tx_index_len)
            .min(tx_index_len);
        Ok(stores
            .addr_tx_indexes_before(output_type, type_index, before)?
            .rev()
            .take(limit)
            .collect())
    }

    pub fn resolve_addr_chain_txs_for(
        &self,
        output_type: OutputType,
        type_index: TypeIndex,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<ResolvedAddrChainTxs> {
        let txindices = self.addr_txindices_for(output_type, type_index, after_txid, limit)?;
        let anchor = txindices
            .first()
            .map(|txindex| -> Result<_> {
                let height = self.confirmed_status_height(*txindex)?;
                let hash = self.resolve_block_hash(height)?;
                Ok((height, hash))
            })
            .transpose()?;
        let activity_anchor = anchor
            .map(|(_, hash)| hash)
            .unwrap_or_else(|| self.tip_blockhash());

        Ok(ResolvedAddrChainTxs {
            txindices,
            anchor_height: anchor.map(|(height, _)| height),
            activity_anchor,
        })
    }

    /// Caller holds publication exclusion across any source selection and read.
    pub fn addr_txs_chain_at(&self, resolved: ResolvedAddrChainTxs) -> Result<Vec<Transaction>> {
        let guard = self.pin_safe_lengths()?;
        if let Some(height) = resolved.anchor_height {
            self.validate_block_at_height(&resolved.activity_anchor, height, &guard)?;
        }
        self.transactions_at_indices(&resolved.txindices, &guard)
    }
}

#[cfg(test)]
#[path = "../../../../tests/unit/impl/addr/txs/chain.rs"]
mod tests;
