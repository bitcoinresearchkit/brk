use std::str::FromStr;

use brk_error::{Error, Result};
use brk_types::{Address, AddressBytes, Txid};

use crate::Query;

/// Maximum number of mempool txids to return
const MAX_MEMPOOL_TXIDS: usize = 50;

/// Get mempool transaction IDs for an address
pub fn get_address_mempool_txids(address: Address, query: &Query) -> Result<Vec<Txid>> {
    let mempool = query.mempool().ok_or(Error::Str("Mempool not available"))?;

    let bytes = AddressBytes::from_str(&address.address)?;
    let addresses = mempool.get_addresses();

    let txids: Vec<Txid> = addresses
        .get(&bytes)
        .map(|(_, txids)| txids.iter().take(MAX_MEMPOOL_TXIDS).cloned().collect())
        .unwrap_or_default();

    Ok(txids)
}
