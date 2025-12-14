use brk_error::{Error, Result};
use brk_types::MempoolBlock;

use crate::Query;

/// Get projected mempool blocks for fee estimation
pub fn get_mempool_blocks(query: &Query) -> Result<Vec<MempoolBlock>> {
    let mempool = query.mempool().ok_or(Error::Str("Mempool not available"))?;

    let block_stats = mempool.get_block_stats();

    let blocks = block_stats
        .into_iter()
        .map(|stats| {
            MempoolBlock::new(stats.tx_count, stats.total_vsize, stats.total_fee, stats.fee_range)
        })
        .collect();

    Ok(blocks)
}
