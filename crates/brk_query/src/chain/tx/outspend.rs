use std::str::FromStr;

use brk_error::{Error, Result};
use brk_types::{TxInIndex, TxOutspend, TxStatus, Txid, TxidPath, TxidPrefix, Vin, Vout};
use vecdb::{GenericStoredVec, TypedVecIterator};

use crate::Query;

/// Get the spend status of a specific output
pub fn get_tx_outspend(
    TxidPath { txid }: TxidPath,
    vout: Vout,
    query: &Query,
) -> Result<TxOutspend> {
    let Ok(txid) = bitcoin::Txid::from_str(&txid) else {
        return Err(Error::InvalidTxid);
    };

    let txid = Txid::from(txid);

    // Mempool outputs are unspent in on-chain terms
    if let Some(mempool) = query.mempool()
        && mempool.get_txs().contains_key(&txid)
    {
        return Ok(TxOutspend::UNSPENT);
    }

    // Look up confirmed transaction
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

    // Calculate txoutindex
    let first_txoutindex = indexer
        .vecs
        .tx
        .txindex_to_first_txoutindex
        .read_once(txindex)?;
    let txoutindex = first_txoutindex + vout;

    // Look up spend status
    let computer = query.computer();
    let txinindex = computer
        .stateful
        .txoutindex_to_txinindex
        .read_once(txoutindex)?;

    if txinindex == TxInIndex::UNSPENT {
        return Ok(TxOutspend::UNSPENT);
    }

    get_outspend_details(txinindex, query)
}

/// Get the spend status of all outputs in a transaction
pub fn get_tx_outspends(TxidPath { txid }: TxidPath, query: &Query) -> Result<Vec<TxOutspend>> {
    let Ok(txid) = bitcoin::Txid::from_str(&txid) else {
        return Err(Error::InvalidTxid);
    };

    let txid = Txid::from(txid);

    // Mempool outputs are unspent in on-chain terms
    if let Some(mempool) = query.mempool()
        && let Some(tx_with_hex) = mempool.get_txs().get(&txid)
    {
        let output_count = tx_with_hex.tx().output.len();
        return Ok(vec![TxOutspend::UNSPENT; output_count]);
    }

    // Look up confirmed transaction
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

    // Get output range
    let first_txoutindex = indexer
        .vecs
        .tx
        .txindex_to_first_txoutindex
        .read_once(txindex)?;
    let next_first_txoutindex = indexer
        .vecs
        .tx
        .txindex_to_first_txoutindex
        .read_once(txindex.incremented())?;
    let output_count = usize::from(next_first_txoutindex) - usize::from(first_txoutindex);

    // Get spend status for each output
    let computer = query.computer();
    let mut txoutindex_to_txinindex_iter = computer.stateful.txoutindex_to_txinindex.iter()?;

    let mut outspends = Vec::with_capacity(output_count);
    for i in 0..output_count {
        let txoutindex = first_txoutindex + Vout::from(i);
        let txinindex = txoutindex_to_txinindex_iter.get_unwrap(txoutindex);

        if txinindex == TxInIndex::UNSPENT {
            outspends.push(TxOutspend::UNSPENT);
        } else {
            outspends.push(get_outspend_details(txinindex, query)?);
        }
    }

    Ok(outspends)
}

/// Get spending transaction details from a txinindex
fn get_outspend_details(txinindex: TxInIndex, query: &Query) -> Result<TxOutspend> {
    let indexer = query.indexer();

    // Look up spending txindex directly
    let spending_txindex = indexer
        .vecs
        .txin
        .txinindex_to_txindex
        .read_once(txinindex)?;

    // Calculate vin
    let spending_first_txinindex = indexer
        .vecs
        .tx
        .txindex_to_first_txinindex
        .read_once(spending_txindex)?;
    let vin = Vin::from(usize::from(txinindex) - usize::from(spending_first_txinindex));

    // Get spending tx details
    let spending_txid = indexer
        .vecs
        .tx
        .txindex_to_txid
        .read_once(spending_txindex)?;
    let spending_height = indexer
        .vecs
        .tx
        .txindex_to_height
        .read_once(spending_txindex)?;
    let block_hash = indexer
        .vecs
        .block
        .height_to_blockhash
        .read_once(spending_height)?;
    let block_time = indexer
        .vecs
        .block
        .height_to_timestamp
        .read_once(spending_height)?;

    Ok(TxOutspend {
        spent: true,
        txid: Some(spending_txid),
        vin: Some(vin),
        status: Some(TxStatus {
            confirmed: true,
            block_height: Some(spending_height),
            block_hash: Some(block_hash),
            block_time: Some(block_time),
        }),
    })
}
