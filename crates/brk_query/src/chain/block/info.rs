use brk_error::{Error, Result};
use brk_types::{BlockInfo, Height, TxIndex};
use vecdb::{AnyVec, GenericStoredVec, VecIndex};

use crate::Query;

/// Get block info by height
pub fn get_block_by_height(height: Height, query: &Query) -> Result<BlockInfo> {
    let indexer = query.indexer();

    let max_height = max_height(query);
    if height > max_height {
        return Err(Error::Str("Block height out of range"));
    }

    let blockhash = indexer.vecs.block.height_to_blockhash.read_once(height)?;
    let difficulty = indexer.vecs.block.height_to_difficulty.read_once(height)?;
    let timestamp = indexer.vecs.block.height_to_timestamp.read_once(height)?;
    let size = indexer.vecs.block.height_to_total_size.read_once(height)?;
    let weight = indexer.vecs.block.height_to_weight.read_once(height)?;
    let tx_count = tx_count_at_height(height, max_height, query)?;

    Ok(BlockInfo {
        id: blockhash,
        height,
        tx_count,
        size: *size,
        weight,
        timestamp,
        difficulty: *difficulty,
    })
}

fn max_height(query: &Query) -> Height {
    Height::from(
        query
            .indexer()
            .vecs
            .block
            .height_to_blockhash
            .len()
            .saturating_sub(1),
    )
}

fn tx_count_at_height(height: Height, max_height: Height, query: &Query) -> Result<u32> {
    let indexer = query.indexer();
    let computer = query.computer();

    let first_txindex = indexer.vecs.tx.height_to_first_txindex.read_once(height)?;
    let next_first_txindex = if height < max_height {
        indexer
            .vecs
            .tx
            .height_to_first_txindex
            .read_once(height.incremented())?
    } else {
        TxIndex::from(computer.indexes.txindex_to_txindex.len())
    };

    Ok((next_first_txindex.to_usize() - first_txindex.to_usize()) as u32)
}
