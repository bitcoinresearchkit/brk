use bitview_plugin::PublicationReadGuard;
use brk_error::{Error, Result};
use brk_types::{BlockHash, Height, TxIndex, Txid};
use vecdb::ReadableVec;

use super::ResolvedConfirmedTx;
use crate::Query;

/// One logical read of this query's published indexer state.
///
/// Only the constructor below can pair a query with the shared pipeline guard.
/// Dependent reads must reuse this view instead of acquiring another guard.
pub(crate) struct IndexerRead<'a> {
    query: &'a Query,
    _guard: PublicationReadGuard,
}

impl Query {
    pub(crate) fn read_indexer(&self) -> Result<IndexerRead<'_>> {
        Ok(IndexerRead {
            query: self,
            _guard: self.read_publication()?,
        })
    }
}

impl IndexerRead<'_> {
    pub(crate) fn query(&self) -> &Query {
        self.query
    }

    /// Validate between safe-bound snapshots. Rollback lowers the published
    /// bounds before mutating transaction vectors or confirmed-height data.
    fn validate_confirmed_position(&self, txid: &Txid, index: TxIndex) -> Result<Height> {
        let query = self.query;
        let safe = query.safe_lengths();
        if index >= safe.tx_index
            || query.indexer().vecs().transactions.txid.collect_one(index) != Some(*txid)
        {
            return Err(Error::UnknownTxid);
        }

        let height = query.confirmed_status_height(index)?;
        let safe = query.safe_lengths();
        if index >= safe.tx_index || height >= safe.height {
            return Err(Error::UnknownTxid);
        }

        Ok(height)
    }

    /// Read a confirmed block hash between safe-bound snapshots.
    fn confirmed_block_hash(&self, height: Height) -> Result<BlockHash> {
        let query = self.query;
        if height >= query.safe_lengths().height {
            return Err(Error::UnknownTxid);
        }
        let hash = query
            .indexer()
            .vecs()
            .blocks
            .blockhash
            .inner
            .collect_one(height)
            .ok_or(Error::UnknownTxid)?;
        if height >= query.safe_lengths().height {
            return Err(Error::UnknownTxid);
        }
        Ok(hash)
    }

    pub(crate) fn resolve_confirmed_tx(&self, txid: &Txid) -> Result<ResolvedConfirmedTx> {
        let (index, height) = self.resolve_confirmed_position(txid)?;
        let block_hash = self.confirmed_block_hash(height)?;
        Ok(ResolvedConfirmedTx {
            txid: *txid,
            index,
            height,
            block_hash,
        })
    }

    /// Resolve the position without reading a block hash the caller may not need.
    pub(crate) fn resolve_confirmed_position(&self, txid: &Txid) -> Result<(TxIndex, Height)> {
        let index = self.query.resolve_tx_index(txid)?;
        let height = self.validate_confirmed_position(txid, index)?;
        Ok((index, height))
    }

    /// Revalidate an async handoff without repeating its txid-prefix store lookup.
    pub(crate) fn revalidate_confirmed_tx(
        &self,
        tx: ResolvedConfirmedTx,
    ) -> Result<(Txid, TxIndex, Height)> {
        // The block hash commits to the transaction list and its ordering.
        // Since the token was exactly verified when constructed, confirming
        // that the same block remains at the same height is sufficient.
        if self.confirmed_block_hash(tx.height)? != tx.block_hash {
            return Err(Error::UnknownTxid);
        }
        Ok((tx.txid, tx.index, tx.height))
    }
}
