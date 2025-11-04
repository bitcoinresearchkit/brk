use std::{fs, path::Path, sync::Arc};

use brk_error::Result;
use brk_grouper::ByAddressType;
use brk_store::{AnyStore, StoreRedb as Store};
use brk_types::{
    AddressBytes, AddressBytesHash, BlockHashPrefix, Height, OutPoint, StoredString, TxIndex,
    TxOutIndex, TxidPrefix, TypeIndex, TypeIndexAndOutPoint, TypeIndexAndTxIndex, Unit, Version,
    Vout,
};
use rayon::prelude::*;
use redb::Database;
use vecdb::{AnyVec, GenericStoredVec, StoredIndex, VecIterator, VecIteratorExtended};

use crate::Indexes;

use super::Vecs;

#[derive(Clone)]
pub struct Stores {
    pub database: Arc<Database>,

    pub addressbyteshash_to_typeindex: Store<AddressBytesHash, TypeIndex>,
    pub blockhashprefix_to_height: Store<BlockHashPrefix, Height>,
    pub height_to_coinbase_tag: Store<Height, StoredString>,
    pub txidprefix_to_txindex: Store<TxidPrefix, TxIndex>,
    pub addresstype_to_typeindex_and_txindex: ByAddressType<Store<TypeIndexAndTxIndex, Unit>>,
    pub addresstype_to_typeindex_and_unspentoutpoint:
        ByAddressType<Store<TypeIndexAndOutPoint, Unit>>,
}

impl Stores {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        let pathbuf = parent.join("stores");
        let path = pathbuf.as_path();

        fs::create_dir_all(&pathbuf)?;

        let database = Arc::new(match brk_store::open_redb_database(path) {
            Ok(database) => database,
            Err(_) => {
                fs::remove_dir_all(path)?;
                return Self::forced_import(path, version);
            }
        });

        let database_ref = &database;

        let create_addressindex_and_txindex_store = |index| {
            Store::import(
                database_ref,
                path,
                &format!("a2t{}", index),
                version,
                Some(false),
            )
        };

        let create_addressindex_and_unspentoutpoint_store =
            |index| Store::import(database_ref, path, &format!("a2u{}", index), version, None);

        Ok(Self {
            database: database.clone(),

            height_to_coinbase_tag: Store::import(database_ref, path, "h2c", version, None)?,
            addressbyteshash_to_typeindex: Store::import(database_ref, path, "a2t", version, None)?,
            blockhashprefix_to_height: Store::import(database_ref, path, "b2h", version, None)?,
            txidprefix_to_txindex: Store::import(database_ref, path, "t2t", version, None)?,
            addresstype_to_typeindex_and_txindex: ByAddressType::new_with_index(
                create_addressindex_and_txindex_store,
            )?,
            addresstype_to_typeindex_and_unspentoutpoint: ByAddressType::new_with_index(
                create_addressindex_and_unspentoutpoint_store,
            )?,
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
        ]
        // .into_iter() // Changed from par_iter_mut()
        .into_par_iter() // Changed from par_iter_mut()
        .chain(
            self.addresstype_to_typeindex_and_txindex
                // .iter_mut()
                .par_iter_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addresstype_to_typeindex_and_unspentoutpoint
                // .iter_mut()
                .par_iter_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .try_for_each(|store| store.commit(height))?;

        Ok(())
        // self.database
        //     .persist(PersistMode::SyncAll)
        //     .map_err(|e| e.into())
    }

    fn iter_any_store(&self) -> impl Iterator<Item = &dyn AnyStore> {
        [
            &self.addressbyteshash_to_typeindex as &dyn AnyStore,
            &self.blockhashprefix_to_height,
            &self.height_to_coinbase_tag,
            &self.txidprefix_to_txindex,
        ]
        .into_iter()
        .chain(
            self.addresstype_to_typeindex_and_txindex
                .iter()
                .map(|s| s as &dyn AnyStore),
        )
        .chain(
            self.addresstype_to_typeindex_and_unspentoutpoint
                .iter()
                .map(|s| s as &dyn AnyStore),
        )
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
            && self
                .addresstype_to_typeindex_and_txindex
                .iter()
                .map(|s| s.is_empty())
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .all(|empty| empty)
            && self
                .addresstype_to_typeindex_and_unspentoutpoint
                .iter()
                .map(|s| s.is_empty())
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .all(|empty| empty)
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
                .one_shot_read(starting_indexes.height)
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
                .one_shot_read(starting_indexes.height)
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
                .one_shot_read(starting_indexes.height)
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
                .one_shot_read(starting_indexes.height)
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
                .one_shot_read(starting_indexes.height)
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
                .one_shot_read(starting_indexes.height)
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
                .one_shot_read(starting_indexes.height)
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
                .one_shot_read(starting_indexes.height)
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
                .for_each(|((txoutindex, outputtype), typeindex)| {
                    let txindex = txoutindex_to_txindex_iter.unsafe_get_(txoutindex);

                    let vout = Vout::from(
                        txoutindex.to_usize()
                            - txindex_to_first_txoutindex_iter
                                .unsafe_get(txindex)
                                .to_usize(),
                    );
                    let outpoint = OutPoint::new(txindex, vout);

                    self.addresstype_to_typeindex_and_unspentoutpoint
                        .get_mut(outputtype)
                        .unwrap()
                        .remove(TypeIndexAndOutPoint::from((typeindex, outpoint)));
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
                    let txoutindex = txindex_to_first_txoutindex_iter.unsafe_get(txindex) + vout;

                    // Only process if this output was created before the rollback point
                    if txoutindex < starting_indexes.txoutindex {
                        let outputtype = txoutindex_to_outputtype_iter.unsafe_get(txoutindex);

                        if outputtype.is_address() {
                            let typeindex = txoutindex_to_typeindex_iter.unsafe_get(txoutindex);

                            self.addresstype_to_typeindex_and_unspentoutpoint
                                .get_mut(outputtype)
                                .unwrap()
                                .insert(TypeIndexAndOutPoint::from((typeindex, outpoint)), Unit);
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
