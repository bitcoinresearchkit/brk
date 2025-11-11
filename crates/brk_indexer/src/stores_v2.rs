use std::{fs, path::Path};

use brk_error::Result;
use brk_grouper::ByAddressType;
use brk_store::{AnyStore, Mode, StoreFjallV2 as Store, Type};
use brk_types::{
    AddressBytes, AddressHash, AddressIndexOutPoint, AddressIndexTxIndex, BlockHashPrefix, Height,
    OutPoint, OutputType, StoredString, TxIndex, TxOutIndex, TxidPrefix, TypeIndex, Unit, Version,
    Vout,
};
use fjall2::{CompressionType as Compression, PersistMode, TransactionalKeyspace};
use rayon::prelude::*;
use vecdb::{AnyVec, GenericStoredVec, TypedVecIterator, VecIndex, VecIterator};

use crate::Indexes;

use super::Vecs;

#[derive(Clone)]
pub struct Stores {
    pub keyspace: TransactionalKeyspace,

    pub addresstype_to_addresshash_to_addressindex: ByAddressType<Store<AddressHash, TypeIndex>>,
    pub addresstype_to_addressindex_and_txindex: ByAddressType<Store<AddressIndexTxIndex, Unit>>,
    pub addresstype_to_addressindex_and_unspentoutpoint:
        ByAddressType<Store<AddressIndexOutPoint, Unit>>,
    pub blockhashprefix_to_height: Store<BlockHashPrefix, Height>,
    pub height_to_coinbase_tag: Store<Height, StoredString>,
    pub txidprefix_to_txindex: Store<TxidPrefix, TxIndex>,
}

impl Stores {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        let pathbuf = parent.join("stores");
        let path = pathbuf.as_path();

        fs::create_dir_all(&pathbuf)?;

        let keyspace = match brk_store::open_keyspace(path) {
            Ok(keyspace) => keyspace,
            Err(_) => {
                fs::remove_dir_all(path)?;
                return Self::forced_import(path, version);
            }
        };

        let keyspace_ref = &keyspace;

        let create_addresshash_to_addressindex_store = |index| {
            Store::import(
                keyspace_ref,
                path,
                &format!("h2i{}", index),
                version,
                Mode::UniquePushOnly(Type::Random),
                Compression::Lz4,
            )
        };

        let create_addressindex_to_txindex_store = |index| {
            Store::import(
                keyspace_ref,
                path,
                &format!("a2t{}", index),
                version,
                Mode::VecLike,
                Compression::Lz4,
            )
        };

        let create_addressindex_to_unspentoutpoint_store = |index| {
            Store::import(
                keyspace_ref,
                path,
                &format!("a2u{}", index),
                version,
                Mode::VecLike,
                Compression::Lz4,
            )
        };

        Ok(Self {
            keyspace: keyspace.clone(),

            height_to_coinbase_tag: Store::import(
                keyspace_ref,
                path,
                "h2c",
                version,
                Mode::UniquePushOnly(Type::Sequential),
                Compression::Lz4,
            )?,
            addresstype_to_addresshash_to_addressindex: ByAddressType::new_with_index(
                create_addresshash_to_addressindex_store,
            )?,
            blockhashprefix_to_height: Store::import(
                keyspace_ref,
                path,
                "b2h",
                version,
                Mode::UniquePushOnly(Type::Random),
                Compression::Lz4,
            )?,
            txidprefix_to_txindex: Store::import(
                keyspace_ref,
                path,
                "t2t",
                version,
                Mode::UniquePushOnly(Type::Random),
                Compression::Lz4,
            )?,
            addresstype_to_addressindex_and_txindex: ByAddressType::new_with_index(
                create_addressindex_to_txindex_store,
            )?,
            addresstype_to_addressindex_and_unspentoutpoint: ByAddressType::new_with_index(
                create_addressindex_to_unspentoutpoint_store,
            )?,
        })
    }

    pub fn starting_height(&self) -> Height {
        [
            &self.blockhashprefix_to_height as &dyn AnyStore,
            &self.height_to_coinbase_tag,
            &self.txidprefix_to_txindex,
        ]
        .into_iter()
        .chain(
            self.addresstype_to_addresshash_to_addressindex
                .iter()
                .map(|s| s as &dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_txindex
                .iter()
                .map(|s| s as &dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_unspentoutpoint
                .iter()
                .map(|s| s as &dyn AnyStore),
        )
        .map(|store| {
            // let height =
            store.height().map(Height::incremented).unwrap_or_default()
            // dbg!((height, store.name()));
        })
        .min()
        .unwrap()
    }

    pub fn commit(&mut self, height: Height) -> Result<()> {
        let tuples = [
            &mut self.blockhashprefix_to_height as &mut dyn AnyStore,
            &mut self.height_to_coinbase_tag,
            &mut self.txidprefix_to_txindex,
        ]
        .into_par_iter()
        .chain(
            self.addresstype_to_addresshash_to_addressindex
                .par_iter_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_txindex
                .par_iter_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_unspentoutpoint
                .par_iter_mut()
                .map(|s| s as &mut dyn AnyStore),
        ) // Changed from par_iter_mut()
        .map(|store| {
            let items = store.take_all_f2();
            store.export_meta_if_needed(height)?;
            Ok((store.partition(), items))
        })
        .collect::<Result<Vec<_>>>()?;

        self.keyspace.inner().batch().commit_partitions(tuples)?;

        self.keyspace
            .persist(PersistMode::SyncAll)
            .map_err(|e| e.into())
    }

    pub fn rollback_if_needed(
        &mut self,
        vecs: &mut Vecs,
        starting_indexes: &Indexes,
    ) -> Result<()> {
        if self.blockhashprefix_to_height.is_empty()?
            && self.txidprefix_to_txindex.is_empty()?
            && self.height_to_coinbase_tag.is_empty()?
            && self
                .addresstype_to_addresshash_to_addressindex
                .iter()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
            && self
                .addresstype_to_addressindex_and_txindex
                .iter()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
            && self
                .addresstype_to_addressindex_and_unspentoutpoint
                .iter()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
        {
            return Ok(());
        }

        if starting_indexes.height != Height::ZERO {
            vecs.height_to_blockhash
                .iter()?
                .skip(starting_indexes.height.to_usize())
                .map(BlockHashPrefix::from)
                .for_each(|prefix| {
                    self.blockhashprefix_to_height.remove(prefix);
                });

            (starting_indexes.height.to_usize()..vecs.height_to_blockhash.len())
                .map(Height::from)
                .for_each(|h| {
                    self.height_to_coinbase_tag.remove(h);
                });

            if let Ok(mut index) = vecs
                .height_to_first_p2pk65addressindex
                .read_once(starting_indexes.height)
            {
                let mut p2pk65addressindex_to_p2pk65bytes_iter =
                    vecs.p2pk65addressindex_to_p2pk65bytes.iter()?;

                while let Some(typedbytes) = p2pk65addressindex_to_p2pk65bytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressHash::from(&bytes);
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(OutputType::P2PK65)
                        .remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2pk33addressindex
                .read_once(starting_indexes.height)
            {
                let mut p2pk33addressindex_to_p2pk33bytes_iter =
                    vecs.p2pk33addressindex_to_p2pk33bytes.iter()?;

                while let Some(typedbytes) = p2pk33addressindex_to_p2pk33bytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressHash::from(&bytes);
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(OutputType::P2PK33)
                        .remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2pkhaddressindex
                .read_once(starting_indexes.height)
            {
                let mut p2pkhaddressindex_to_p2pkhbytes_iter =
                    vecs.p2pkhaddressindex_to_p2pkhbytes.iter()?;

                while let Some(typedbytes) = p2pkhaddressindex_to_p2pkhbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressHash::from(&bytes);
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(OutputType::P2PKH)
                        .remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2shaddressindex
                .read_once(starting_indexes.height)
            {
                let mut p2shaddressindex_to_p2shbytes_iter =
                    vecs.p2shaddressindex_to_p2shbytes.iter()?;

                while let Some(typedbytes) = p2shaddressindex_to_p2shbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressHash::from(&bytes);
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(OutputType::P2SH)
                        .remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2wpkhaddressindex
                .read_once(starting_indexes.height)
            {
                let mut p2wpkhaddressindex_to_p2wpkhbytes_iter =
                    vecs.p2wpkhaddressindex_to_p2wpkhbytes.iter()?;

                while let Some(typedbytes) = p2wpkhaddressindex_to_p2wpkhbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressHash::from(&bytes);
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(OutputType::P2WPKH)
                        .remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2wshaddressindex
                .read_once(starting_indexes.height)
            {
                let mut p2wshaddressindex_to_p2wshbytes_iter =
                    vecs.p2wshaddressindex_to_p2wshbytes.iter()?;

                while let Some(typedbytes) = p2wshaddressindex_to_p2wshbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressHash::from(&bytes);
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(OutputType::P2WSH)
                        .remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2traddressindex
                .read_once(starting_indexes.height)
            {
                let mut p2traddressindex_to_p2trbytes_iter =
                    vecs.p2traddressindex_to_p2trbytes.iter()?;

                while let Some(typedbytes) = p2traddressindex_to_p2trbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressHash::from(&bytes);
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(OutputType::P2TR)
                        .remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2aaddressindex
                .read_once(starting_indexes.height)
            {
                let mut p2aaddressindex_to_p2abytes_iter =
                    vecs.p2aaddressindex_to_p2abytes.iter()?;

                while let Some(typedbytes) = p2aaddressindex_to_p2abytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressHash::from(&bytes);
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(OutputType::P2A)
                        .remove(hash);
                    index.increment();
                }
            }
        } else {
            unreachable!();
            // self.blockhashprefix_to_height.reset()?;
            // self.addresshash_to_typeindex.reset()?;
        }

        if starting_indexes.txindex != TxIndex::ZERO {
            vecs.txindex_to_txid
                .iter()?
                .enumerate()
                .skip(starting_indexes.txindex.to_usize())
                .for_each(|(txindex, txid)| {
                    let txindex = TxIndex::from(txindex);

                    let txidprefix = TxidPrefix::from(&txid);

                    // "d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599"
                    let is_not_first_dup = txindex != TxIndex::new(142783)
                        || txidprefix != TxidPrefix::from([153, 133, 216, 41, 84, 225, 15, 34]);

                    // "e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468"
                    let is_not_second_dup = txindex != TxIndex::new(142841)
                        || txidprefix != TxidPrefix::from([104, 180, 95, 88, 182, 116, 233, 78]);

                    if is_not_first_dup && is_not_second_dup {
                        self.txidprefix_to_txindex.remove(txidprefix);
                    }
                });
        } else {
            unreachable!();
            // self.txidprefix_to_txindex.reset()?;
        }

        if starting_indexes.txoutindex != TxOutIndex::ZERO {
            let mut txoutindex_to_txindex_iter = vecs.txoutindex_to_txindex.iter()?;
            let mut txindex_to_first_txoutindex_iter = vecs.txindex_to_first_txoutindex.iter()?;
            vecs.txoutindex_to_outputtype
                .iter()?
                .enumerate()
                .skip(starting_indexes.txoutindex.to_usize())
                .zip(
                    vecs.txoutindex_to_typeindex
                        .iter()?
                        .skip(starting_indexes.txoutindex.to_usize()),
                )
                .filter(|((_, outputtype), _)| outputtype.is_address())
                .for_each(|((txoutindex, addresstype), addressindex)| {
                    let txindex = txoutindex_to_txindex_iter.get_at_unwrap(txoutindex);

                    self.addresstype_to_addressindex_and_txindex
                        .get_mut_unwrap(addresstype)
                        .remove(AddressIndexTxIndex::from((addressindex, txindex)));

                    let vout = Vout::from(
                        txoutindex.to_usize()
                            - txindex_to_first_txoutindex_iter
                                .get_unwrap(txindex)
                                .to_usize(),
                    );
                    let outpoint = OutPoint::new(txindex, vout);

                    self.addresstype_to_addressindex_and_unspentoutpoint
                        .get_mut_unwrap(addresstype)
                        .remove(AddressIndexOutPoint::from((addressindex, outpoint)));
                });

            // Add back outputs that were spent after the rollback point
            let mut txindex_to_first_txoutindex_iter = vecs.txindex_to_first_txoutindex.iter()?;
            let mut txoutindex_to_outputtype_iter = vecs.txoutindex_to_outputtype.iter()?;
            let mut txoutindex_to_typeindex_iter = vecs.txoutindex_to_typeindex.iter()?;
            vecs.txinindex_to_outpoint
                .iter()?
                .skip(starting_indexes.txinindex.to_usize())
                .for_each(|outpoint| {
                    if outpoint.is_coinbase() {
                        return;
                    }

                    let txindex = outpoint.txindex();
                    let vout = outpoint.vout();

                    // Calculate txoutindex from txindex and vout
                    let txoutindex = txindex_to_first_txoutindex_iter.get_unwrap(txindex) + vout;

                    // Only process if this output was created before the rollback point
                    if txoutindex < starting_indexes.txoutindex {
                        let outputtype = txoutindex_to_outputtype_iter.get_unwrap(txoutindex);

                        if outputtype.is_address() {
                            let addresstype = outputtype;
                            let addressindex = txoutindex_to_typeindex_iter.get_unwrap(txoutindex);

                            self.addresstype_to_addressindex_and_txindex
                                .get_mut_unwrap(addresstype)
                                .remove(AddressIndexTxIndex::from((addressindex, txindex)));

                            self.addresstype_to_addressindex_and_unspentoutpoint
                                .get_mut_unwrap(addresstype)
                                .insert(AddressIndexOutPoint::from((addressindex, outpoint)), Unit);
                        }
                    }
                });
        } else {
            unreachable!();
            // self.addresstype_to_typeindex_and_txindex
            //     .iter_mut()
            //     .try_for_each(|s| s.reset())?;
            // self.addresstype_to_typeindex_and_unspentoutpoint
            //     .iter_mut()
            //     .try_for_each(|s| s.reset())?;
        }

        self.commit(starting_indexes.height.decremented().unwrap_or_default())?;

        Ok(())
    }
}
