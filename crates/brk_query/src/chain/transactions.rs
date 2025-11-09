use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    str::FromStr,
};

use bitcoin::consensus::Decodable;
use brk_error::{Error, Result};
use brk_reader::XORIndex;
use brk_types::{Transaction, Txid, TxidPath, TxidPrefix};
use vecdb::TypedVecIterator;

use crate::Query;

pub fn get_transaction(TxidPath { txid }: TxidPath, query: &Query) -> Result<Transaction> {
    let Ok(txid) = bitcoin::Txid::from_str(&txid) else {
        return Err(Error::InvalidTxid);
    };

    let txid = Txid::from(txid);
    let prefix = TxidPrefix::from(&txid);
    let indexer = query.indexer();
    let Ok(Some(index)) = indexer
        .stores
        .txidprefix_to_txindex
        .get(&prefix)
        .map(|opt| opt.map(|cow| cow.into_owned()))
    else {
        return Err(Error::UnknownTxid);
    };

    let txid = indexer.vecs.txindex_to_txid.iter()?.get_unwrap(index);

    let reader = query.reader();
    let computer = query.computer();

    let position = computer.blks.txindex_to_position.iter()?.get_unwrap(index);
    let len = indexer.vecs.txindex_to_total_size.iter()?.get_unwrap(index);

    let blk_index_to_blk_path = reader.blk_index_to_blk_path();

    let Some(blk_path) = blk_index_to_blk_path.get(&position.blk_index()) else {
        return Err(Error::Str("Failed to get the correct blk file"));
    };

    let mut xori = XORIndex::default();
    xori.add_assign(position.offset() as usize);

    let Ok(mut file) = File::open(blk_path) else {
        return Err(Error::Str("Failed to open blk file"));
    };

    if file
        .seek(SeekFrom::Start(position.offset() as u64))
        .is_err()
    {
        return Err(Error::Str("Failed to seek position in file"));
    }

    let mut buffer = vec![0u8; *len as usize];
    if file.read_exact(&mut buffer).is_err() {
        return Err(Error::Str("Failed to read the transaction (read exact)"));
    }
    xori.bytes(&mut buffer, reader.xor_bytes());

    let mut reader = Cursor::new(buffer);
    let Ok(_) = bitcoin::Transaction::consensus_decode(&mut reader) else {
        return Err(Error::Str("Failed decode the transaction"));
    };

    todo!();

    // Ok(TxInfo {
    //     txid,
    //     index,
    //     // tx
    // })
}
