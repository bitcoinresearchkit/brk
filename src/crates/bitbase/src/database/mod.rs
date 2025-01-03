use std::{collections::BTreeSet, ops::Sub, thread};

use biter::bitcoin::{hashes::Hash, BlockHash, Txid};
use color_eyre::eyre::ContextCompat;
use fjall::Slice;

mod addressbytes_prefix_to_addressindex;
mod addressindex_to_addressbytes;
mod addressindex_to_addresstype;
mod addressindex_to_txoutindexes;
mod blockhash_prefix_to_height;
mod height_to_blockhash;
mod height_to_txindex;
mod txid_prefix_to_txindex;
mod txindex_to_height;
mod txindex_to_txid;
mod txoutindex_to_addressindex;
mod txoutindex_to_amount;

pub use addressbytes_prefix_to_addressindex::*;
pub use addressindex_to_addressbytes::*;
pub use addressindex_to_addresstype::*;
pub use addressindex_to_txoutindexes::*;
pub use blockhash_prefix_to_height::*;
pub use height_to_blockhash::*;
pub use height_to_txindex::*;
pub use txid_prefix_to_txindex::*;
pub use txindex_to_height::*;
pub use txindex_to_txid::*;
pub use txoutindex_to_addressindex::*;
pub use txoutindex_to_amount::*;

use crate::structs::{Addressindex, Exit, Height, Txindex, Txoutindex};

pub struct Database {
    pub addressbytes_prefix_to_addressindex: AddressbytesPrefixToAddressindex,
    pub addressindex_to_addressbytes: AddressindexToAddressbytes,
    pub addressindex_to_addresstype: AddressindexToAddresstype,
    pub addressindex_to_txoutindexes: AddressindexToTxoutindexes,
    pub blockhash_prefix_to_height: BlockhashPrefixToHeight,
    pub height_to_blockhash: HeightToBlockhash,
    pub height_to_first_txindex: HeightToTxindex,
    pub height_to_last_txindex: HeightToTxindex,
    pub txid_prefix_to_txindex: TxidPrefixToTxindex,
    pub txindex_to_txid: TxindexToTxid,
    pub txindex_to_height: TxindexToHeight,
    pub txoutindex_to_addressindex: TxoutindexToAddressindex,
    pub txoutindex_to_amount: TxoutindexToAmount,
}

const UNSAFE_BLOCKS: usize = 100;

impl Database {
    pub fn import() -> color_eyre::Result<Self> {
        thread::scope(|scope| {
            let addressbytes_prefix_to_addressindex_handle =
                scope.spawn(AddressbytesPrefixToAddressindex::import);
            let addressindex_to_addressbytes_handle =
                scope.spawn(AddressindexToAddressbytes::import);
            let addressindex_to_addresstype_handle = scope.spawn(AddressindexToAddresstype::import);
            let addressindex_to_txoutindexes_handle =
                scope.spawn(AddressindexToTxoutindexes::import);
            let blockhash_prefix_to_height_handle = scope.spawn(BlockhashPrefixToHeight::import);
            let height_to_blockhash_handle = scope.spawn(HeightToBlockhash::import);
            let height_to_first_txindex_handle =
                scope.spawn(|| HeightToTxindex::import(HeightToTxindexPosition::First));
            let height_to_last_txindex_handle =
                scope.spawn(|| HeightToTxindex::import(HeightToTxindexPosition::Last));
            let txid_prefix_to_txindex_handle = scope.spawn(TxidPrefixToTxindex::import);
            let txindex_to_height_handle = scope.spawn(TxindexToHeight::import);
            let txindex_to_txid_handle = scope.spawn(TxindexToTxid::import);
            let txoutindex_to_addressindex_handle = scope.spawn(TxoutindexToAddressindex::import);
            let txoutindex_to_amount_handle = scope.spawn(TxoutindexToAmount::import);

            Ok(Self {
                addressbytes_prefix_to_addressindex: addressbytes_prefix_to_addressindex_handle
                    .join()
                    .unwrap()?,
                addressindex_to_addressbytes: addressindex_to_addressbytes_handle
                    .join()
                    .unwrap()?,
                addressindex_to_addresstype: addressindex_to_addresstype_handle.join().unwrap()?,
                addressindex_to_txoutindexes: addressindex_to_txoutindexes_handle
                    .join()
                    .unwrap()?,
                blockhash_prefix_to_height: blockhash_prefix_to_height_handle.join().unwrap()?,
                height_to_blockhash: height_to_blockhash_handle.join().unwrap()?,
                height_to_first_txindex: height_to_first_txindex_handle.join().unwrap()?,
                height_to_last_txindex: height_to_last_txindex_handle.join().unwrap()?,
                txid_prefix_to_txindex: txid_prefix_to_txindex_handle.join().unwrap()?,
                txindex_to_height: txindex_to_height_handle.join().unwrap()?,
                txindex_to_txid: txindex_to_txid_handle.join().unwrap()?,
                txoutindex_to_addressindex: txoutindex_to_addressindex_handle.join().unwrap()?,
                txoutindex_to_amount: txoutindex_to_amount_handle.join().unwrap()?,
            })
        })
    }

    pub fn export(&mut self, height: Height) -> color_eyre::Result<()> {
        thread::scope(|scope| {
            scope.spawn(|| {
                self.addressbytes_prefix_to_addressindex
                    .export(height)
                    .unwrap()
            });
            scope.spawn(|| self.addressindex_to_addressbytes.export(height).unwrap());
            scope.spawn(|| self.addressindex_to_addresstype.export(height).unwrap());
            scope.spawn(|| self.addressindex_to_txoutindexes.export(height).unwrap());
            scope.spawn(|| self.blockhash_prefix_to_height.export(height).unwrap());
            scope.spawn(|| self.height_to_blockhash.export(height).unwrap());
            scope.spawn(|| self.height_to_first_txindex.export(height).unwrap());
            scope.spawn(|| self.height_to_last_txindex.export(height).unwrap());
            scope.spawn(|| self.txid_prefix_to_txindex.export(height).unwrap());
            scope.spawn(|| self.txindex_to_height.export(height).unwrap());
            scope.spawn(|| self.txindex_to_txid.export(height).unwrap());
            scope.spawn(|| self.txoutindex_to_addressindex.export(height).unwrap());
            scope.spawn(|| self.txoutindex_to_amount.export(height).unwrap());
        });
        Ok(())
    }

    pub fn start_height(&self) -> Height {
        self.min_height()
            .map(|h| h.sub(UNSAFE_BLOCKS))
            .unwrap_or_default()
    }

    fn min_height(&self) -> Option<Height> {
        [
            self.addressbytes_prefix_to_addressindex.height(),
            self.addressindex_to_addressbytes.height(),
            self.addressindex_to_addresstype.height(),
            self.addressindex_to_txoutindexes.height(),
            self.blockhash_prefix_to_height.height(),
            self.height_to_blockhash.height(),
            self.height_to_first_txindex.height(),
            self.height_to_last_txindex.height(),
            self.txid_prefix_to_txindex.height(),
            self.txindex_to_height.height(),
            self.txindex_to_txid.height(),
            self.txoutindex_to_addressindex.height(),
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

    pub fn rollback_from(&mut self, height: Height, exit: &Exit) -> color_eyre::Result<()> {
        exit.block();

        self.export(height)?;

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
                self.txindex_to_height.remove(txindex);
                self.txid_prefix_to_txindex.remove(&txid);

                Ok(())
            })?;

        let txoutindex = Txoutindex::from(txindex);

        let mut addressindexes = BTreeSet::new();

        self.txoutindex_to_amount
            .range(Slice::from(txoutindex)..)
            .try_for_each(|slice| -> color_eyre::Result<()> {
                let (slice_txoutindex, _) = slice?;
                let txoutindex = Txoutindex::from(slice_txoutindex);

                self.txoutindex_to_amount.remove(txoutindex);

                if let Some(addressindex_slice) =
                    self.txoutindex_to_addressindex.get(txoutindex.into())?
                {
                    self.txoutindex_to_addressindex.remove(txoutindex);

                    let addressindex = Addressindex::from(addressindex_slice);
                    addressindexes.insert(addressindex);
                    self.addressindex_to_txoutindexes
                        .remove(addressindex, txoutindex);
                }

                Ok(())
            })?;

        self.export(height)?;

        addressindexes
            .into_iter()
            .filter(|addressindex| self.addressindex_to_txoutindexes.is_empty(*addressindex))
            .try_for_each(|addressindex| -> color_eyre::Result<()> {
                let addressbytes = self
                    .addressindex_to_addressbytes
                    .get(addressindex)?
                    .context("addressindex_to_address to have value")?;
                self.addressbytes_prefix_to_addressindex
                    .remove(&addressbytes);
                self.addressindex_to_addressbytes.remove(addressindex);
                self.addressindex_to_addresstype.remove(addressindex);

                Ok(())
            })?;

        self.export(height)?;

        exit.unblock();

        Ok(())
    }
}
