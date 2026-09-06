use crate::internals::*;

use bitcoin::hex::DisplayHex;
use brk_error::Result;
use brk_types::{Transaction, Txid};

use crate::{Query, RepresentationId, representation_id::content_hash};

use super::{ResolvedConfirmedTx, resolved::TransactionSource};

/// Raw transaction data resolved to one exact in-memory or indexed source.
pub struct ResolvedRawTransaction {
    source: RawTransactionSource,
}

enum RawTransactionSource {
    Memory { bytes: Vec<u8>, hash: u64 },
    Chain(ResolvedConfirmedTx),
}

impl ResolvedRawTransaction {
    fn memory(transaction: &Transaction) -> Self {
        let bytes = transaction.encode_bytes();
        let hash = content_hash(&bytes);
        Self {
            source: RawTransactionSource::Memory { bytes, hash },
        }
    }

    pub fn identity(&self) -> RepresentationId {
        match &self.source {
            RawTransactionSource::Memory { hash, .. } => RepresentationId::Content(*hash),
            RawTransactionSource::Chain(transaction) => transaction.identity(),
        }
    }
}

impl Query {
    /// Resolve raw transaction data once before an async response handoff.
    pub fn resolve_raw_transaction(&self, txid: &Txid) -> Result<ResolvedRawTransaction> {
        Ok(match self.resolve_transaction_source(txid)? {
            TransactionSource::Memory(transaction) => ResolvedRawTransaction::memory(&transaction),
            TransactionSource::Chain(transaction) => ResolvedRawTransaction {
                source: RawTransactionSource::Chain(transaction),
            },
        })
    }

    /// Read raw bytes without repeating the transaction-prefix lookup.
    pub fn transaction_raw_resolved(&self, transaction: ResolvedRawTransaction) -> Result<Vec<u8>> {
        match transaction.source {
            RawTransactionSource::Memory { bytes, .. } => Ok(bytes),
            RawTransactionSource::Chain(transaction) => {
                let _guard = self.read_plugin(self.indexer())?;
                let (_, index, _) = self.revalidate_confirmed_tx(transaction)?;
                let bytes = self.transaction_raw_by_index(index)?;
                Ok(bytes)
            }
        }
    }

    /// Hex-encode raw bytes without repeating transaction resolution.
    pub fn transaction_hex_resolved(&self, transaction: ResolvedRawTransaction) -> Result<String> {
        self.transaction_raw_resolved(transaction)
            .map(|bytes| bytes.to_lower_hex_string())
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/impl/tx/raw.rs"]
mod tests;
