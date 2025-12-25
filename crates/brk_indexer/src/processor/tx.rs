//! TXID computation and collision checking.

use brk_error::{Error, Result};
use brk_types::{StoredBool, TxIndex, Txid, TxidPrefix};
use rayon::prelude::*;
use vecdb::{AnyVec, GenericStoredVec, TypedVecIterator, likely};

use crate::constants::DUPLICATE_TXIDS;

use super::{BlockProcessor, ComputedTx};

impl<'a> BlockProcessor<'a> {
    /// Compute TXIDs in parallel (CPU-intensive operation).
    pub fn compute_txids(&self) -> Result<Vec<ComputedTx<'a>>> {
        let will_check_collisions =
            self.check_collisions && self.stores.txidprefix_to_txindex.needs(self.height);
        let base_txindex = self.indexes.txindex;

        self.block
            .txdata
            .par_iter()
            .enumerate()
            .map(|(index, tx)| {
                let txid = Txid::from(tx.compute_txid());
                let txid_prefix = TxidPrefix::from(&txid);

                let prev_txindex_opt = if will_check_collisions {
                    self.stores
                        .txidprefix_to_txindex
                        .get(&txid_prefix)?
                        .map(|v| *v)
                } else {
                    None
                };

                Ok(ComputedTx {
                    txindex: base_txindex + TxIndex::from(index),
                    tx,
                    txid,
                    txid_prefix,
                    prev_txindex_opt,
                })
            })
            .collect()
    }

    /// Check for TXID collisions (only for known duplicate TXIDs).
    pub fn check_txid_collisions(&self, txs: &[ComputedTx]) -> Result<()> {
        if likely(!self.check_collisions) {
            return Ok(());
        }

        let mut txindex_to_txid_iter = self.vecs.tx.txindex_to_txid.into_iter();
        for ct in txs.iter() {
            let Some(prev_txindex) = ct.prev_txindex_opt else {
                continue;
            };

            // In case if we start at an already parsed height
            if ct.txindex == prev_txindex {
                continue;
            }

            let len = self.vecs.tx.txindex_to_txid.len();
            let prev_txid = txindex_to_txid_iter
                .get(prev_txindex)
                .ok_or(Error::Internal("Missing txid for txindex"))
                .inspect_err(|_| {
                    dbg!(ct.txindex, len);
                })?;

            let is_dup = DUPLICATE_TXIDS.contains(&prev_txid);

            if !is_dup {
                dbg!(self.height, ct.txindex, prev_txid, prev_txindex);
                return Err(Error::Internal("Unexpected TXID collision"));
            }
        }

        Ok(())
    }

    /// Store transaction metadata.
    pub fn store_transaction_metadata(&mut self, txs: Vec<ComputedTx>) -> Result<()> {
        let height = self.height;

        for ct in txs {
            if ct.prev_txindex_opt.is_none() {
                self.stores.txidprefix_to_txindex.insert_if_needed(
                    ct.txid_prefix,
                    ct.txindex,
                    height,
                );
            }

            self.vecs
                .tx
                .txindex_to_height
                .checked_push(ct.txindex, height)?;
            self.vecs
                .tx
                .txindex_to_txversion
                .checked_push(ct.txindex, ct.tx.version.into())?;
            self.vecs
                .tx
                .txindex_to_txid
                .checked_push(ct.txindex, ct.txid)?;
            self.vecs
                .tx
                .txindex_to_rawlocktime
                .checked_push(ct.txindex, ct.tx.lock_time.into())?;
            self.vecs
                .tx
                .txindex_to_base_size
                .checked_push(ct.txindex, ct.tx.base_size().into())?;
            self.vecs
                .tx
                .txindex_to_total_size
                .checked_push(ct.txindex, ct.tx.total_size().into())?;
            self.vecs
                .tx
                .txindex_to_is_explicitly_rbf
                .checked_push(ct.txindex, StoredBool::from(ct.tx.is_explicitly_rbf()))?;
        }

        Ok(())
    }
}
