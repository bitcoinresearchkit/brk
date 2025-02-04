use std::{fs, io, path::Path};

use biter::bitcoin::{self, transaction, BlockHash, Txid, Weight};
use exit::Exit;
use rayon::prelude::*;
use storable_vec::{Version, CACHED_GETS};

use crate::structs::{
    Addressbytes, Addressindex, Addresstype, Addresstypeindex, Amount, Height, P2PK33AddressBytes, P2PK65AddressBytes,
    P2PKHAddressBytes, P2SHAddressBytes, P2TRAddressBytes, P2WPKHAddressBytes, P2WSHAddressBytes, Timestamp, Txindex,
    Txinindex, Txoutindex,
};

mod base;

pub use base::*;

pub struct StorableVecs<const MODE: u8> {
    pub addressindex_to_addresstype: StorableVec<Addressindex, Addresstype, MODE>,
    pub addressindex_to_addresstypeindex: StorableVec<Addressindex, Addresstypeindex, MODE>,
    pub addressindex_to_height: StorableVec<Addressindex, Height, MODE>,
    pub height_to_blockhash: StorableVec<Height, BlockHash, MODE>,
    pub height_to_difficulty: StorableVec<Height, f64, MODE>,
    pub height_to_first_addressindex: StorableVec<Height, Addressindex, MODE>,
    pub height_to_first_emptyindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_multisigindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_opreturnindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_pushonlyindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_txindex: StorableVec<Height, Txindex, MODE>,
    pub height_to_first_txinindex: StorableVec<Height, Txinindex, MODE>,
    pub height_to_first_txoutindex: StorableVec<Height, Txoutindex, MODE>,
    pub height_to_first_unknownindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_p2pk33index: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_p2pk65index: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_p2pkhindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_p2shindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_p2trindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_p2wpkhindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_first_p2wshindex: StorableVec<Height, Addresstypeindex, MODE>,
    pub height_to_size: StorableVec<Height, usize, MODE>,
    pub height_to_timestamp: StorableVec<Height, Timestamp, MODE>,
    pub height_to_weight: StorableVec<Height, Weight, MODE>,
    pub p2pk33index_to_p2pk33addressbytes: StorableVec<Addresstypeindex, P2PK33AddressBytes, MODE>,
    pub p2pk65index_to_p2pk65addressbytes: StorableVec<Addresstypeindex, P2PK65AddressBytes, MODE>,
    pub p2pkhindex_to_p2pkhaddressbytes: StorableVec<Addresstypeindex, P2PKHAddressBytes, MODE>,
    pub p2shindex_to_p2shaddressbytes: StorableVec<Addresstypeindex, P2SHAddressBytes, MODE>,
    pub p2trindex_to_p2traddressbytes: StorableVec<Addresstypeindex, P2TRAddressBytes, MODE>,
    pub p2wpkhindex_to_p2wpkhaddressbytes: StorableVec<Addresstypeindex, P2WPKHAddressBytes, MODE>,
    pub p2wshindex_to_p2wshaddressbytes: StorableVec<Addresstypeindex, P2WSHAddressBytes, MODE>,
    pub txindex_to_first_txinindex: StorableVec<Txindex, Txinindex, MODE>,
    pub txindex_to_first_txoutindex: StorableVec<Txindex, Txoutindex, MODE>,
    pub txindex_to_height: StorableVec<Txindex, Height, MODE>,
    pub txindex_to_locktime: StorableVec<Txindex, bitcoin::absolute::LockTime, MODE>,
    pub txindex_to_txid: StorableVec<Txindex, Txid, MODE>,
    pub txindex_to_txversion: StorableVec<Txindex, transaction::Version, MODE>,
    pub txinindex_to_txoutindex: StorableVec<Txinindex, Txoutindex, MODE>,
    pub txoutindex_to_addressindex: StorableVec<Txoutindex, Addressindex, MODE>,
    pub txoutindex_to_amount: StorableVec<Txoutindex, Amount, MODE>,
}

// const UNSAFE_BLOCKS: usize = 100;

impl<const MODE: u8> StorableVecs<MODE> {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            addressindex_to_addresstype: StorableVec::import(
                &path.join("addressindex_to_addresstype"),
                Version::from(1),
            )?,
            addressindex_to_addresstypeindex: StorableVec::import(
                &path.join("addressindex_to_addresstypeindex"),
                Version::from(1),
            )?,
            addressindex_to_height: StorableVec::import(&path.join("addressindex_to_height"), Version::from(1))?,
            height_to_blockhash: StorableVec::import(&path.join("height_to_blockhash"), Version::from(1))?,
            height_to_difficulty: StorableVec::import(&path.join("height_to_difficulty"), Version::from(1))?,
            height_to_first_addressindex: StorableVec::import(
                &path.join("height_to_first_addressindex"),
                Version::from(1),
            )?,
            height_to_first_emptyindex: StorableVec::import(
                &path.join("height_to_first_emptyindex"),
                Version::from(1),
            )?,
            height_to_first_multisigindex: StorableVec::import(
                &path.join("height_to_first_multisigindex"),
                Version::from(1),
            )?,
            height_to_first_opreturnindex: StorableVec::import(
                &path.join("height_to_first_opreturnindex"),
                Version::from(1),
            )?,
            height_to_first_pushonlyindex: StorableVec::import(
                &path.join("height_to_first_pushonlyindex"),
                Version::from(1),
            )?,
            height_to_first_txindex: StorableVec::import(&path.join("height_to_first_txindex"), Version::from(1))?,
            height_to_first_txinindex: StorableVec::import(&path.join("height_to_first_txinindex"), Version::from(1))?,
            height_to_first_txoutindex: StorableVec::import(
                &path.join("height_to_first_txoutindex"),
                Version::from(1),
            )?,
            height_to_first_unknownindex: StorableVec::import(
                &path.join("height_to_first_unkownindex"),
                Version::from(1),
            )?,
            height_to_first_p2pk33index: StorableVec::import(
                &path.join("height_to_first_p2pk33index"),
                Version::from(1),
            )?,
            height_to_first_p2pk65index: StorableVec::import(
                &path.join("height_to_first_p2pk65index"),
                Version::from(1),
            )?,
            height_to_first_p2pkhindex: StorableVec::import(
                &path.join("height_to_first_p2pkhindex"),
                Version::from(1),
            )?,
            height_to_first_p2shindex: StorableVec::import(&path.join("height_to_first_p2shindex"), Version::from(1))?,
            height_to_first_p2trindex: StorableVec::import(&path.join("height_to_first_p2trindex"), Version::from(1))?,
            height_to_first_p2wpkhindex: StorableVec::import(
                &path.join("height_to_first_p2wpkhindex"),
                Version::from(1),
            )?,
            height_to_first_p2wshindex: StorableVec::import(
                &path.join("height_to_first_p2wshindex"),
                Version::from(1),
            )?,
            height_to_size: StorableVec::import(&path.join("height_to_size"), Version::from(1))?,
            height_to_timestamp: StorableVec::import(&path.join("height_to_timestamp"), Version::from(1))?,
            height_to_weight: StorableVec::import(&path.join("height_to_weight"), Version::from(1))?,
            p2pk33index_to_p2pk33addressbytes: StorableVec::import(
                &path.join("p2pk33index_to_p2pk33addressbytes"),
                Version::from(1),
            )?,
            p2pk65index_to_p2pk65addressbytes: StorableVec::import(
                &path.join("p2pk65index_to_p2pk65addressbytes"),
                Version::from(1),
            )?,
            p2pkhindex_to_p2pkhaddressbytes: StorableVec::import(
                &path.join("p2pkhindex_to_p2pkhaddressbytes"),
                Version::from(1),
            )?,
            p2shindex_to_p2shaddressbytes: StorableVec::import(
                &path.join("p2shindex_to_p2shaddressbytes"),
                Version::from(1),
            )?,
            p2trindex_to_p2traddressbytes: StorableVec::import(
                &path.join("p2trindex_to_p2traddressbytes"),
                Version::from(1),
            )?,
            p2wpkhindex_to_p2wpkhaddressbytes: StorableVec::import(
                &path.join("p2wpkhindex_to_p2wpkhaddressbytes"),
                Version::from(1),
            )?,
            p2wshindex_to_p2wshaddressbytes: StorableVec::import(
                &path.join("p2wshindex_to_p2wshaddressbytes"),
                Version::from(1),
            )?,
            txindex_to_first_txinindex: StorableVec::import(
                &path.join("txindex_to_first_txinindex"),
                Version::from(1),
            )?,
            txindex_to_first_txoutindex: StorableVec::import(
                &path.join("txindex_to_first_txoutindex"),
                Version::from(1),
            )?,
            txindex_to_height: StorableVec::import(&path.join("txindex_to_height"), Version::from(1))?,
            txindex_to_locktime: StorableVec::import(&path.join("txindex_to_locktime"), Version::from(1))?,
            txindex_to_txid: StorableVec::import(&path.join("txindex_to_txid"), Version::from(1))?,
            txindex_to_txversion: StorableVec::import(&path.join("txindex_to_txversion"), Version::from(1))?,
            txinindex_to_txoutindex: StorableVec::import(&path.join("txinindex_to_txoutindex"), Version::from(1))?,
            txoutindex_to_addressindex: StorableVec::import(
                &path.join("txoutindex_to_addressindex"),
                Version::from(1),
            )?,
            txoutindex_to_amount: StorableVec::import(&path.join("txoutindex_to_amount"), Version::from(1))?,
        })
    }

    #[allow(unused)]
    pub fn rollback_from(&mut self, _height: Height, _exit: &Exit) -> color_eyre::Result<()> {
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

        // Ok(())
    }

    pub fn flush(&mut self, height: Height) -> io::Result<()> {
        self.as_mut_slice()
            .into_par_iter()
            .try_for_each(|vec| vec.flush(height))
    }

    pub fn min_height(&self) -> color_eyre::Result<Option<Height>> {
        Ok(self
            .as_slice()
            .into_iter()
            .map(|vec| vec.height().unwrap_or_default())
            .min())
    }

    pub fn as_slice(&self) -> [&dyn AnyStorableVec; 40] {
        [
            &self.addressindex_to_addresstype as &dyn AnyStorableVec,
            &self.addressindex_to_addresstypeindex,
            &self.addressindex_to_height,
            &self.height_to_blockhash,
            &self.height_to_difficulty,
            &self.height_to_first_addressindex,
            &self.height_to_first_emptyindex,
            &self.height_to_first_multisigindex,
            &self.height_to_first_opreturnindex,
            &self.height_to_first_pushonlyindex,
            &self.height_to_first_txindex,
            &self.height_to_first_txinindex,
            &self.height_to_first_txoutindex,
            &self.height_to_first_unknownindex,
            &self.height_to_first_p2pk33index,
            &self.height_to_first_p2pk65index,
            &self.height_to_first_p2pkhindex,
            &self.height_to_first_p2shindex,
            &self.height_to_first_p2trindex,
            &self.height_to_first_p2wpkhindex,
            &self.height_to_first_p2wshindex,
            &self.height_to_size,
            &self.height_to_timestamp,
            &self.height_to_weight,
            &self.p2pk33index_to_p2pk33addressbytes,
            &self.p2pk65index_to_p2pk65addressbytes,
            &self.p2pkhindex_to_p2pkhaddressbytes,
            &self.p2shindex_to_p2shaddressbytes,
            &self.p2trindex_to_p2traddressbytes,
            &self.p2wpkhindex_to_p2wpkhaddressbytes,
            &self.p2wshindex_to_p2wshaddressbytes,
            &self.txindex_to_first_txinindex,
            &self.txindex_to_first_txoutindex,
            &self.txindex_to_height,
            &self.txindex_to_locktime,
            &self.txindex_to_txid,
            &self.txindex_to_txversion,
            &self.txinindex_to_txoutindex,
            &self.txoutindex_to_addressindex,
            &self.txoutindex_to_amount,
        ]
    }

    pub fn as_mut_slice(&mut self) -> [&mut (dyn AnyStorableVec + Send + Sync); 40] {
        [
            &mut self.addressindex_to_addresstype as &mut (dyn AnyStorableVec + Send + Sync),
            &mut self.addressindex_to_addresstypeindex,
            &mut self.addressindex_to_height,
            &mut self.height_to_blockhash,
            &mut self.height_to_difficulty,
            &mut self.height_to_first_addressindex,
            &mut self.height_to_first_emptyindex,
            &mut self.height_to_first_multisigindex,
            &mut self.height_to_first_opreturnindex,
            &mut self.height_to_first_pushonlyindex,
            &mut self.height_to_first_txindex,
            &mut self.height_to_first_txinindex,
            &mut self.height_to_first_txoutindex,
            &mut self.height_to_first_unknownindex,
            &mut self.height_to_first_p2pk33index,
            &mut self.height_to_first_p2pk65index,
            &mut self.height_to_first_p2pkhindex,
            &mut self.height_to_first_p2shindex,
            &mut self.height_to_first_p2trindex,
            &mut self.height_to_first_p2wpkhindex,
            &mut self.height_to_first_p2wshindex,
            &mut self.height_to_size,
            &mut self.height_to_timestamp,
            &mut self.height_to_weight,
            &mut self.p2pk33index_to_p2pk33addressbytes,
            &mut self.p2pk65index_to_p2pk65addressbytes,
            &mut self.p2pkhindex_to_p2pkhaddressbytes,
            &mut self.p2shindex_to_p2shaddressbytes,
            &mut self.p2trindex_to_p2traddressbytes,
            &mut self.p2wpkhindex_to_p2wpkhaddressbytes,
            &mut self.p2wshindex_to_p2wshaddressbytes,
            &mut self.txindex_to_first_txinindex,
            &mut self.txindex_to_first_txoutindex,
            &mut self.txindex_to_height,
            &mut self.txindex_to_locktime,
            &mut self.txindex_to_txid,
            &mut self.txindex_to_txversion,
            &mut self.txinindex_to_txoutindex,
            &mut self.txoutindex_to_addressindex,
            &mut self.txoutindex_to_amount,
        ]
    }
}

impl StorableVecs<CACHED_GETS> {
    pub fn get_addressbytes(
        &self,
        addresstype: Addresstype,
        addresstypeindex: Addresstypeindex,
    ) -> storable_vec::Result<Option<Addressbytes>> {
        Ok(match addresstype {
            Addresstype::P2PK65 => self
                .p2pk65index_to_p2pk65addressbytes
                .get(addresstypeindex)?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2PK33 => self
                .p2pk33index_to_p2pk33addressbytes
                .get(addresstypeindex)?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2PKH => self
                .p2pkhindex_to_p2pkhaddressbytes
                .get(addresstypeindex)?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2SH => self
                .p2shindex_to_p2shaddressbytes
                .get(addresstypeindex)?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2WPKH => self
                .p2wpkhindex_to_p2wpkhaddressbytes
                .get(addresstypeindex)?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2WSH => self
                .p2wshindex_to_p2wshaddressbytes
                .get(addresstypeindex)?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2TR => self
                .p2trindex_to_p2traddressbytes
                .get(addresstypeindex)?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            _ => unreachable!(),
        })
    }

    pub fn push_addressbytes_if_needed(
        &mut self,
        index: Addresstypeindex,
        addressbytes: Addressbytes,
    ) -> storable_vec::Result<()> {
        match addressbytes {
            Addressbytes::P2PK65(bytes) => self.p2pk65index_to_p2pk65addressbytes.push_if_needed(index, bytes),
            Addressbytes::P2PK33(bytes) => self.p2pk33index_to_p2pk33addressbytes.push_if_needed(index, bytes),
            Addressbytes::P2PKH(bytes) => self.p2pkhindex_to_p2pkhaddressbytes.push_if_needed(index, bytes),
            Addressbytes::P2SH(bytes) => self.p2shindex_to_p2shaddressbytes.push_if_needed(index, bytes),
            Addressbytes::P2WPKH(bytes) => self.p2wpkhindex_to_p2wpkhaddressbytes.push_if_needed(index, bytes),
            Addressbytes::P2WSH(bytes) => self.p2wshindex_to_p2wshaddressbytes.push_if_needed(index, bytes),
            Addressbytes::P2TR(bytes) => self.p2trindex_to_p2traddressbytes.push_if_needed(index, bytes),
        }
    }
}
