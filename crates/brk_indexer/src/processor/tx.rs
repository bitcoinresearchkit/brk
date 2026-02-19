use brk_error::{Error, Result};
use brk_store::Store;
use brk_types::{Height, StoredBool, TxIndex, Txid, TxidPrefix};
use rayon::prelude::*;
use vecdb::{AnyVec, WritableVec, likely};

use crate::TxMetadataVecs;
use crate::constants::DUPLICATE_TXIDS;

use super::{BlockProcessor, ComputedTx};

impl<'a> BlockProcessor<'a> {
    pub fn compute_txids(&self) -> Result<Vec<ComputedTx<'a>>> {
        let will_check_collisions = self.check_collisions;
        let base_txindex = self.indexes.txindex;

        self.block
            .txdata
            .par_iter()
            .enumerate()
            .map(|(index, tx)| {
                let (btc_txid, base_size, total_size) =
                    self.block.compute_tx_id_and_sizes(index);
                let txid = Txid::from(btc_txid);
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
            let Some(prev_txindex) = ct.prev_txindex_opt else {
                continue;
            };

            if ct.txindex == prev_txindex {
                continue;
            }

            let len = self.vecs.transactions.txid.len();
            let prev_txid = self
                .vecs
                .transactions
                .txid
                .get_pushed_or_read(prev_txindex, &self.readers.txid)
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
}

pub(super) fn store_tx_metadata(
    height: Height,
    txs: Vec<ComputedTx>,
    store: &mut Store<TxidPrefix, TxIndex>,
    md: &mut TxMetadataVecs<'_>,
) -> Result<()> {
    for ct in txs {
        if ct.prev_txindex_opt.is_none() {
            store.insert(ct.txid_prefix, ct.txindex);
        }
        md.height.checked_push(ct.txindex, height)?;
        md.txversion
            .checked_push(ct.txindex, ct.tx.version.into())?;
        md.txid.checked_push(ct.txindex, ct.txid)?;
        md.rawlocktime
            .checked_push(ct.txindex, ct.tx.lock_time.into())?;
        md.base_size
            .checked_push(ct.txindex, ct.base_size.into())?;
        md.total_size
            .checked_push(ct.txindex, ct.total_size.into())?;
        md.is_explicitly_rbf
            .checked_push(ct.txindex, StoredBool::from(ct.tx.is_explicitly_rbf()))?;
    }
    Ok(())
}
