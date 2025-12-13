use brk_error::{Error, Result};
use brk_types::{BlockHash, BlockHashPrefix, Height};

use crate::Query;

/// Resolve a block hash to height
pub fn get_height_by_hash(hash: &str, query: &Query) -> Result<Height> {
    let indexer = query.indexer();

    let blockhash: BlockHash = hash.parse().map_err(|_| Error::Str("Invalid block hash"))?;
    let prefix = BlockHashPrefix::from(&blockhash);

    indexer
        .stores
        .blockhashprefix_to_height
        .get(&prefix)?
        .map(|h| *h)
        .ok_or(Error::Str("Block not found"))
}
