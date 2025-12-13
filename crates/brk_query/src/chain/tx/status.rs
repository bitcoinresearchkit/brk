use std::str::FromStr;

use brk_error::{Error, Result};
use brk_types::{TxStatus, Txid, TxidPath, TxidPrefix};
use vecdb::GenericStoredVec;

use crate::Query;

pub fn get_transaction_status(TxidPath { txid }: TxidPath, query: &Query) -> Result<TxStatus> {
    let Ok(txid) = bitcoin::Txid::from_str(&txid) else {
        return Err(Error::InvalidTxid);
    };

    let txid = Txid::from(txid);

    // First check mempool for unconfirmed transactions
    if let Some(mempool) = query.mempool()
        && mempool.get_txs().contains_key(&txid)
    {
        return Ok(TxStatus::UNCONFIRMED);
    }

    // Look up confirmed transaction by txid prefix
    let prefix = TxidPrefix::from(&txid);
    let indexer = query.indexer();
    let Ok(Some(txindex)) = indexer
        .stores
        .txidprefix_to_txindex
        .get(&prefix)
        .map(|opt| opt.map(|cow| cow.into_owned()))
    else {
        return Err(Error::UnknownTxid);
    };

    // Get block info for status
    let height = indexer.vecs.tx.txindex_to_height.read_once(txindex)?;
    let block_hash = indexer.vecs.block.height_to_blockhash.read_once(height)?;
    let block_time = indexer.vecs.block.height_to_timestamp.read_once(height)?;

    Ok(TxStatus {
        confirmed: true,
        block_height: Some(height),
        block_hash: Some(block_hash),
        block_time: Some(block_time),
    })
}
