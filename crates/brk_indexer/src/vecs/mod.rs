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
    pub addressindex_to_addresstype: IndexedVec<Addressindex, Addresstype>,
    pub addressindex_to_addresstypeindex: IndexedVec<Addressindex, Addresstypeindex>,
    pub addressindex_to_height: IndexedVec<Addressindex, Height>,
    pub height_to_blockhash: IndexedVec<Height, BlockHash>,
    pub height_to_difficulty: IndexedVec<Height, f64>,
    pub height_to_first_addressindex: IndexedVec<Height, Addressindex>,
    pub height_to_first_emptyindex: IndexedVec<Height, Emptyindex>,
    pub height_to_first_multisigindex: IndexedVec<Height, Multisigindex>,
    pub height_to_first_opreturnindex: IndexedVec<Height, Opreturnindex>,
    pub height_to_first_pushonlyindex: IndexedVec<Height, Pushonlyindex>,
    pub height_to_first_txindex: IndexedVec<Height, Txindex>,
    pub height_to_first_txinindex: IndexedVec<Height, Txinindex>,
    pub height_to_first_txoutindex: IndexedVec<Height, Txoutindex>,
    pub height_to_first_unknownindex: IndexedVec<Height, Unknownindex>,
    pub height_to_first_p2pk33index: IndexedVec<Height, P2PK33index>,
    pub height_to_first_p2pk65index: IndexedVec<Height, P2PK65index>,
    pub height_to_first_p2pkhindex: IndexedVec<Height, P2PKHindex>,
    pub height_to_first_p2shindex: IndexedVec<Height, P2SHindex>,
    pub height_to_first_p2trindex: IndexedVec<Height, P2TRindex>,
    pub height_to_first_p2wpkhindex: IndexedVec<Height, P2WPKHindex>,
    pub height_to_first_p2wshindex: IndexedVec<Height, P2WSHindex>,
    pub height_to_size: IndexedVec<Height, usize>,
    pub height_to_timestamp: IndexedVec<Height, Timestamp>,
    pub height_to_weight: IndexedVec<Height, Weight>,
    pub p2pk33index_to_p2pk33addressbytes: IndexedVec<P2PK33index, P2PK33AddressBytes>,
    pub p2pk65index_to_p2pk65addressbytes: IndexedVec<P2PK65index, P2PK65AddressBytes>,
    pub p2pkhindex_to_p2pkhaddressbytes: IndexedVec<P2PKHindex, P2PKHAddressBytes>,
    pub p2shindex_to_p2shaddressbytes: IndexedVec<P2SHindex, P2SHAddressBytes>,
    pub p2trindex_to_p2traddressbytes: IndexedVec<P2TRindex, P2TRAddressBytes>,
    pub p2wpkhindex_to_p2wpkhaddressbytes: IndexedVec<P2WPKHindex, P2WPKHAddressBytes>,
    pub p2wshindex_to_p2wshaddressbytes: IndexedVec<P2WSHindex, P2WSHAddressBytes>,
    pub txindex_to_first_txinindex: IndexedVec<Txindex, Txinindex>,
    pub txindex_to_first_txoutindex: IndexedVec<Txindex, Txoutindex>,
    pub txindex_to_height: IndexedVec<Txindex, Height>,
    pub txindex_to_locktime: IndexedVec<Txindex, LockTime>,
    pub txindex_to_txid: IndexedVec<Txindex, Txid>,
    pub txindex_to_base_size: IndexedVec<Txindex, usize>,
    pub txindex_to_total_size: IndexedVec<Txindex, usize>,
    pub txindex_to_is_explicitly_rbf: IndexedVec<Txindex, bool>,
    pub txindex_to_txversion: IndexedVec<Txindex, TxVersion>,
    /// If txoutindex == Txoutindex MAX then is it's coinbase
    pub txinindex_to_txoutindex: IndexedVec<Txinindex, Txoutindex>,
    pub txoutindex_to_addressindex: IndexedVec<Txoutindex, Addressindex>,
    pub txoutindex_to_value: IndexedVec<Txoutindex, Sats>,
}

impl Vecs {
    pub fn import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            addressindex_to_addresstype: IndexedVec::forced_import(
                &path.join("addressindex_to_addresstype"),
                Version::ONE,
                compressed,
            )?,
            addressindex_to_addresstypeindex: IndexedVec::forced_import(
                &path.join("addressindex_to_addresstypeindex"),
                Version::ONE,
                compressed,
            )?,
            addressindex_to_height: IndexedVec::forced_import(
                &path.join("addressindex_to_height"),
                Version::ONE,
                compressed,
            )?,
            height_to_blockhash: IndexedVec::forced_import(
                &path.join("height_to_blockhash"),
                Version::ONE,
                Compressed::NO,
            )?,
            height_to_difficulty: IndexedVec::forced_import(
                &path.join("height_to_difficulty"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_addressindex: IndexedVec::forced_import(
                &path.join("height_to_first_addressindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_emptyindex: IndexedVec::forced_import(
                &path.join("height_to_first_emptyindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_multisigindex: IndexedVec::forced_import(
                &path.join("height_to_first_multisigindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_opreturnindex: IndexedVec::forced_import(
                &path.join("height_to_first_opreturnindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_pushonlyindex: IndexedVec::forced_import(
                &path.join("height_to_first_pushonlyindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_txindex: IndexedVec::forced_import(
                &path.join("height_to_first_txindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_txinindex: IndexedVec::forced_import(
                &path.join("height_to_first_txinindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_txoutindex: IndexedVec::forced_import(
                &path.join("height_to_first_txoutindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_unknownindex: IndexedVec::forced_import(
                &path.join("height_to_first_unkownindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_p2pk33index: IndexedVec::forced_import(
                &path.join("height_to_first_p2pk33index"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_p2pk65index: IndexedVec::forced_import(
                &path.join("height_to_first_p2pk65index"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_p2pkhindex: IndexedVec::forced_import(
                &path.join("height_to_first_p2pkhindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_p2shindex: IndexedVec::forced_import(
                &path.join("height_to_first_p2shindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_p2trindex: IndexedVec::forced_import(
                &path.join("height_to_first_p2trindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_p2wpkhindex: IndexedVec::forced_import(
                &path.join("height_to_first_p2wpkhindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_first_p2wshindex: IndexedVec::forced_import(
                &path.join("height_to_first_p2wshindex"),
                Version::ONE,
                compressed,
            )?,
            height_to_size: IndexedVec::forced_import(
                &path.join("height_to_size"),
                Version::ONE,
                compressed,
            )?,
            height_to_timestamp: IndexedVec::forced_import(
                &path.join("height_to_timestamp"),
                Version::ONE,
                compressed,
            )?,
            height_to_weight: IndexedVec::forced_import(
                &path.join("height_to_weight"),
                Version::ONE,
                compressed,
            )?,
            p2pk33index_to_p2pk33addressbytes: IndexedVec::forced_import(
                &path.join("p2pk33index_to_p2pk33addressbytes"),
                Version::ONE,
                Compressed::NO,
            )?,
            p2pk65index_to_p2pk65addressbytes: IndexedVec::forced_import(
                &path.join("p2pk65index_to_p2pk65addressbytes"),
                Version::ONE,
                Compressed::NO,
            )?,
            p2pkhindex_to_p2pkhaddressbytes: IndexedVec::forced_import(
                &path.join("p2pkhindex_to_p2pkhaddressbytes"),
                Version::ONE,
                Compressed::NO,
            )?,
            p2shindex_to_p2shaddressbytes: IndexedVec::forced_import(
                &path.join("p2shindex_to_p2shaddressbytes"),
                Version::ONE,
                Compressed::NO,
            )?,
            p2trindex_to_p2traddressbytes: IndexedVec::forced_import(
                &path.join("p2trindex_to_p2traddressbytes"),
                Version::ONE,
                Compressed::NO,
            )?,
            p2wpkhindex_to_p2wpkhaddressbytes: IndexedVec::forced_import(
                &path.join("p2wpkhindex_to_p2wpkhaddressbytes"),
                Version::ONE,
                Compressed::NO,
            )?,
            p2wshindex_to_p2wshaddressbytes: IndexedVec::forced_import(
                &path.join("p2wshindex_to_p2wshaddressbytes"),
                Version::ONE,
                Compressed::NO,
            )?,
            txindex_to_first_txinindex: IndexedVec::forced_import(
                &path.join("txindex_to_first_txinindex"),
                Version::ONE,
                compressed,
            )?,
            txindex_to_first_txoutindex: IndexedVec::forced_import(
                &path.join("txindex_to_first_txoutindex"),
                Version::ONE,
                Compressed::NO,
            )?,
            txindex_to_height: IndexedVec::forced_import(
                &path.join("txindex_to_height"),
                Version::ONE,
                compressed,
            )?,
            txindex_to_locktime: IndexedVec::forced_import(
                &path.join("txindex_to_locktime"),
                Version::ONE,
                compressed,
            )?,
            txindex_to_txid: IndexedVec::forced_import(
                &path.join("txindex_to_txid"),
                Version::ONE,
                Compressed::NO,
            )?,
            txindex_to_base_size: IndexedVec::forced_import(
                &path.join("txindex_to_base_size"),
                Version::ONE,
                compressed,
            )?,
            txindex_to_total_size: IndexedVec::forced_import(
                &path.join("txindex_to_total_size"),
                Version::ONE,
                compressed,
            )?,
            txindex_to_is_explicitly_rbf: IndexedVec::forced_import(
                &path.join("txindex_to_is_explicitly_rbf"),
                Version::ONE,
                compressed,
            )?,
            txindex_to_txversion: IndexedVec::forced_import(
                &path.join("txindex_to_txversion"),
                Version::ONE,
                compressed,
            )?,
            txinindex_to_txoutindex: IndexedVec::forced_import(
                &path.join("txinindex_to_txoutindex"),
                Version::ONE,
                compressed,
            )?,
            txoutindex_to_addressindex: IndexedVec::forced_import(
                &path.join("txoutindex_to_addressindex"),
                Version::ONE,
                compressed,
            )?,
            txoutindex_to_value: IndexedVec::forced_import(
                &path.join("txoutindex_to_value"),
                Version::ONE,
                compressed,
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
            self.addressindex_to_addresstype.any_vec(),
            self.addressindex_to_addresstypeindex.any_vec(),
            self.addressindex_to_height.any_vec(),
            self.height_to_blockhash.any_vec(),
            self.height_to_difficulty.any_vec(),
            self.height_to_first_addressindex.any_vec(),
            self.height_to_first_emptyindex.any_vec(),
            self.height_to_first_multisigindex.any_vec(),
            self.height_to_first_opreturnindex.any_vec(),
            self.height_to_first_pushonlyindex.any_vec(),
            self.height_to_first_txindex.any_vec(),
            self.height_to_first_txinindex.any_vec(),
            self.height_to_first_txoutindex.any_vec(),
            self.height_to_first_unknownindex.any_vec(),
            self.height_to_first_p2pk33index.any_vec(),
            self.height_to_first_p2pk65index.any_vec(),
            self.height_to_first_p2pkhindex.any_vec(),
            self.height_to_first_p2shindex.any_vec(),
            self.height_to_first_p2trindex.any_vec(),
            self.height_to_first_p2wpkhindex.any_vec(),
            self.height_to_first_p2wshindex.any_vec(),
            self.height_to_size.any_vec(),
            self.height_to_timestamp.any_vec(),
            self.height_to_weight.any_vec(),
            self.p2pk33index_to_p2pk33addressbytes.any_vec(),
            self.p2pk65index_to_p2pk65addressbytes.any_vec(),
            self.p2pkhindex_to_p2pkhaddressbytes.any_vec(),
            self.p2shindex_to_p2shaddressbytes.any_vec(),
            self.p2trindex_to_p2traddressbytes.any_vec(),
            self.p2wpkhindex_to_p2wpkhaddressbytes.any_vec(),
            self.p2wshindex_to_p2wshaddressbytes.any_vec(),
            self.txindex_to_first_txinindex.any_vec(),
            self.txindex_to_first_txoutindex.any_vec(),
            self.txindex_to_height.any_vec(),
            self.txindex_to_locktime.any_vec(),
            self.txindex_to_txid.any_vec(),
            self.txindex_to_base_size.any_vec(),
            self.txindex_to_total_size.any_vec(),
            self.txindex_to_is_explicitly_rbf.any_vec(),
            self.txindex_to_txversion.any_vec(),
            self.txinindex_to_txoutindex.any_vec(),
            self.txoutindex_to_addressindex.any_vec(),
            self.txoutindex_to_value.any_vec(),
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
