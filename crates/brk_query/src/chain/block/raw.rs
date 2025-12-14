use brk_error::{Error, Result};
use brk_types::Height;
use vecdb::{AnyVec, GenericStoredVec};

use crate::Query;

/// Get raw block bytes by height
pub fn get_block_raw(height: Height, query: &Query) -> Result<Vec<u8>> {
    let indexer = query.indexer();
    let computer = query.computer();
    let reader = query.reader();

    let max_height = Height::from(
        indexer
            .vecs
            .block
            .height_to_blockhash
            .len()
            .saturating_sub(1),
    );
    if height > max_height {
        return Err(Error::Str("Block height out of range"));
    }

    let position = computer.blks.height_to_position.read_once(height)?;
    let size = indexer.vecs.block.height_to_total_size.read_once(height)?;

    reader.read_raw_bytes(position, *size as usize)
}
