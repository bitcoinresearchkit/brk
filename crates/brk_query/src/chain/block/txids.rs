use brk_error::{Error, Result};
use brk_types::{Height, TxIndex, Txid};
use vecdb::{AnyVec, GenericStoredVec};

use crate::Query;

/// Get all txids in a block by height
pub fn get_block_txids(height: Height, query: &Query) -> Result<Vec<Txid>> {
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
