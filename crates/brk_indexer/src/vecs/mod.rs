use std::{fs, io, path::Path};

use brk_core::{
    Addressbytes, Addressindex, Addresstype, Addresstypeindex, BlockHash, Emptyindex, Height,
    LockTime, Multisigindex, Opreturnindex, P2PK33AddressBytes, P2PK33index, P2PK65AddressBytes,
    P2PK65index, P2PKHAddressBytes, P2PKHindex, P2SHAddressBytes, P2SHindex, P2TRAddressBytes,
    P2TRindex, P2WPKHAddressBytes, P2WPKHindex, P2WSHAddressBytes, P2WSHindex, Pushonlyindex, Sats,
    Timestamp, TxVersion, Txid, Txindex, Txinindex, Txoutindex, Unknownindex, Weight,
};
use brk_vec::{AnyStorableVec, Compressed, Version};
use rayon::prelude::*;

use crate::Indexes;

mod base;

pub use base::*;

#[derive(Clone)]
pub struct Vecs {
    pub addressindex_to_addresstype: StorableVec<Addressindex, Addresstype>,
    pub addressindex_to_addresstypeindex: StorableVec<Addressindex, Addresstypeindex>,
    pub addressindex_to_height: StorableVec<Addressindex, Height>,
    pub height_to_blockhash: StorableVec<Height, BlockHash>,
    pub height_to_difficulty: StorableVec<Height, f64>,
    pub height_to_first_addressindex: StorableVec<Height, Addressindex>,
    pub height_to_first_emptyindex: StorableVec<Height, Emptyindex>,
    pub height_to_first_multisigindex: StorableVec<Height, Multisigindex>,
    pub height_to_first_opreturnindex: StorableVec<Height, Opreturnindex>,
    pub height_to_first_pushonlyindex: StorableVec<Height, Pushonlyindex>,
    pub height_to_first_txindex: StorableVec<Height, Txindex>,
    pub height_to_first_txinindex: StorableVec<Height, Txinindex>,
    pub height_to_first_txoutindex: StorableVec<Height, Txoutindex>,
    pub height_to_first_unknownindex: StorableVec<Height, Unknownindex>,
    pub height_to_first_p2pk33index: StorableVec<Height, P2PK33index>,
    pub height_to_first_p2pk65index: StorableVec<Height, P2PK65index>,
    pub height_to_first_p2pkhindex: StorableVec<Height, P2PKHindex>,
    pub height_to_first_p2shindex: StorableVec<Height, P2SHindex>,
    pub height_to_first_p2trindex: StorableVec<Height, P2TRindex>,
    pub height_to_first_p2wpkhindex: StorableVec<Height, P2WPKHindex>,
    pub height_to_first_p2wshindex: StorableVec<Height, P2WSHindex>,
    pub height_to_size: StorableVec<Height, usize>,
    pub height_to_timestamp: StorableVec<Height, Timestamp>,
    pub height_to_weight: StorableVec<Height, Weight>,
    pub p2pk33index_to_p2pk33addressbytes: StorableVec<P2PK33index, P2PK33AddressBytes>,
    pub p2pk65index_to_p2pk65addressbytes: StorableVec<P2PK65index, P2PK65AddressBytes>,
    pub p2pkhindex_to_p2pkhaddressbytes: StorableVec<P2PKHindex, P2PKHAddressBytes>,
    pub p2shindex_to_p2shaddressbytes: StorableVec<P2SHindex, P2SHAddressBytes>,
    pub p2trindex_to_p2traddressbytes: StorableVec<P2TRindex, P2TRAddressBytes>,
    pub p2wpkhindex_to_p2wpkhaddressbytes: StorableVec<P2WPKHindex, P2WPKHAddressBytes>,
    pub p2wshindex_to_p2wshaddressbytes: StorableVec<P2WSHindex, P2WSHAddressBytes>,
    pub txindex_to_first_txinindex: StorableVec<Txindex, Txinindex>,
    pub txindex_to_first_txoutindex: StorableVec<Txindex, Txoutindex>,
    pub txindex_to_height: StorableVec<Txindex, Height>,
    pub txindex_to_locktime: StorableVec<Txindex, LockTime>,
    pub txindex_to_txid: StorableVec<Txindex, Txid>,
    pub txindex_to_base_size: StorableVec<Txindex, usize>,
    pub txindex_to_total_size: StorableVec<Txindex, usize>,
    pub txindex_to_is_explicitly_rbf: StorableVec<Txindex, bool>,
    pub txindex_to_txversion: StorableVec<Txindex, TxVersion>,
    pub txinindex_to_txoutindex: StorableVec<Txinindex, Txoutindex>,
    pub txoutindex_to_addressindex: StorableVec<Txoutindex, Addressindex>,
    pub txoutindex_to_value: StorableVec<Txoutindex, Sats>,
}

impl Vecs {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            addressindex_to_addresstype: StorableVec::import(
                &path.join("addressindex_to_addresstype"),
                Version::from(1),
                Compressed::YES,
            )?,
            addressindex_to_addresstypeindex: StorableVec::import(
                &path.join("addressindex_to_addresstypeindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            addressindex_to_height: StorableVec::import(
                &path.join("addressindex_to_height"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_blockhash: StorableVec::import(
                &path.join("height_to_blockhash"),
                Version::from(1),
                Compressed::NO,
            )?,
            height_to_difficulty: StorableVec::import(
                &path.join("height_to_difficulty"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_addressindex: StorableVec::import(
                &path.join("height_to_first_addressindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_emptyindex: StorableVec::import(
                &path.join("height_to_first_emptyindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_multisigindex: StorableVec::import(
                &path.join("height_to_first_multisigindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_opreturnindex: StorableVec::import(
                &path.join("height_to_first_opreturnindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_pushonlyindex: StorableVec::import(
                &path.join("height_to_first_pushonlyindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_txindex: StorableVec::import(
                &path.join("height_to_first_txindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_txinindex: StorableVec::import(
                &path.join("height_to_first_txinindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_txoutindex: StorableVec::import(
                &path.join("height_to_first_txoutindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_unknownindex: StorableVec::import(
                &path.join("height_to_first_unkownindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_p2pk33index: StorableVec::import(
                &path.join("height_to_first_p2pk33index"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_p2pk65index: StorableVec::import(
                &path.join("height_to_first_p2pk65index"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_p2pkhindex: StorableVec::import(
                &path.join("height_to_first_p2pkhindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_p2shindex: StorableVec::import(
                &path.join("height_to_first_p2shindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_p2trindex: StorableVec::import(
                &path.join("height_to_first_p2trindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_p2wpkhindex: StorableVec::import(
                &path.join("height_to_first_p2wpkhindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_first_p2wshindex: StorableVec::import(
                &path.join("height_to_first_p2wshindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_size: StorableVec::import(
                &path.join("height_to_size"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_timestamp: StorableVec::import(
                &path.join("height_to_timestamp"),
                Version::from(1),
                Compressed::YES,
            )?,
            height_to_weight: StorableVec::import(
                &path.join("height_to_weight"),
                Version::from(1),
                Compressed::YES,
            )?,
            p2pk33index_to_p2pk33addressbytes: StorableVec::import(
                &path.join("p2pk33index_to_p2pk33addressbytes"),
                Version::from(1),
                Compressed::NO,
            )?,
            p2pk65index_to_p2pk65addressbytes: StorableVec::import(
                &path.join("p2pk65index_to_p2pk65addressbytes"),
                Version::from(1),
                Compressed::NO,
            )?,
            p2pkhindex_to_p2pkhaddressbytes: StorableVec::import(
                &path.join("p2pkhindex_to_p2pkhaddressbytes"),
                Version::from(1),
                Compressed::NO,
            )?,
            p2shindex_to_p2shaddressbytes: StorableVec::import(
                &path.join("p2shindex_to_p2shaddressbytes"),
                Version::from(1),
                Compressed::NO,
            )?,
            p2trindex_to_p2traddressbytes: StorableVec::import(
                &path.join("p2trindex_to_p2traddressbytes"),
                Version::from(1),
                Compressed::NO,
            )?,
            p2wpkhindex_to_p2wpkhaddressbytes: StorableVec::import(
                &path.join("p2wpkhindex_to_p2wpkhaddressbytes"),
                Version::from(1),
                Compressed::NO,
            )?,
            p2wshindex_to_p2wshaddressbytes: StorableVec::import(
                &path.join("p2wshindex_to_p2wshaddressbytes"),
                Version::from(1),
                Compressed::NO,
            )?,
            txindex_to_first_txinindex: StorableVec::import(
                &path.join("txindex_to_first_txinindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            txindex_to_first_txoutindex: StorableVec::import(
                &path.join("txindex_to_first_txoutindex"),
                Version::from(1),
                Compressed::NO,
            )?,
            txindex_to_height: StorableVec::import(
                &path.join("txindex_to_height"),
                Version::from(1),
                Compressed::YES,
            )?,
            txindex_to_locktime: StorableVec::import(
                &path.join("txindex_to_locktime"),
                Version::from(1),
                Compressed::YES,
            )?,
            txindex_to_txid: StorableVec::import(
                &path.join("txindex_to_txid"),
                Version::from(1),
                Compressed::NO,
            )?,
            txindex_to_base_size: StorableVec::import(
                &path.join("txindex_to_base_size"),
                Version::from(1),
                Compressed::YES,
            )?,
            txindex_to_total_size: StorableVec::import(
                &path.join("txindex_to_total_size"),
                Version::from(1),
                Compressed::YES,
            )?,
            txindex_to_is_explicitly_rbf: StorableVec::import(
                &path.join("txindex_to_is_explicitly_rbf"),
                Version::from(1),
                Compressed::YES,
            )?,
            txindex_to_txversion: StorableVec::import(
                &path.join("txindex_to_txversion"),
                Version::from(1),
                Compressed::YES,
            )?,
            txinindex_to_txoutindex: StorableVec::import(
                &path.join("txinindex_to_txoutindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            txoutindex_to_addressindex: StorableVec::import(
                &path.join("txoutindex_to_addressindex"),
                Version::from(1),
                Compressed::YES,
            )?,
            txoutindex_to_value: StorableVec::import(
                &path.join("txoutindex_to_value"),
                Version::from(1),
                Compressed::YES,
            )?,
        })
    }

    pub fn rollback_if_needed(&mut self, starting_indexes: &Indexes) -> brk_vec::Result<()> {
        let saved_height = starting_indexes.height.decremented().unwrap_or_default();

        // We don't want to override the starting indexes so we cut from n + 1
        let height = starting_indexes.height.incremented();

        self.height_to_first_addressindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_emptyindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_multisigindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_opreturnindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2pk33index
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2pk65index
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2pkhindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2shindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2trindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2wpkhindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2wshindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_pushonlyindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_txindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_txinindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_txoutindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_unknownindex
            .truncate_if_needed(height, saved_height)?;

        // Now we can cut everything that's out of date
        let &Indexes {
            addressindex,
            height,
            p2pk33index,
            p2pk65index,
            p2pkhindex,
            p2shindex,
            p2trindex,
            p2wpkhindex,
            p2wshindex,
            txindex,
            txinindex,
            txoutindex,
            ..
        } = starting_indexes;

        self.height_to_blockhash
            .truncate_if_needed(height, saved_height)?;
        self.height_to_difficulty
            .truncate_if_needed(height, saved_height)?;
        self.height_to_size
            .truncate_if_needed(height, saved_height)?;
        self.height_to_timestamp
            .truncate_if_needed(height, saved_height)?;
        self.height_to_weight
            .truncate_if_needed(height, saved_height)?;

        self.addressindex_to_addresstype
            .truncate_if_needed(addressindex, saved_height)?;
        self.addressindex_to_addresstypeindex
            .truncate_if_needed(addressindex, saved_height)?;
        self.addressindex_to_height
            .truncate_if_needed(addressindex, saved_height)?;

        self.p2pk33index_to_p2pk33addressbytes
            .truncate_if_needed(p2pk33index, saved_height)?;
        self.p2pk65index_to_p2pk65addressbytes
            .truncate_if_needed(p2pk65index, saved_height)?;
        self.p2pkhindex_to_p2pkhaddressbytes
            .truncate_if_needed(p2pkhindex, saved_height)?;
        self.p2shindex_to_p2shaddressbytes
            .truncate_if_needed(p2shindex, saved_height)?;
        self.p2trindex_to_p2traddressbytes
            .truncate_if_needed(p2trindex, saved_height)?;
        self.p2wpkhindex_to_p2wpkhaddressbytes
            .truncate_if_needed(p2wpkhindex, saved_height)?;
        self.p2wshindex_to_p2wshaddressbytes
            .truncate_if_needed(p2wshindex, saved_height)?;

        self.txindex_to_first_txinindex
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_first_txoutindex
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_height
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_locktime
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_txid
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_txversion
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_base_size
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_total_size
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_is_explicitly_rbf
            .truncate_if_needed(txindex, saved_height)?;

        self.txinindex_to_txoutindex
            .truncate_if_needed(txinindex, saved_height)?;

        self.txoutindex_to_addressindex
            .truncate_if_needed(txoutindex, saved_height)?;
        self.txoutindex_to_value
            .truncate_if_needed(txoutindex, saved_height)?;

        Ok(())
    }

    pub fn get_addressbytes(
        &self,
        addresstype: Addresstype,
        addresstypeindex: Addresstypeindex,
    ) -> brk_vec::Result<Option<Addressbytes>> {
        Ok(match addresstype {
            Addresstype::P2PK65 => self
                .p2pk65index_to_p2pk65addressbytes
                .get(addresstypeindex.into())?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2PK33 => self
                .p2pk33index_to_p2pk33addressbytes
                .get(addresstypeindex.into())?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2PKH => self
                .p2pkhindex_to_p2pkhaddressbytes
                .get(addresstypeindex.into())?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2SH => self
                .p2shindex_to_p2shaddressbytes
                .get(addresstypeindex.into())?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2WPKH => self
                .p2wpkhindex_to_p2wpkhaddressbytes
                .get(addresstypeindex.into())?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2WSH => self
                .p2wshindex_to_p2wshaddressbytes
                .get(addresstypeindex.into())?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            Addresstype::P2TR => self
                .p2trindex_to_p2traddressbytes
                .get(addresstypeindex.into())?
                // .map(|v| Addressbytes::from(v.clone())),
                .map(|v| Addressbytes::from(v.into_inner())),
            _ => unreachable!(),
        })
    }

    pub fn push_addressbytes_if_needed(
        &mut self,
        index: Addresstypeindex,
        addressbytes: Addressbytes,
    ) -> brk_vec::Result<()> {
        match addressbytes {
            Addressbytes::P2PK65(bytes) => self
                .p2pk65index_to_p2pk65addressbytes
                .push_if_needed(index.into(), bytes),
            Addressbytes::P2PK33(bytes) => self
                .p2pk33index_to_p2pk33addressbytes
                .push_if_needed(index.into(), bytes),
            Addressbytes::P2PKH(bytes) => self
                .p2pkhindex_to_p2pkhaddressbytes
                .push_if_needed(index.into(), bytes),
            Addressbytes::P2SH(bytes) => self
                .p2shindex_to_p2shaddressbytes
                .push_if_needed(index.into(), bytes),
            Addressbytes::P2WPKH(bytes) => self
                .p2wpkhindex_to_p2wpkhaddressbytes
                .push_if_needed(index.into(), bytes),
            Addressbytes::P2WSH(bytes) => self
                .p2wshindex_to_p2wshaddressbytes
                .push_if_needed(index.into(), bytes),
            Addressbytes::P2TR(bytes) => self
                .p2trindex_to_p2traddressbytes
                .push_if_needed(index.into(), bytes),
        }
    }

    pub fn flush(&mut self, height: Height) -> io::Result<()> {
        self.as_mut_any_vecs()
            .into_par_iter()
            .try_for_each(|vec| vec.flush(height))
    }

    pub fn starting_height(&mut self) -> Height {
        self.as_mut_any_vecs()
            .into_iter()
            .map(|vec| vec.height().map(Height::incremented).unwrap_or_default())
            .min()
            .unwrap()
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        vec![
            &*self.addressindex_to_addresstype,
            &*self.addressindex_to_addresstypeindex,
            &*self.addressindex_to_height,
            &*self.height_to_blockhash,
            &*self.height_to_difficulty,
            &*self.height_to_first_addressindex,
            &*self.height_to_first_emptyindex,
            &*self.height_to_first_multisigindex,
            &*self.height_to_first_opreturnindex,
            &*self.height_to_first_pushonlyindex,
            &*self.height_to_first_txindex,
            &*self.height_to_first_txinindex,
            &*self.height_to_first_txoutindex,
            &*self.height_to_first_unknownindex,
            &*self.height_to_first_p2pk33index,
            &*self.height_to_first_p2pk65index,
            &*self.height_to_first_p2pkhindex,
            &*self.height_to_first_p2shindex,
            &*self.height_to_first_p2trindex,
            &*self.height_to_first_p2wpkhindex,
            &*self.height_to_first_p2wshindex,
            &*self.height_to_size,
            &*self.height_to_timestamp,
            &*self.height_to_weight,
            &*self.p2pk33index_to_p2pk33addressbytes,
            &*self.p2pk65index_to_p2pk65addressbytes,
            &*self.p2pkhindex_to_p2pkhaddressbytes,
            &*self.p2shindex_to_p2shaddressbytes,
            &*self.p2trindex_to_p2traddressbytes,
            &*self.p2wpkhindex_to_p2wpkhaddressbytes,
            &*self.p2wshindex_to_p2wshaddressbytes,
            &*self.txindex_to_first_txinindex,
            &*self.txindex_to_first_txoutindex,
            &*self.txindex_to_height,
            &*self.txindex_to_locktime,
            &*self.txindex_to_txid,
            &*self.txindex_to_base_size,
            &*self.txindex_to_total_size,
            &*self.txindex_to_is_explicitly_rbf,
            &*self.txindex_to_txversion,
            &*self.txinindex_to_txoutindex,
            &*self.txoutindex_to_addressindex,
            &*self.txoutindex_to_value,
        ]
    }

    fn as_mut_any_vecs(&mut self) -> Vec<&mut dyn AnyIndexedVec> {
        vec![
            &mut self.addressindex_to_addresstype,
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
            &mut self.txindex_to_base_size,
            &mut self.txindex_to_total_size,
            &mut self.txindex_to_is_explicitly_rbf,
            &mut self.txindex_to_txversion,
            &mut self.txinindex_to_txoutindex,
            &mut self.txoutindex_to_addressindex,
            &mut self.txoutindex_to_value,
        ]
    }
}
