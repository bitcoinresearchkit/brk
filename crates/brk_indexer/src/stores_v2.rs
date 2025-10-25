use std::{borrow::Cow, fs, path::Path};

use brk_error::Result;
use brk_grouper::ByAddressType;
use brk_store::{AnyStore, StoreV2 as Store};
use brk_types::{
    AddressBytes, AddressBytesHash, BlockHashPrefix, Height, OutPoint, StoredString, TxIndex,
    TxOutIndex, TxidPrefix, TypeIndex, TypeIndexAndOutPoint, TypeIndexAndTxIndex, Unit, Version,
    Vout,
};
use fjall2::{PersistMode, TransactionalKeyspace};
use rayon::prelude::*;
use vecdb::{AnyVec, StoredIndex, VecIterator};

use crate::Indexes;

use super::Vecs;

#[derive(Clone)]
pub struct Stores {
    pub keyspace: TransactionalKeyspace,

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

        let keyspace = match brk_store::open_keyspace(path) {
            Ok(keyspace) => keyspace,
            Err(_) => {
                fs::remove_dir_all(path)?;
                return Self::forced_import(path, version);
            }
        };

        let keyspace_ref = &keyspace;

        let create_addressindex_and_txindex_store = |index| {
            Store::import(
                keyspace_ref,
                path,
                &format!("a2t{}", index),
                version,
                Some(false),
            )
        };

        let create_addressindex_and_unspentoutpoint_store =
            |index| Store::import(keyspace_ref, path, &format!("a2u{}", index), version, None);

        Ok(Self {
            keyspace: keyspace.clone(),

            height_to_coinbase_tag: Store::import(keyspace_ref, path, "h2c", version, None)?,
            addressbyteshash_to_typeindex: Store::import(keyspace_ref, path, "a2t", version, None)?,
            blockhashprefix_to_height: Store::import(keyspace_ref, path, "b2h", version, None)?,
            txidprefix_to_txindex: Store::import(keyspace_ref, path, "t2t", version, None)?,
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
        .into_par_iter() // Changed from par_iter_mut()
        .chain(
            self.addresstype_to_typeindex_and_txindex
                .par_iter_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addresstype_to_typeindex_and_unspentoutpoint
                .par_iter_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .try_for_each(|store| store.commit(height))?;

        self.keyspace
            .persist(PersistMode::SyncAll)
            .map_err(|e| e.into())
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
                .iter_at(starting_indexes.height)
                .for_each(|(_, v)| {
                    let blockhashprefix = BlockHashPrefix::from(v.into_owned());
                    self.blockhashprefix_to_height.remove(blockhashprefix);
                });

            (starting_indexes.height.to_usize()..vecs.height_to_blockhash.len())
                .map(Height::from)
                .for_each(|h| {
                    self.height_to_coinbase_tag.remove(h);
                });

            if let Some(mut index) = vecs
                .height_to_first_p2pk65addressindex
                .iter()
                .get(starting_indexes.height)
                .map(Cow::into_owned)
            {
                let mut p2pk65addressindex_to_p2pk65bytes_iter =
                    vecs.p2pk65addressindex_to_p2pk65bytes.iter();

                while let Some(typedbytes) = p2pk65addressindex_to_p2pk65bytes_iter
                    .get(index)
                    .map(Cow::into_owned)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2pk33addressindex
                .iter()
                .get(starting_indexes.height)
                .map(Cow::into_owned)
            {
                let mut p2pk33addressindex_to_p2pk33bytes_iter =
                    vecs.p2pk33addressindex_to_p2pk33bytes.iter();

                while let Some(typedbytes) = p2pk33addressindex_to_p2pk33bytes_iter
                    .get(index)
                    .map(Cow::into_owned)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2pkhaddressindex
                .iter()
                .get(starting_indexes.height)
                .map(Cow::into_owned)
            {
                let mut p2pkhaddressindex_to_p2pkhbytes_iter =
                    vecs.p2pkhaddressindex_to_p2pkhbytes.iter();

                while let Some(typedbytes) = p2pkhaddressindex_to_p2pkhbytes_iter
                    .get(index)
                    .map(Cow::into_owned)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2shaddressindex
                .iter()
                .get(starting_indexes.height)
                .map(Cow::into_owned)
            {
                let mut p2shaddressindex_to_p2shbytes_iter =
                    vecs.p2shaddressindex_to_p2shbytes.iter();

                while let Some(typedbytes) = p2shaddressindex_to_p2shbytes_iter
                    .get(index)
                    .map(Cow::into_owned)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2traddressindex
                .iter()
                .get(starting_indexes.height)
                .map(Cow::into_owned)
            {
                let mut p2traddressindex_to_p2trbytes_iter =
                    vecs.p2traddressindex_to_p2trbytes.iter();

                while let Some(typedbytes) = p2traddressindex_to_p2trbytes_iter
                    .get(index)
                    .map(Cow::into_owned)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2wpkhaddressindex
                .iter()
                .get(starting_indexes.height)
                .map(Cow::into_owned)
            {
                let mut p2wpkhaddressindex_to_p2wpkhbytes_iter =
                    vecs.p2wpkhaddressindex_to_p2wpkhbytes.iter();

                while let Some(typedbytes) = p2wpkhaddressindex_to_p2wpkhbytes_iter
                    .get(index)
                    .map(Cow::into_owned)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2wshaddressindex
                .iter()
                .get(starting_indexes.height)
                .map(Cow::into_owned)
            {
                let mut p2wshaddressindex_to_p2wshbytes_iter =
                    vecs.p2wshaddressindex_to_p2wshbytes.iter();

                while let Some(typedbytes) = p2wshaddressindex_to_p2wshbytes_iter
                    .get(index)
                    .map(Cow::into_owned)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from(&bytes);
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2aaddressindex
                .iter()
                .get(starting_indexes.height)
                .map(Cow::into_owned)
            {
                let mut p2aaddressindex_to_p2abytes_iter = vecs.p2aaddressindex_to_p2abytes.iter();

                while let Some(typedbytes) = p2aaddressindex_to_p2abytes_iter
                    .get(index)
                    .map(Cow::into_owned)
                {
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
                .iter_at(starting_indexes.txindex)
                .for_each(|(txindex, txid)| {
                    let txidprefix = TxidPrefix::from(&txid.into_owned());

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
            vecs.txoutindex_to_outputtype
                .iter_at(starting_indexes.txoutindex)
                .zip(
                    vecs.txoutindex_to_typeindex
                        .iter_at(starting_indexes.txoutindex),
                )
                .filter(|((_, outputtype), _)| outputtype.is_address())
                .for_each(|((txoutindex, outputtype), (_, typeindex))| {
                    let outputtype = outputtype.into_owned();
                    let typeindex = typeindex.into_owned();

                    let txindex = vecs
                        .txoutindex_to_txindex
                        .iter()
                        .get(txoutindex)
                        .unwrap()
                        .into_owned();

                    let vout = Vout::from(
                        txoutindex.to_usize()
                            - vecs
                                .txindex_to_first_txoutindex
                                .iter()
                                .get(txindex)
                                .unwrap()
                                .into_owned()
                                .to_usize(),
                    );
                    let outpoint = OutPoint::new(txindex, vout);

                    self.addresstype_to_typeindex_and_unspentoutpoint
                        .get_mut(outputtype)
                        .unwrap()
                        .remove(TypeIndexAndOutPoint::from((typeindex, outpoint)));
                });

            // Add back outputs that were spent after the rollback point
            vecs.txinindex_to_outpoint
                .iter_at(starting_indexes.txinindex)
                .for_each(|(_, outpoint)| {
                    let outpoint = outpoint.into_owned();

                    if outpoint.is_coinbase() {
                        return;
                    }

                    let txindex = outpoint.txindex();
                    let vout = outpoint.vout();

                    // Calculate txoutindex from txindex and vout
                    let txoutindex = vecs
                        .txindex_to_first_txoutindex
                        .iter()
                        .get(txindex)
                        .unwrap()
                        .into_owned()
                        + vout;

                    // Only process if this output was created before the rollback point
                    if txoutindex < starting_indexes.txoutindex {
                        let outputtype = vecs
                            .txoutindex_to_outputtype
                            .iter()
                            .get(txoutindex)
                            .unwrap()
                            .into_owned();

                        if outputtype.is_address() {
                            let typeindex = vecs
                                .txoutindex_to_typeindex
                                .iter()
                                .get(txoutindex)
                                .unwrap()
                                .into_owned();

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
