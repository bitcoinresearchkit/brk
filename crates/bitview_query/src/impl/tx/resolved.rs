use crate::internals::*;

use brk_error::{Error, Result};
use brk_types::{Transaction, Txid};
use std::sync::Arc;

use crate::Query;

use super::ResolvedConfirmedTx;

pub enum TransactionSource {
    Memory(Arc<Transaction>),
    Chain(ResolvedConfirmedTx),
}

impl Query {}
pub trait RImplTxResolvedQueryInternal: Sized {
    fn resolve_transaction_source(&self, txid: &Txid) -> Result<TransactionSource>;
}
impl RImplTxResolvedQueryInternal for Query {
    /// Resolve one exact live, confirmed, or recently vanished transaction.
    fn resolve_transaction_source(&self, txid: &Txid) -> Result<TransactionSource> {
        let _guard = self.read_plugin(self.indexer())?;
        match self.resolve_confirmed_tx_guarded(txid) {
            Ok(transaction) => Ok(TransactionSource::Chain(transaction)),
            Err(Error::UnknownTxid) => self
                .mempool()
                .ok_or(Error::UnknownTxid)?
                .transaction(txid, &self.tip_blockhash())?
                .map(TransactionSource::Memory)
                .ok_or(Error::UnknownTxid),
            Err(error) => Err(error),
        }
    }
}
