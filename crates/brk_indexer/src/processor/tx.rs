use brk_error::{Error, Result};
use brk_store::Store;
use brk_types::{Height, StoredBool, TxIndex, Txid, TxidPrefix};
use rayon::prelude::*;
use tracing::error;
use vecdb::{AnyVec, WritableVec, likely};

use crate::TxMetadataVecs;
use crate::constants::DUPLICATE_TXIDS;

use super::{BlockProcessor, ComputedTx};

impl<'a> BlockProcessor<'a> {
    pub fn compute_txids(&self) -> Result<Vec<ComputedTx<'a>>> {
        let will_check_collisions = self.check_collisions;
        let base_tx_index = self.indexes.tx_index;

        self.block
            .txdata
            .par_iter()
            .enumerate()
            .map(|(index, tx)| {
                let (btc_txid, base_size, total_size) = self.block.compute_tx_id_and_sizes(index);
                let txid = Txid::from(btc_txid);
                let txid_prefix = TxidPrefix::from(&txid);

                let prev_tx_index_opt = if will_check_collisions {
                    self.stores
                        .txid_prefix_to_tx_index
                        .get(&txid_prefix)?
                        .map(|v| *v)
                } else {
                    None
                };

                Ok(ComputedTx {
                    tx_index: base_tx_index + TxIndex::from(index),
                    tx,
                    txid,
                    txid_prefix,
                    prev_tx_index_opt,
                    base_size,
                    total_size,
                })
            })
            .collect()
    }

    /// Only for known duplicate TXIDs (BIP-30).
    pub fn check_txid_collisions(&self, txs: &[ComputedTx]) -> Result<()> {
        if likely(!self.check_collisions) {
            return Ok(());
        }

        for ct in txs.iter() {
            let Some(prev_tx_index) = ct.prev_tx_index_opt else {
                continue;
            };

            if ct.tx_index == prev_tx_index {
                continue;
            }

            let len = self.vecs.transactions.txid.len();
            let prev_txid = self
                .vecs
                .transactions
                .txid
                .get_pushed_or_read(prev_tx_index, &self.readers.txid)
                .ok_or(Error::Internal("Missing txid for tx_index"))
                .inspect_err(|_| {
                    error!(tx_index = ?ct.tx_index, len, "Missing txid for tx_index");
                })?;

            let is_dup = DUPLICATE_TXIDS.contains(&prev_txid);

            if !is_dup {
                error!(
                    height = ?self.height, tx_index = ?ct.tx_index,
                    ?prev_txid, ?prev_tx_index,
                    "Unexpected TXID collision"
                );
                return Err(Error::Internal("Unexpected TXID collision"));
            }
        }

        Ok(())
    }
}

pub(super) fn store_tx_metadata(
    height: Height,
    txs: Vec<ComputedTx>,
    store: &mut Store<TxidPrefix, TxIndex>,
    md: &mut TxMetadataVecs<'_>,
) -> Result<()> {
    for ct in txs {
        if ct.prev_tx_index_opt.is_none() {
            store.insert(ct.txid_prefix, ct.tx_index);
        }
        md.height.checked_push(ct.tx_index, height)?;
        md.tx_version
            .checked_push(ct.tx_index, ct.tx.version.into())?;
        md.txid.checked_push(ct.tx_index, ct.txid)?;
        md.raw_locktime
            .checked_push(ct.tx_index, ct.tx.lock_time.into())?;
        md.base_size
            .checked_push(ct.tx_index, ct.base_size.into())?;
        md.total_size
            .checked_push(ct.tx_index, ct.total_size.into())?;
        md.is_explicitly_rbf
            .checked_push(ct.tx_index, StoredBool::from(ct.tx.is_explicitly_rbf()))?;
    }
    Ok(())
}
