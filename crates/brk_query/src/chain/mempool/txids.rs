use brk_error::{Error, Result};
use brk_types::Txid;

use crate::Query;

/// Get all mempool transaction IDs
pub fn get_mempool_txids(query: &Query) -> Result<Vec<Txid>> {
    let mempool = query.mempool().ok_or(Error::Str("Mempool not available"))?;
    let txs = mempool.get_txs();
    Ok(txs.keys().cloned().collect())
}
