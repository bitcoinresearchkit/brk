use std::{ops::Sub, thread};

use biter::{
    bitcoin::{hashes::Hash, BlockHash, Txid},
    bitcoincore_rpc::Client,
};
pub use blockhash_prefix_to_height::*;
use color_eyre::eyre::ContextCompat;
use fjall::Slice;

mod blockhash_prefix_to_height;
mod height_to_blockhash;
mod height_to_txindex;
mod txid_prefix_to_txindex;
mod txindex_to_txid;
mod txoutindex_to_amount;

pub use height_to_blockhash::*;
pub use height_to_txindex::*;
pub use txid_prefix_to_txindex::*;
pub use txindex_to_txid::*;
pub use txoutindex_to_amount::*;

use crate::structs::{Height, Txindex, Txoutindex};

pub struct Databases {
    pub blockhash_prefix_to_height: BlockhashPrefixToHeight,
    pub height_to_blockhash: HeightToBlockhash,
    pub height_to_first_txindex: HeightToTxindex,
    pub height_to_last_txindex: HeightToTxindex,
    pub txid_prefix_to_txindex: TxidPrefixToTxindex,
    pub txindex_to_txid: TxindexToTxid,
    pub txoutindex_to_amount: TxoutindexToAmount,
}

const UNSAFE_BLOCKS: usize = 100;

impl Databases {
    pub fn import() -> color_eyre::Result<Self> {
        thread::scope(|scope| {
            let blockhash_prefix_to_height_handle = scope.spawn(BlockhashPrefixToHeight::import);
            let height_to_blockhash_handle = scope.spawn(HeightToBlockhash::import);
            let height_to_first_txindex_handle =
                scope.spawn(|| HeightToTxindex::import(HeightToTxindexPosition::First));
            let height_to_last_txindex_handle =
                scope.spawn(|| HeightToTxindex::import(HeightToTxindexPosition::Last));
            let txid_prefix_to_txindex_handle = scope.spawn(TxidPrefixToTxindex::import);
            let txindex_to_txid_handle = scope.spawn(TxindexToTxid::import);
            let txoutindex_to_amount_handle = scope.spawn(TxoutindexToAmount::import);

            Ok(Self {
                blockhash_prefix_to_height: blockhash_prefix_to_height_handle.join().unwrap()?,
                height_to_blockhash: height_to_blockhash_handle.join().unwrap()?,
                height_to_first_txindex: height_to_first_txindex_handle.join().unwrap()?,
                height_to_last_txindex: height_to_last_txindex_handle.join().unwrap()?,
                txid_prefix_to_txindex: txid_prefix_to_txindex_handle.join().unwrap()?,
                txindex_to_txid: txindex_to_txid_handle.join().unwrap()?,
                txoutindex_to_amount: txoutindex_to_amount_handle.join().unwrap()?,
            })
        })
    }

    pub fn export(&mut self, height: Height) -> color_eyre::Result<()> {
        thread::scope(|scope| {
            scope.spawn(|| self.blockhash_prefix_to_height.export(height).unwrap());
            scope.spawn(|| self.height_to_blockhash.export(height).unwrap());
            scope.spawn(|| self.height_to_first_txindex.export(height).unwrap());
            scope.spawn(|| self.height_to_last_txindex.export(height).unwrap());
            scope.spawn(|| self.txid_prefix_to_txindex.export(height).unwrap());
            scope.spawn(|| self.txindex_to_txid.export(height).unwrap());
            scope.spawn(|| self.txoutindex_to_amount.export(height).unwrap());
        });
        Ok(())
    }

    pub fn start_height(&self, rpc: &Client) -> color_eyre::Result<Height> {
        let safe_height = Height::try_from(rpc)?.sub(UNSAFE_BLOCKS);
        Ok(self
            .min_height()
            .map(|h| h.sub(UNSAFE_BLOCKS))
            .unwrap_or_default()
            .min(safe_height))
    }

    fn min_height(&self) -> Option<Height> {
        [
            self.blockhash_prefix_to_height.height(),
            self.height_to_blockhash.height(),
            self.height_to_first_txindex.height(),
            self.height_to_last_txindex.height(),
            self.txid_prefix_to_txindex.height(),
            self.txindex_to_txid.height(),
            self.txoutindex_to_amount.height(),
        ]
        .into_iter()
        .map(ToOwned::to_owned)
        .min()
        .flatten()
    }

    pub fn has_different_blockhash(
        &self,
        height: Height,
        blockhash: &BlockHash,
    ) -> fjall::Result<bool> {
        Ok(self
            .height_to_blockhash
            .get(height)?
            .is_some_and(|saved_blockhash| blockhash != &saved_blockhash))
    }

    pub fn erase_from(&mut self, height: Height) -> color_eyre::Result<()> {
        let mut txindex = None;

        self.height_to_blockhash
            .range(Slice::from(height)..)
            .try_for_each(|slice| -> color_eyre::Result<()> {
                let (slice_height, slice_blockhash) = slice?;
                let height = Height::from(slice_height);
                let blockhash = BlockHash::from_slice(&slice_blockhash)?;

                self.height_to_blockhash.remove(height);
                self.blockhash_prefix_to_height.remove(&blockhash);
                if txindex.is_none() {
                    txindex.replace(
                        self.height_to_first_txindex
                            .get(height)?
                            .context("for height to have first txindex")?,
                    );
                }
                self.height_to_first_txindex.remove(height);
                self.height_to_last_txindex.remove(height);

                Ok(())
            })?;

        let txindex = txindex.context("txindex to not be none by now")?;

        self.txindex_to_txid
            .range(Slice::from(txindex)..)
            .try_for_each(|slice| -> color_eyre::Result<()> {
                let (slice_txindex, slice_txid) = slice?;
                let txindex = Txindex::from(slice_txindex);
                let txid = Txid::from_slice(&slice_txid)?;

                self.txindex_to_txid.remove(txindex);
                self.txid_prefix_to_txindex.remove(&txid);

                Ok(())
            })?;

        let txoutindex = Txoutindex::from(txindex);

        self.txoutindex_to_amount
            .range(Slice::from(txoutindex)..)
            .try_for_each(|slice| -> color_eyre::Result<()> {
                let (slice_txoutindex, _) = slice?;
                let txoutindex = Txoutindex::from(slice_txoutindex);

                self.txoutindex_to_amount.remove(txoutindex);

                Ok(())
            })?;

        Ok(())
    }
}
