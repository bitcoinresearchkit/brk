use brk_error::{Error, Result};
use brk_types::{Height, TxIndex, Txid};
use vecdb::{AnyVec, GenericStoredVec, TypedVecIterator};

use crate::Query;

/// Get a single txid at a specific index within a block
pub fn get_block_txid_at_index(height: Height, index: usize, query: &Query) -> Result<Txid> {
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

    if index >= tx_count {
        return Err(Error::Str("Transaction index out of range"));
    }

    let txindex = TxIndex::from(first + index);
    let txid = indexer.vecs.tx.txindex_to_txid.iter()?.get_unwrap(txindex);

    Ok(txid)
}
