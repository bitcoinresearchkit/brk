use std::{path::Path, thread};

use storable_vec::Version;

use crate::{AddressHash, Addressindex, BlockHashPrefix, Height, TxidPrefix, Txindex};

mod base;
mod meta;
// mod version;

pub use base::*;
pub use meta::*;

pub struct Fjalls {
    pub addresshash_to_addressindex: Store<AddressHash, Addressindex>,
    pub blockhash_prefix_to_height: Store<BlockHashPrefix, Height>,
    pub txid_prefix_to_txindex: Store<TxidPrefix, Txindex>,
}

impl Fjalls {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        let addresshash_to_addressindex = Store::import(&path.join("addresshash_to_addressindex"), Version::from(1))?;
        let blockhash_prefix_to_height = Store::import(&path.join("blockhash_prefix_to_height"), Version::from(1))?;
        let txid_prefix_to_txindex = Store::import(&path.join("txid_prefix_to_txindex"), Version::from(1))?;

        Ok(Self {
            addresshash_to_addressindex,
            blockhash_prefix_to_height,
            txid_prefix_to_txindex,
        })
    }

    // pub fn rollback_from(
    //     &mut self,
    //     _wtx: &mut WriteTransaction,
    //     _height: Height,
    //     _exit: &Exit,
    // ) -> color_eyre::Result<()> {
    //     panic!();
    // let mut txindex = None;

    // wtx.range(self.height_to_blockhash.data(), Slice::from(height)..)
    //     .try_for_each(|slice| -> color_eyre::Result<()> {
    //         let (height_slice, slice_blockhash) = slice?;
    //         let blockhash = BlockHash::from_slice(&slice_blockhash)?;

    //         wtx.remove(self.height_to_blockhash.data(), height_slice);

    //         wtx.remove(self.blockhash_prefix_to_height.data(), blockhash.prefix());

    //         if txindex.is_none() {
    //             txindex.replace(
    //                 wtx.get(self.height_to_first_txindex.data(), height_slice)?
    //                     .context("for height to have first txindex")?,
    //             );
    //         }
    //         wtx.remove(self.height_to_first_txindex.data(), height_slice);
    //         wtx.remove(self.height_to_last_txindex.data(), height_slice);

    //         Ok(())
    //     })?;

    // let txindex = txindex.context("txindex to not be none by now")?;

    // wtx.range(self.txindex_to_txid.data(), Slice::from(txindex)..)
    //     .try_for_each(|slice| -> color_eyre::Result<()> {
    //         let (slice_txindex, slice_txid) = slice?;
    //         let txindex = Txindex::from(slice_txindex);
    //         let txid = Txid::from_slice(&slice_txid)?;

    //         wtx.remove(self.txindex_to_txid.data(), Slice::from(txindex));
    //         wtx.remove(self.txindex_to_height.data(), Slice::from(txindex));
    //         wtx.remove(self.txid_prefix_to_txindex.data(), txid.prefix());

    //         Ok(())
    //     })?;

    // let txoutindex = Txoutindex::from(txindex);

    // let mut addressindexes = BTreeSet::new();

    // wtx.range(self.txoutindex_to_amount.data(), Slice::from(txoutindex)..)
    //     .try_for_each(|slice| -> color_eyre::Result<()> {
    //         let (txoutindex_slice, _) = slice?;

    //         wtx.remove(self.txoutindex_to_amount.data(), txoutindex_slice);

    //         if let Some(addressindex_slice) =
    //             wtx.get(self.txoutindex_to_addressindex.data(), txoutindex_slice)?
    //         {
    //             wtx.remove(self.txoutindex_to_addressindex.data(), txoutindex_slice);

    //             let addressindex = Addressindex::from(addressindex_slice);
    //             addressindexes.insert(addressindex);

    //             let txoutindex = Txoutindex::from(txoutindex_slice);
    //             let addresstxoutindex = Addresstxoutindex::from((addressindex, txoutindex));

    //             wtx.remove(
    //                 self.addressindex_to_txoutindexes.data(),
    //                 Slice::from(addresstxoutindex),
    //             );
    //         }

    //         Ok(())
    //     })?;

    // addressindexes
    // .into_iter()
    // .filter(|addressindex| {
    //     let is_empty = wtx
    //         .prefix(
    //             self.addressindex_to_txoutindexes.data(),
    //             Slice::from(*addressindex),
    //         )
    //         .next()
    //         .is_none();
    //     is_empty
    // })
    // .try_for_each(|addressindex| -> color_eyre::Result<()> {
    //     let addressindex_slice = Slice::from(addressindex);

    //     let addressbytes = Addressbytes::from(
    //         wtx.get(
    //             self.addressindex_to_addressbytes.data(),
    //             &addressindex_slice,
    //         )?
    //         .context("addressindex_to_address to have value")?,
    //     );
    //     wtx.remove(
    //         self.addressbytes_prefix_to_addressindex.data(),
    //         addressbytes.prefix(),
    //     );
    //     wtx.remove(
    //         self.addressindex_to_addressbytes.data(),
    //         &addressindex_slice,
    //     );
    //     wtx.remove(self.addressindex_to_addresstype.data(), &addressindex_slice);

    //     Ok(())
    // })?;
    //

    // todo!("clear addresstxoutindexes_out")
    // todo!("clear addresstxoutindexes_in")
    // todo!("clear zero_txoutindexes")
    // todo!("clear txindexvout_to_txoutindex")

    // Ok(())
    // }

    pub fn min_height(&self) -> Option<Height> {
        [
            self.addresshash_to_addressindex.height(),
            self.blockhash_prefix_to_height.height(),
            self.txid_prefix_to_txindex.height(),
        ]
        .into_iter()
        .min()
        .flatten()
        .cloned()
    }

    pub fn commit(&mut self, height: Height) -> fjall::Result<()> {
        thread::scope(|scope| {
            let addresshash_to_addressindex_commit_handle =
                scope.spawn(|| self.addresshash_to_addressindex.commit(height));
            let blockhash_prefix_to_height_commit_handle =
                scope.spawn(|| self.blockhash_prefix_to_height.commit(height));
            let txid_prefix_to_txindex_commit_handle = scope.spawn(|| self.txid_prefix_to_txindex.commit(height));

            addresshash_to_addressindex_commit_handle.join().unwrap()?;
            blockhash_prefix_to_height_commit_handle.join().unwrap()?;
            txid_prefix_to_txindex_commit_handle.join().unwrap()?;

            Ok(())
        })
    }

    // pub fn udpate_meta(&self, wtx: &mut WriteTransaction, height: Height) {
    //     self.addressbytes_prefix_to_addressindex.update_meta(wtx, height);
    //     self.blockhash_prefix_to_height.update_meta(wtx, height);
    //     self.txid_prefix_to_txindex.update_meta(wtx, height);
    // }

    // pub fn export(self, height: Height) -> Result<(), snkrj::Error> {
    //     thread::scope(|scope| {
    //         vec![
    //             scope.spawn(|| self.addressbytes_prefix_to_addressindex.export(height)),
    //             scope.spawn(|| self.blockhash_prefix_to_height.export(height)),
    //             scope.spawn(|| self.txid_prefix_to_txindex.export(height)),
    //         ]
    //         .into_iter()
    //         .try_for_each(|handle| -> Result<(), snkrj::Error> { handle.join().unwrap() })
    //     })
    // }
}
