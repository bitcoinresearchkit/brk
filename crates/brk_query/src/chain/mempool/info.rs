use brk_error::{Error, Result};
use brk_types::MempoolInfo;

use crate::Query;

/// Get mempool statistics
pub fn get_mempool_info(query: &Query) -> Result<MempoolInfo> {
    let mempool = query.mempool().ok_or(Error::Str("Mempool not available"))?;
    Ok(mempool.get_info())
}
