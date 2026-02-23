use brk_error::{Error, Result};
use brk_types::{BlockHash, Height, Transaction, TxIndex, Txid};
use vecdb::{AnyVec, ReadableVec};

use super::BLOCK_TXS_PAGE_SIZE;
use crate::Query;

impl Query {
    pub fn block_txids(&self, hash: &BlockHash) -> Result<Vec<Txid>> {
        let height = self.height_by_hash(hash)?;
        self.block_txids_by_height(height)
    }

    pub fn block_txs(&self, hash: &BlockHash, start_index: TxIndex) -> Result<Vec<Transaction>> {
        let height = self.height_by_hash(hash)?;
        self.block_txs_by_height(height, start_index.into())
    }

    pub fn block_txid_at_index(&self, hash: &BlockHash, index: TxIndex) -> Result<Txid> {
        let height = self.height_by_hash(hash)?;
        self.block_txid_at_index_by_height(height, index.into())
    }

    // === Helper methods ===

    fn block_txids_by_height(&self, height: Height) -> Result<Vec<Txid>> {
        let indexer = self.indexer();

        let max_height = self.height();
        if height > max_height {
            return Err(Error::OutOfRange("Block height out of range".into()));
        }

        let first_txindex = indexer.vecs.transactions.first_txindex.collect_one(height).unwrap();
        let next_first_txindex = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_one(height.incremented())
            .unwrap_or_else(|| TxIndex::from(indexer.vecs.transactions.txid.len()));

        let first: usize = first_txindex.into();
        let next: usize = next_first_txindex.into();

        let txids: Vec<Txid> = indexer
            .vecs
            .transactions
            .txid
            .collect_range_at(first, next);

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
            return Err(Error::OutOfRange("Block height out of range".into()));
        }

        let first_txindex = indexer.vecs.transactions.first_txindex.collect_one(height).unwrap();
        let next_first_txindex = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_one(height.incremented())
            .unwrap_or_else(|| TxIndex::from(indexer.vecs.transactions.txid.len()));

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
            return Err(Error::OutOfRange("Block height out of range".into()));
        }

        let first_txindex = indexer.vecs.transactions.first_txindex.collect_one(height).unwrap();
        let next_first_txindex = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_one(height.incremented())
            .unwrap_or_else(|| TxIndex::from(indexer.vecs.transactions.txid.len()));

        let first: usize = first_txindex.into();
        let next: usize = next_first_txindex.into();
        let tx_count = next - first;

        if index >= tx_count {
            return Err(Error::OutOfRange("Transaction index out of range".into()));
        }

        let txindex = first + index;
        let txid = indexer.vecs.transactions.txid.reader().get(txindex);

        Ok(txid)
    }
}
