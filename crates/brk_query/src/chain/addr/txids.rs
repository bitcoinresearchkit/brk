use brk_error::{Error, Result};
use brk_types::{Address, AddressIndexTxIndex, TxIndex, Txid, Unit};
use vecdb::TypedVecIterator;

use super::resolve::resolve_address;
use crate::Query;

/// Get transaction IDs for an address, newest first
pub fn get_address_txids(
    address: Address,
    after_txid: Option<Txid>,
    limit: usize,
    query: &Query,
) -> Result<Vec<Txid>> {
    let indexer = query.indexer();
    let stores = &indexer.stores;

    let (outputtype, type_index) = resolve_address(&address, query)?;

    let store = stores
        .addresstype_to_addressindex_and_txindex
        .get(outputtype)
        .unwrap();

    let prefix = u32::from(type_index).to_be_bytes();

    let after_txindex = if let Some(after_txid) = after_txid {
        let txindex = stores
            .txidprefix_to_txindex
            .get(&after_txid.into())
            .map_err(|_| Error::Str("Failed to look up after_txid"))?
            .ok_or(Error::Str("after_txid not found"))?
            .into_owned();
        Some(txindex)
    } else {
        None
    };

    let txindices: Vec<TxIndex> = store
        .prefix(prefix)
        .rev()
        .filter(|(key, _): &(AddressIndexTxIndex, Unit)| {
            if let Some(after) = after_txindex {
                TxIndex::from(key.txindex()) < after
            } else {
                true
            }
        })
        .take(limit)
        .map(|(key, _)| TxIndex::from(key.txindex()))
        .collect();

    let mut txindex_to_txid_iter = indexer.vecs.tx.txindex_to_txid.iter()?;
    let txids: Vec<Txid> = txindices
        .into_iter()
        .map(|txindex| txindex_to_txid_iter.get_unwrap(txindex))
        .collect();

    Ok(txids)
}
