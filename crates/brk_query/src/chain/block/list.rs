use brk_error::Result;
use brk_types::{BlockInfo, Height};

use crate::Query;

use super::info::get_block_by_height;

const DEFAULT_BLOCK_COUNT: u32 = 10;

/// Get a list of blocks, optionally starting from a specific height
pub fn get_blocks(start_height: Option<Height>, query: &Query) -> Result<Vec<BlockInfo>> {
    let max_height = query.get_height();

    let start = start_height.unwrap_or(max_height);
    let start = start.min(max_height);

    let start_u32: u32 = start.into();
    let count = DEFAULT_BLOCK_COUNT.min(start_u32 + 1);

    let mut blocks = Vec::with_capacity(count as usize);
    for i in 0..count {
        let height = Height::from(start_u32 - i);
        blocks.push(get_block_by_height(height, query)?);
    }

    Ok(blocks)
}
