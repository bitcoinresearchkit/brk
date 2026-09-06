//! CPFP queries shared by live mempool and confirmed transactions.

use crate::internals::*;

pub mod confirmed;
pub mod resolved;

pub use resolved::ResolvedCpfp;

use brk_error::{Error, OptionData, Result};
use brk_types::{CpfpInfo, FeeRate, Txid};
use vecdb::ReadableVec;

use crate::Query;

use resolved::CpfpSource;

impl Query {
    /// Reconstruct the published same-block cluster, falling back to live
    /// mempool information only for a transaction not yet published.
    pub fn cpfp(&self, txid: &Txid) -> Result<CpfpInfo> {
        match self.resolve_cpfp_source(txid)? {
            CpfpSource::Memory(info) => Ok(info),
            CpfpSource::Chain(transaction) => self.confirmed_cpfp_resolved(transaction),
        }
    }

    /// Effective SFL chunk rate for live, confirmed, or replaced transactions.
    pub fn effective_fee_rate(&self, txid: &Txid) -> Result<FeeRate> {
        let plugins = self.plugins();
        let _guard = self.read_plugins(vec![plugins.indexer, plugins.transactions])?;
        match self.resolve_tx_index_bounded(txid) {
            Ok(index) => {
                return self
                    .plugins()
                    .transactions
                    .fees
                    .effective_fee_rate
                    .tx_index
                    .collect_one(index)
                    .data();
            }
            Err(Error::UnknownTxid) => {}
            Err(error) => return Err(error),
        }

        if let Some(mempool) = self.mempool()
            && let Some(rate) = mempool.effective_fee_rate(txid, &self.tip_blockhash())?
        {
            return Ok(rate);
        }

        Err(Error::UnknownTxid)
    }
}
