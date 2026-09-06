use crate::internals::*;

use std::{str::FromStr, sync::Arc};

use brk_error::{Error, Result};
use brk_types::{Addr, AddrBytes, Transaction};

use super::ResolvedAddrTxs;
use crate::Query;

impl Query {
    /// Select both parts against the same indexed chain. Mempool bodies are
    /// immutable shared values; publication exclusion keeps chain rows stable.
    pub fn resolve_addr_txs(
        &self,
        addr: &Addr,
        mempool_limit: usize,
        chain_floor: usize,
        total_target: usize,
    ) -> Result<ResolvedAddrTxs> {
        let addr = AddrBytes::from_str(addr)?;
        let guard = self.read_plugin(self.indexer())?;
        let chain_addr = self.find_addr_bytes(&addr)?;
        let mempool = self
            .mempool()
            .map(|mempool| mempool.addr_txs(&addr, mempool_limit, &self.tip_blockhash()))
            .transpose()?
            .unwrap_or_default();
        if chain_addr.is_none() && mempool.is_empty() {
            return Err(Error::UnknownAddr);
        }
        let chain_limit = total_target.saturating_sub(mempool.len()).max(chain_floor);
        let chain = chain_addr
            .map(|(output_type, type_index)| {
                self.resolve_addr_chain_txs_for(output_type, type_index, None, chain_limit)
            })
            .transpose()?;
        Ok(ResolvedAddrTxs::new(guard, mempool, chain))
    }

    pub fn addr_txs_resolved(&self, resolved: ResolvedAddrTxs) -> Result<Vec<Arc<Transaction>>> {
        let (_guard, mut mempool, chain) = resolved.into_parts();
        if let Some(chain) = chain {
            mempool.extend(self.addr_txs_chain_at(chain)?.into_iter().map(Arc::new));
        }
        Ok(mempool)
    }
}
