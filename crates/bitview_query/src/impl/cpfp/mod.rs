//! CPFP queries shared by live mempool and confirmed transactions.

pub mod confirmed;
pub mod resolved;

pub use resolved::ResolvedCpfp;

use brk_error::{Error, OptionData, Result};
use brk_types::{CpfpInfo, FeeRate, Txid};
use vecdb::ReadableVec;

use crate::{Query, r#impl::tx::ResolvedConfirmedTx};

enum CpfpSource {
    Memory(CpfpInfo),
    Chain(ResolvedConfirmedTx),
}

impl Query {
    /// Reconstruct the published same-block cluster, falling back to live
    /// mempool information only for a transaction not yet published.
    pub fn cpfp(&self, txid: &Txid) -> Result<CpfpInfo> {
        match self
            .resolve_cpfp_source(txid)
            .map_err(|error| self.transaction_error(error))?
        {
            CpfpSource::Memory(info) => Ok(info),
            CpfpSource::Chain(transaction) => self.confirmed_cpfp_resolved(transaction),
        }
    }

    /// Effective SFL chunk rate for live, confirmed, or replaced transactions.
    pub fn effective_fee_rate(&self, txid: &Txid) -> Result<FeeRate> {
        let read = self.read_indexer()?;
        match read.resolve_confirmed_tx(txid) {
            Ok(transaction) => {
                // Acquire publication before the prefix pin: never wait for
                // the pipeline while retaining a pin needed by its rollback.
                drop(read);
                let _publication = self.read_publication()?;
                let read = self.read_indexer()?;
                let (_, index, _) = read.revalidate_confirmed_tx(transaction)?;
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
            && let Some(rate) =
                mempool.effective_fee_rate(txid, &self.tip_blockhash_at(read.pin())?)?
        {
            return Ok(rate);
        }

        Err(Error::UnknownTxid)
    }

    fn resolve_cpfp_source(&self, txid: &Txid) -> Result<CpfpSource> {
        let read = self.read_indexer()?;
        match read.resolve_confirmed_tx(txid) {
            Ok(transaction) => Ok(CpfpSource::Chain(transaction)),
            Err(Error::UnknownTxid) => self
                .mempool()
                .ok_or(Error::UnknownTxid)?
                .cpfp_info(txid, &self.tip_blockhash_at(read.pin())?)?
                .map(CpfpSource::Memory)
                .ok_or(Error::UnknownTxid),
            Err(error) => Err(error),
        }
    }
}
