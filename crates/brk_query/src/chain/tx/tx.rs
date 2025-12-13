use std::{io::Cursor, str::FromStr};

use bitcoin::consensus::Decodable;
use brk_error::{Error, Result};
use brk_types::{
    Sats, Transaction, TxIn, TxIndex, TxOut, TxStatus, Txid, TxidPath, TxidPrefix, Vout, Weight,
};
use vecdb::{GenericStoredVec, TypedVecIterator};

use crate::Query;

pub fn get_transaction(TxidPath { txid }: TxidPath, query: &Query) -> Result<Transaction> {
    let Ok(txid) = bitcoin::Txid::from_str(&txid) else {
        return Err(Error::InvalidTxid);
    };

    let txid = Txid::from(txid);

    // First check mempool for unconfirmed transactions
    if let Some(mempool) = query.mempool()
        && let Some(tx_with_hex) = mempool.get_txs().get(&txid)
    {
        return Ok(tx_with_hex.tx().clone());
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

    get_transaction_by_index(txindex, query)
}

pub fn get_transaction_by_index(txindex: TxIndex, query: &Query) -> Result<Transaction> {
    let indexer = query.indexer();
    let reader = query.reader();
    let computer = query.computer();

    // Get tx metadata using read_once for single lookups
    let txid = indexer.vecs.tx.txindex_to_txid.read_once(txindex)?;
    let height = indexer.vecs.tx.txindex_to_height.read_once(txindex)?;
    let version = indexer.vecs.tx.txindex_to_txversion.read_once(txindex)?;
    let lock_time = indexer.vecs.tx.txindex_to_rawlocktime.read_once(txindex)?;
    let total_size = indexer.vecs.tx.txindex_to_total_size.read_once(txindex)?;
    let first_txinindex = indexer
        .vecs
        .tx
        .txindex_to_first_txinindex
        .read_once(txindex)?;
    let position = computer.blks.txindex_to_position.read_once(txindex)?;

    // Get block info for status
    let block_hash = indexer.vecs.block.height_to_blockhash.read_once(height)?;
    let block_time = indexer.vecs.block.height_to_timestamp.read_once(height)?;

    // Read and decode the raw transaction from blk file
    let buffer = reader.read_raw_bytes(position, *total_size as usize)?;
    let mut cursor = Cursor::new(buffer);
    let tx = bitcoin::Transaction::consensus_decode(&mut cursor)
        .map_err(|_| Error::Str("Failed to decode transaction"))?;

    // For iterating through inputs, we need iterators (multiple lookups)
    let mut txindex_to_txid_iter = indexer.vecs.tx.txindex_to_txid.iter()?;
    let mut txindex_to_first_txoutindex_iter =
        indexer.vecs.tx.txindex_to_first_txoutindex.iter()?;
    let mut txinindex_to_outpoint_iter = indexer.vecs.txin.txinindex_to_outpoint.iter()?;
    let mut txoutindex_to_value_iter = indexer.vecs.txout.txoutindex_to_value.iter()?;

    // Build inputs with prevout information
    let input: Vec<TxIn> = tx
        .input
        .iter()
        .enumerate()
        .map(|(i, txin)| {
            let txinindex = first_txinindex + i;
            let outpoint = txinindex_to_outpoint_iter.get_unwrap(txinindex);

            let is_coinbase = outpoint.is_coinbase();

            // Get prevout info if not coinbase
            let (prev_txid, prev_vout, prevout) = if is_coinbase {
                (Txid::COINBASE, Vout::MAX, None)
            } else {
                let prev_txindex = outpoint.txindex();
                let prev_vout = outpoint.vout();
                let prev_txid = txindex_to_txid_iter.get_unwrap(prev_txindex);

                // Calculate the txoutindex for the prevout
                let prev_first_txoutindex =
                    txindex_to_first_txoutindex_iter.get_unwrap(prev_txindex);
                let prev_txoutindex = prev_first_txoutindex + prev_vout;

                // Get the value of the prevout
                let prev_value = txoutindex_to_value_iter.get_unwrap(prev_txoutindex);

                // We don't have the script_pubkey stored directly, so we need to reconstruct
                // For now, we'll get it from the decoded transaction's witness/scriptsig
                // which can reveal the prevout script type, but the actual script needs
                // to be fetched from the spending tx or reconstructed from address bytes
                let prevout = Some(TxOut::from((
                    bitcoin::ScriptBuf::new(), // Placeholder - would need to reconstruct
                    prev_value,
                )));

                (prev_txid, prev_vout, prevout)
            };

            TxIn {
                txid: prev_txid,
                vout: prev_vout,
                prevout,
                script_sig: txin.script_sig.clone(),
                script_sig_asm: (),
                is_coinbase,
                sequence: txin.sequence.0,
                inner_redeem_script_asm: (),
            }
        })
        .collect();

    // Calculate weight before consuming tx.output
    let weight = Weight::from(tx.weight());

    // Calculate sigop cost
    // Note: Using |_| None means P2SH and SegWit sigops won't be counted accurately
    // since we don't provide the prevout scripts. This matches mempool tx behavior.
    // For accurate counting, we'd need to reconstruct prevout scripts from indexed data.
    let total_sigop_cost = tx.total_sigop_cost(|_| None);

    // Build outputs
    let output: Vec<TxOut> = tx.output.into_iter().map(TxOut::from).collect();

    // Build status
    let status = TxStatus {
        confirmed: true,
        block_height: Some(height),
        block_hash: Some(block_hash),
        block_time: Some(block_time),
    };

    let mut transaction = Transaction {
        index: Some(txindex),
        txid,
        version,
        lock_time,
        total_size: *total_size as usize,
        weight,
        total_sigop_cost,
        fee: Sats::ZERO, // Will be computed below
        input,
        output,
        status,
    };

    // Compute fee from inputs - outputs
    transaction.compute_fee();

    Ok(transaction)
}
