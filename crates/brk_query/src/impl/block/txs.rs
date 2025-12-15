use brk_error::{Error, Result};
use brk_types::{Height, Transaction, TxIndex, Txid};
use vecdb::{AnyVec, GenericStoredVec, TypedVecIterator};

use super::BLOCK_TXS_PAGE_SIZE;
use crate::Query;

impl Query {
    pub fn block_txids(&self, hash: &str) -> Result<Vec<Txid>> {
        let height = self.height_by_hash(hash)?;
        self.block_txids_by_height(height)
    }

    pub fn block_txs(&self, hash: &str, start_index: usize) -> Result<Vec<Transaction>> {
        let height = self.height_by_hash(hash)?;
        self.block_txs_by_height(height, start_index)
    }

    pub fn block_txid_at_index(&self, hash: &str, index: usize) -> Result<Txid> {
        let height = self.height_by_hash(hash)?;
        self.block_txid_at_index_by_height(height, index)
    }

    // === Helper methods ===

    fn block_txids_by_height(&self, height: Height) -> Result<Vec<Txid>> {
        let indexer = self.indexer();

        let max_height = self.height();
        if height > max_height {
            return Err(Error::Str("Block height out of range"));
        }

        let first_txindex = indexer.vecs.tx.height_to_first_txindex.read_once(height)?;
        let next_first_txindex = indexer
            .vecs
            .tx
            .height_to_first_txindex
            .read_once(height.incremented())
            .unwrap_or_else(|_| TxIndex::from(indexer.vecs.tx.txindex_to_txid.len()));

        let first: usize = first_txindex.into();
        let next: usize = next_first_txindex.into();
        let count = next - first;

        let txids: Vec<Txid> = indexer
            .vecs
            .tx
            .txindex_to_txid
            .iter()?
            .skip(first)
            .take(count)
            .collect();

        Ok(txids)
    }

    fn block_txs_by_height(
        &self,
        height: Height,
        start_index: usize,
    ) -> Result<Vec<Transaction>> {
        let indexer = self.indexer();

        let max_height = self.height();
        if height > max_height {
            return Err(Error::Str("Block height out of range"));
        }

        let first_txindex = indexer.vecs.tx.height_to_first_txindex.read_once(height)?;
        let next_first_txindex = indexer
            .vecs
            .tx
            .height_to_first_txindex
            .read_once(height.incremented())
            .unwrap_or_else(|_| TxIndex::from(indexer.vecs.tx.txindex_to_txid.len()));

        let first: usize = first_txindex.into();
        let next: usize = next_first_txindex.into();
        let tx_count = next - first;

        if start_index >= tx_count {
            return Ok(Vec::new());
        }

        let end_index = (start_index + BLOCK_TXS_PAGE_SIZE).min(tx_count);
        let count = end_index - start_index;

        let mut txs = Vec::with_capacity(count);
        for i in start_index..end_index {
            let txindex = TxIndex::from(first + i);
            let tx = self.transaction_by_index(txindex)?;
            txs.push(tx);
        }

        Ok(txs)
    }

    fn block_txid_at_index_by_height(&self, height: Height, index: usize) -> Result<Txid> {
        let indexer = self.indexer();

        let max_height = self.height();
        if height > max_height {
            return Err(Error::Str("Block height out of range"));
        }

        let first_txindex = indexer.vecs.tx.height_to_first_txindex.read_once(height)?;
        let next_first_txindex = indexer
            .vecs
            .tx
            .height_to_first_txindex
            .read_once(height.incremented())
            .unwrap_or_else(|_| TxIndex::from(indexer.vecs.tx.txindex_to_txid.len()));

        let first: usize = first_txindex.into();
        let next: usize = next_first_txindex.into();
        let tx_count = next - first;

        if index >= tx_count {
            return Err(Error::Str("Transaction index out of range"));
        }

        let txindex = TxIndex::from(first + index);
        let txid = indexer.vecs.tx.txindex_to_txid.iter()?.get_unwrap(txindex);

        Ok(txid)
    }
}
