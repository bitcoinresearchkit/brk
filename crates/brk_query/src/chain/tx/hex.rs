use std::str::FromStr;

use bitcoin::hex::DisplayHex;
use brk_error::{Error, Result};
use brk_types::{TxIndex, Txid, TxidPath, TxidPrefix};
use vecdb::GenericStoredVec;

use crate::Query;

pub fn get_transaction_hex(TxidPath { txid }: TxidPath, query: &Query) -> Result<String> {
    let Ok(txid) = bitcoin::Txid::from_str(&txid) else {
        return Err(Error::InvalidTxid);
    };

    let txid = Txid::from(txid);

    // First check mempool for unconfirmed transactions
    if let Some(mempool) = query.mempool()
        && let Some(tx_with_hex) = mempool.get_txs().get(&txid)
    {
        return Ok(tx_with_hex.hex().to_string());
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

    get_transaction_hex_by_index(txindex, query)
}

pub fn get_transaction_hex_by_index(txindex: TxIndex, query: &Query) -> Result<String> {
    let indexer = query.indexer();
    let reader = query.reader();
    let computer = query.computer();

    let total_size = indexer.vecs.tx.txindex_to_total_size.read_once(txindex)?;
    let position = computer.blks.txindex_to_position.read_once(txindex)?;

    let buffer = reader.read_raw_bytes(position, *total_size as usize)?;

    Ok(buffer.to_lower_hex_string())
}
