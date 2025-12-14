use brk_error::{Error, Result};
use brk_types::{Height, Transaction, TxIndex};
use vecdb::{AnyVec, GenericStoredVec};

use crate::{Query, chain::tx::get_transaction_by_index};

pub const BLOCK_TXS_PAGE_SIZE: usize = 25;

/// Get paginated transactions in a block by height
pub fn get_block_txs(height: Height, start_index: usize, query: &Query) -> Result<Vec<Transaction>> {
    let indexer = query.indexer();

    let max_height = query.get_height();
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
        let tx = get_transaction_by_index(txindex, query)?;
        txs.push(tx);
    }

    Ok(txs)
}
