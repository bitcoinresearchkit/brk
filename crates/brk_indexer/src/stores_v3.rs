use std::{fs, path::Path};

use brk_error::Result;
use brk_store::{AnyStore, Kind3, Mode3, StoreFjallV3 as Store};
use brk_types::{
    AddressBytes, AddressBytesHash, AddressTypeAddressIndexOutPoint,
    AddressTypeAddressIndexTxIndex, BlockHashPrefix, Height, OutPoint, StoredString, TxIndex,
    TxOutIndex, TxidPrefix, TypeIndex, Unit, Version, Vout,
};
use fjall3::{Database, PersistMode};
use rayon::prelude::*;
use vecdb::{AnyVec, GenericStoredVec, StoredIndex, VecIterator, VecIteratorExtended};

use crate::Indexes;

use super::Vecs;

#[derive(Clone)]
pub struct Stores {
    pub database: Database,

    pub addressbyteshash_to_typeindex: Store<AddressBytesHash, TypeIndex>,
    pub blockhashprefix_to_height: Store<BlockHashPrefix, Height>,
    pub height_to_coinbase_tag: Store<Height, StoredString>,
    pub txidprefix_to_txindex: Store<TxidPrefix, TxIndex>,
    pub addresstype_to_addressindex_and_txindex: Store<AddressTypeAddressIndexTxIndex, Unit>,
    // pub addresstype_to_addressindex_and_unspentoutpoint:
    // Store<AddressTypeAddressIndexOutPoint, Unit>,
}

impl Stores {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        let pathbuf = parent.join("stores");
        let path = pathbuf.as_path();

        fs::create_dir_all(&pathbuf)?;

        let database = match brk_store::open_fjall3_database(path) {
            Ok(database) => database,
            Err(_) => {
                fs::remove_dir_all(path)?;
                return Self::forced_import(path, version);
            }
        };

        let database_ref = &database;

        Ok(Self {
            database: database.clone(),

            height_to_coinbase_tag: Store::import(
                database_ref,
                path,
                "height_to_coinbase_tag",
                version,
                Mode3::PushOnly,
                Kind3::Sequential,
            )?,
            addressbyteshash_to_typeindex: Store::import(
                database_ref,
                path,
                "addressbyteshash_to_typeindex",
                version,
                Mode3::PushOnly,
                Kind3::Random,
            )?,
            blockhashprefix_to_height: Store::import(
                database_ref,
                path,
                "blockhashprefix_to_height",
                version,
                Mode3::PushOnly,
                Kind3::Random,
            )?,
            txidprefix_to_txindex: Store::import(
                database_ref,
                path,
                "txidprefix_to_txindex",
                version,
                Mode3::PushOnly,
                Kind3::Random,
            )?,
            addresstype_to_addressindex_and_txindex: Store::import(
                database_ref,
                path,
                "addresstype_to_addressindex_and_txindex",
                version,
                Mode3::PushOnly,
                Kind3::Vec,
            )?,
            // addresstype_to_addressindex_and_unspentoutpoint: Store::import(
            //     database_ref,
            //     path,
            //     "addresstype_to_addressindex_and_unspentoutpoint",
            //     version,
            //     Mode3::Any,
            //     Kind3::Vec,
            // )?,
        })
    }

    pub fn starting_height(&self) -> Height {
        self.iter_any_store()
            .map(|store| {
                // let height =
                store.height().map(Height::incremented).unwrap_or_default()
                // dbg!((height, store.name()));
            })
            .min()
            .unwrap()
    }

    pub fn commit(&mut self, height: Height) -> Result<()> {
        [
            &mut self.addressbyteshash_to_typeindex as &mut dyn AnyStore,
            &mut self.blockhashprefix_to_height,
            &mut self.height_to_coinbase_tag,
            &mut self.txidprefix_to_txindex,
            &mut self.addresstype_to_addressindex_and_txindex,
            // &mut self.addresstype_to_addressindex_and_unspentoutpoint,
        ]
        .into_par_iter() // Changed from par_iter_mut()
        .try_for_each(|store| store.commit(height))?;

        self.database
            .persist(PersistMode::SyncAll)
            .map_err(|e| e.into())
    }

    fn iter_any_store(&self) -> impl Iterator<Item = &dyn AnyStore> {
        [
            &self.addressbyteshash_to_typeindex as &dyn AnyStore,
            &self.blockhashprefix_to_height,
            &self.height_to_coinbase_tag,
            &self.txidprefix_to_txindex,
            &self.addresstype_to_addressindex_and_txindex,
            // &self.addresstype_to_addressindex_and_unspentoutpoint,
        ]
        .into_iter()
    }

    pub fn rollback_if_needed(
        &mut self,
        vecs: &mut Vecs,
        starting_indexes: &Indexes,
    ) -> Result<()> {
        if self.addressbyteshash_to_typeindex.is_empty()?
            && self.blockhashprefix_to_height.is_empty()?
            && self.txidprefix_to_txindex.is_empty()?
            && self.height_to_coinbase_tag.is_empty()?
            && self.addresstype_to_addressindex_and_txindex.is_empty()?
        // && self
        //     .addresstype_to_addressindex_and_unspentoutpoint
        //     .is_empty()?
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
                .read(starting_indexes.height)
            {
                let mut p2pk65addressindex_to_p2pk65bytes_iter =
                    vecs.p2pk65addressindex_to_p2pk65bytes.iter()?;

                while let Some(typedbytes) = p2pk65addressindex_to_p2pk65bytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2pk33addressindex
                .read(starting_indexes.height)
            {
                let mut p2pk33addressindex_to_p2pk33bytes_iter =
                    vecs.p2pk33addressindex_to_p2pk33bytes.iter()?;

                while let Some(typedbytes) = p2pk33addressindex_to_p2pk33bytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2pkhaddressindex
                .read(starting_indexes.height)
            {
                let mut p2pkhaddressindex_to_p2pkhbytes_iter =
                    vecs.p2pkhaddressindex_to_p2pkhbytes.iter()?;

                while let Some(typedbytes) = p2pkhaddressindex_to_p2pkhbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2shaddressindex
                .read(starting_indexes.height)
            {
                let mut p2shaddressindex_to_p2shbytes_iter =
                    vecs.p2shaddressindex_to_p2shbytes.iter()?;

                while let Some(typedbytes) = p2shaddressindex_to_p2shbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2traddressindex
                .read(starting_indexes.height)
            {
                let mut p2traddressindex_to_p2trbytes_iter =
                    vecs.p2traddressindex_to_p2trbytes.iter()?;

                while let Some(typedbytes) = p2traddressindex_to_p2trbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2wpkhaddressindex
                .read(starting_indexes.height)
            {
                let mut p2wpkhaddressindex_to_p2wpkhbytes_iter =
                    vecs.p2wpkhaddressindex_to_p2wpkhbytes.iter()?;

                while let Some(typedbytes) = p2wpkhaddressindex_to_p2wpkhbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2wshaddressindex
                .read(starting_indexes.height)
            {
                let mut p2wshaddressindex_to_p2wshbytes_iter =
                    vecs.p2wshaddressindex_to_p2wshbytes.iter()?;

                while let Some(typedbytes) = p2wshaddressindex_to_p2wshbytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Ok(mut index) = vecs
                .height_to_first_p2aaddressindex
                .read(starting_indexes.height)
            {
                let mut p2aaddressindex_to_p2abytes_iter =
                    vecs.p2aaddressindex_to_p2abytes.iter()?;

                while let Some(typedbytes) = p2aaddressindex_to_p2abytes_iter.get(index) {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }
        } else {
            unreachable!();
            // self.blockhashprefix_to_height.reset()?;
            // self.addressbyteshash_to_typeindex.reset()?;
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
                    let txindex = txoutindex_to_txindex_iter.get_unwrap_at(txoutindex);

                    self.addresstype_to_addressindex_and_txindex.remove(
                        AddressTypeAddressIndexTxIndex::from((addresstype, addressindex, txindex)),
                    );

                    let vout = Vout::from(
                        txoutindex.to_usize()
                            - txindex_to_first_txoutindex_iter
                                .get_unwrap(txindex)
                                .to_usize(),
                    );
                    let outpoint = OutPoint::new(txindex, vout);

                    // self.addresstype_to_addressindex_and_unspentoutpoint.remove(
                    //     AddressTypeAddressIndexOutPoint::from((
                    //         addresstype,
                    //         addressindex,
                    //         outpoint,
                    //     )),
                    // );
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

                            self.addresstype_to_addressindex_and_txindex.remove(
                                AddressTypeAddressIndexTxIndex::from((
                                    addresstype,
                                    addressindex,
                                    txindex,
                                )),
                            );

                            // self.addresstype_to_addressindex_and_unspentoutpoint.insert(
                            //     AddressTypeAddressIndexOutPoint::from((
                            //         addresstype,
                            //         addressindex,
                            //         outpoint,
                            //     )),
                            //     Unit,
                            // );
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
