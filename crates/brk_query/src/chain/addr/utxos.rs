use brk_error::Result;
use brk_types::{
    Address, AddressIndexOutPoint, Sats, TxIndex, TxStatus, Txid, Unit, Utxo, Vout,
};
use vecdb::TypedVecIterator;

use super::resolve::resolve_address;
use crate::Query;

/// Get UTXOs for an address
pub fn get_address_utxos(address: Address, query: &Query) -> Result<Vec<Utxo>> {
    let indexer = query.indexer();
    let stores = &indexer.stores;
    let vecs = &indexer.vecs;

    let (outputtype, type_index) = resolve_address(&address, query)?;

    let store = stores
        .addresstype_to_addressindex_and_unspentoutpoint
        .get(outputtype)
        .unwrap();

    let prefix = u32::from(type_index).to_be_bytes();

    // Collect outpoints (txindex, vout)
    let outpoints: Vec<(TxIndex, Vout)> = store
        .prefix(prefix)
        .map(|(key, _): (AddressIndexOutPoint, Unit)| (key.txindex(), key.vout()))
        .collect();

    // Create iterators for looking up tx data
    let mut txindex_to_txid_iter = vecs.tx.txindex_to_txid.iter()?;
    let mut txindex_to_height_iter = vecs.tx.txindex_to_height.iter()?;
    let mut txindex_to_first_txoutindex_iter = vecs.tx.txindex_to_first_txoutindex.iter()?;
    let mut txoutindex_to_value_iter = vecs.txout.txoutindex_to_value.iter()?;
    let mut height_to_blockhash_iter = vecs.block.height_to_blockhash.iter()?;
    let mut height_to_timestamp_iter = vecs.block.height_to_timestamp.iter()?;

    let utxos: Vec<Utxo> = outpoints
        .into_iter()
        .map(|(txindex, vout)| {
            let txid: Txid = txindex_to_txid_iter.get_unwrap(txindex);
            let height = txindex_to_height_iter.get_unwrap(txindex);
            let first_txoutindex = txindex_to_first_txoutindex_iter.get_unwrap(txindex);
            let txoutindex = first_txoutindex + vout;
            let value: Sats = txoutindex_to_value_iter.get_unwrap(txoutindex);
            let block_hash = height_to_blockhash_iter.get_unwrap(height);
            let block_time = height_to_timestamp_iter.get_unwrap(height);

            Utxo {
                txid,
                vout,
                status: TxStatus {
                    confirmed: true,
                    block_height: Some(height),
                    block_hash: Some(block_hash),
                    block_time: Some(block_time),
                },
                value,
            }
        })
        .collect();

    Ok(utxos)
}
