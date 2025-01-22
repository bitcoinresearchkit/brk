use std::{path::Path, thread};

use crate::structs::Height;

use super::{
    AddressbytesPrefix, Addressindex, Addressindextxoutindex, BlockHashPrefix, Store, TxidPrefix, Txindex, Txindexvout,
    Txoutindex, Version,
};

pub struct Stores {
    pub addressbytes_prefix_to_addressindex: Store<AddressbytesPrefix, Addressindex>,
    pub addressindextxoutindex_in: Store<Addressindextxoutindex, ()>,
    pub addressindextxoutindex_out: Store<Addressindextxoutindex, ()>,
    pub blockhash_prefix_to_height: Store<BlockHashPrefix, Height>,
    pub txid_prefix_to_txindex: Store<TxidPrefix, Txindex>,
    pub txindexvout_to_txoutindex: Store<Txindexvout, Txoutindex>,
}

impl Stores {
    pub fn open(path: &Path) -> color_eyre::Result<Self> {
        Ok(Self {
            addressbytes_prefix_to_addressindex: Store::open(
                &path.join("addressbytes_prefix_to_addressindex"),
                Version::from(1),
            )?,
            addressindextxoutindex_in: Store::open(&path.join("addresstxoutindexes_in"), Version::from(1))?,
            addressindextxoutindex_out: Store::open(&path.join("addresstxoutindexes_out"), Version::from(1))?,
            blockhash_prefix_to_height: Store::open(&path.join("blockhash_prefix_to_height"), Version::from(1))?,
            txid_prefix_to_txindex: Store::open(&path.join("txid_prefix_to_txindex"), Version::from(1))?,
            txindexvout_to_txoutindex: Store::open(&path.join("txindexvout_to_txoutindex"), Version::from(1))?,
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
            self.addressbytes_prefix_to_addressindex.height(),
            self.addressindextxoutindex_in.height(),
            self.addressindextxoutindex_out.height(),
            self.blockhash_prefix_to_height.height(),
            self.txid_prefix_to_txindex.height(),
            self.txindexvout_to_txoutindex.height(),
        ]
        .into_iter()
        .min()
        .flatten()
        .cloned()
    }

    pub fn export(self, height: Height) {
        thread::scope(|scope| {
            scope.spawn(|| self.addressbytes_prefix_to_addressindex.export(height));
            scope.spawn(|| self.addressindextxoutindex_in.export(height));
            scope.spawn(|| self.addressindextxoutindex_out.export(height));
            scope.spawn(|| self.blockhash_prefix_to_height.export(height));
            scope.spawn(|| self.txid_prefix_to_txindex.export(height));
            scope.spawn(|| self.txindexvout_to_txoutindex.export(height));
        });
    }
}
