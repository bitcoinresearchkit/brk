use std::{str::FromStr, sync::Arc};

use brk_error::{Error, Result};
use brk_types::{Addr, AddrBytes, Transaction};

use crate::Query;

impl Query {
    /// Capture shared transaction bodies only when mempool and indexed chain
    /// refer to the same completed publication.
    pub fn addr_mempool_txs(&self, addr: &Addr, limit: usize) -> Result<Vec<Arc<Transaction>>> {
        let bytes = AddrBytes::from_str(addr)?;
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let _guard = self.read_publication()?;
        mempool.addr_txs(&bytes, limit, &self.tip_blockhash())
    }
}
