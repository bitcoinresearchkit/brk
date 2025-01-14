use std::ops::Sub;

use fjall::{Slice, TransactionalKeyspace, WriteTransaction};

use crate::structs::{Exit, Height, Partition, Version};

pub struct Partitions {
    pub addressbytes_prefix_to_addressindex: Partition,
    pub addressindextxoutindex_in: Partition,
    pub addressindextxoutindex_out: Partition,
    pub blockhash_prefix_to_height: Partition,
    pub txid_prefix_to_txindex: Partition,
    pub txindexvout_to_txoutindex: Partition,
}

const UNSAFE_BLOCKS: usize = 100;

impl Partitions {
    pub fn import(keyspace: &TransactionalKeyspace, exit: &Exit) -> color_eyre::Result<Self> {
        Ok(Self {
            addressbytes_prefix_to_addressindex: Partition::import(
                keyspace,
                "addressbytes_prefix_to_addressindex",
                Version::from(1),
                exit,
            )?,
            addressindextxoutindex_in: Partition::import(keyspace, "addresstxoutindexes_in", Version::from(1), exit)?,
            addressindextxoutindex_out: Partition::import(keyspace, "addresstxoutindexes_out", Version::from(1), exit)?,
            blockhash_prefix_to_height: Partition::import(
                keyspace,
                "blockhash_prefix_to_height",
                Version::from(1),
                exit,
            )?,
            txid_prefix_to_txindex: Partition::import(keyspace, "txid_prefix_to_txindex", Version::from(1), exit)?,
            txindexvout_to_txoutindex: Partition::import(
                keyspace,
                "txindexvout_to_txoutindex",
                Version::from(1),
                exit,
            )?,
        })
    }

    pub fn udpate_meta(&self, wtx: &mut WriteTransaction, height: Height) {
        self.to_vec().into_iter().for_each(|part| {
            let meta = part.meta();
            wtx.insert(meta, Partition::VERSION, Slice::from(part.version()));
            wtx.insert(meta, Partition::HEIGHT, height.to_be_bytes());
        });
    }

    pub fn start_height(&self) -> Height {
        self.min_height()
            .map(|height| height.sub(UNSAFE_BLOCKS))
            .unwrap_or_default()
    }

    fn min_height(&self) -> Option<Height> {
        self.to_vec()
            .into_iter()
            .map(|part| part.height())
            .map(ToOwned::to_owned)
            .min()
            .flatten()
    }

    pub fn rollback_from(
        &mut self,
        _wtx: &mut WriteTransaction,
        _height: Height,
        _exit: &Exit,
    ) -> color_eyre::Result<()> {
        panic!();
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
    }

    fn to_vec(&self) -> Vec<&Partition> {
        vec![
            &self.addressbytes_prefix_to_addressindex,
            &self.addressindextxoutindex_in,
            &self.addressindextxoutindex_out,
            &self.blockhash_prefix_to_height,
            &self.txid_prefix_to_txindex,
            &self.txindexvout_to_txoutindex,
        ]
    }
}
