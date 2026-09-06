use crate::internals::*;

use brk_error::{Error, Result};
use brk_types::{CpfpInfo, Txid};
use serde_json::to_vec;

use crate::{
    Query, RepresentationId, r#impl::tx::ResolvedConfirmedTx, representation_id::content_hash,
};

/// CPFP JSON resolved to one exact live or confirmed transaction source.
pub struct ResolvedCpfp {
    source: ResolvedCpfpSource,
}

enum ResolvedCpfpSource {
    Memory { bytes: Vec<u8>, hash: u64 },
    Chain(ResolvedConfirmedTx),
}

pub enum CpfpSource {
    Memory(CpfpInfo),
    Chain(ResolvedConfirmedTx),
}

impl ResolvedCpfp {
    fn memory(info: CpfpInfo) -> Self {
        let bytes = to_vec(&info).unwrap();
        let hash = content_hash(&bytes);
        Self {
            source: ResolvedCpfpSource::Memory { bytes, hash },
        }
    }

    pub fn identity(&self) -> RepresentationId {
        match &self.source {
            ResolvedCpfpSource::Memory { hash, .. } => RepresentationId::Content(*hash),
            ResolvedCpfpSource::Chain(transaction) => transaction.identity(),
        }
    }
}

impl Query {
    /// Resolve CPFP JSON once before an async response handoff.
    pub fn resolve_cpfp(&self, txid: &Txid) -> Result<ResolvedCpfp> {
        Ok(match self.resolve_cpfp_source(txid)? {
            CpfpSource::Memory(info) => ResolvedCpfp::memory(info),
            CpfpSource::Chain(transaction) => ResolvedCpfp {
                source: ResolvedCpfpSource::Chain(transaction),
            },
        })
    }

    /// Build JSON bytes without repeating transaction resolution.
    pub fn cpfp_json_resolved(&self, cpfp: ResolvedCpfp) -> Result<Vec<u8>> {
        match cpfp.source {
            ResolvedCpfpSource::Memory { bytes, .. } => Ok(bytes),
            ResolvedCpfpSource::Chain(transaction) => {
                let info = self.confirmed_cpfp_resolved(transaction)?;
                Ok(to_vec(&info).unwrap())
            }
        }
    }
}
pub trait RImplCpfpResolvedQueryInternal: Sized {
    fn resolve_cpfp_source(&self, txid: &Txid) -> Result<CpfpSource>;
}
impl RImplCpfpResolvedQueryInternal for Query {
    fn resolve_cpfp_source(&self, txid: &Txid) -> Result<CpfpSource> {
        let _guard = self.read_plugin(self.indexer())?;
        match self.resolve_confirmed_tx_guarded(txid) {
            Ok(transaction) => Ok(CpfpSource::Chain(transaction)),
            Err(Error::UnknownTxid) => self
                .mempool()
                .ok_or(Error::UnknownTxid)?
                .cpfp_info(txid, &self.tip_blockhash())?
                .map(CpfpSource::Memory)
                .ok_or(Error::UnknownTxid),
            Err(error) => Err(error),
        }
    }
}
