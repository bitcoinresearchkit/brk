use std::{borrow::Cow, fs, path::Path, thread};

use brk_error::Result;
use brk_grouper::ByAddressType;
use brk_store::{AnyStore, Store};
use brk_structs::{
    AddressBytes, AddressBytesHash, BlockHashPrefix, Height, OutputType, StoredString, TxIndex,
    TxOutIndex, TxidPrefix, TypeIndex, TypeIndexWithOutputindex, Unit, Version,
};
use fjall::{PersistMode, TransactionalKeyspace};
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
    // Do address_type_to_typeindex_with_txindex_to_vin_and_vout_instead ?
    // and should vin be txoutindex or txinindex?
    pub addresstype_to_typeindex_with_txoutindex:
        ByAddressType<Store<TypeIndexWithOutputindex, Unit>>,
}

impl Stores {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        let path = parent.join("stores");

        fs::create_dir_all(&path)?;

        let keyspace = match brk_store::open_keyspace(&path) {
            Ok(keyspace) => keyspace,
            Err(_) => {
                fs::remove_dir_all(&path)?;
                return Self::forced_import(&path, version);
            }
        };

        thread::scope(|scope| {
            let addressbyteshash_to_typeindex = scope.spawn(|| {
                Store::import(
                    &keyspace,
                    &path,
                    "addressbyteshash_to_typeindex",
                    version,
                    None,
                )
            });
            let blockhashprefix_to_height = scope.spawn(|| {
                Store::import(&keyspace, &path, "blockhashprefix_to_height", version, None)
            });
            let txidprefix_to_txindex = scope
                .spawn(|| Store::import(&keyspace, &path, "txidprefix_to_txindex", version, None));
            let p2aaddressindex_with_txoutindex = scope.spawn(|| {
                Store::import(
                    &keyspace,
                    &path,
                    "p2aaddressindex_with_txoutindex",
                    version,
                    Some(false),
                )
            });
            let p2pk33addressindex_with_txoutindex = scope.spawn(|| {
                Store::import(
                    &keyspace,
                    &path,
                    "p2pk33addressindex_with_txoutindex",
                    version,
                    Some(false),
                )
            });
            let p2pk65addressindex_with_txoutindex = scope.spawn(|| {
                Store::import(
                    &keyspace,
                    &path,
                    "p2pk65addressindex_with_txoutindex",
                    version,
                    Some(false),
                )
            });
            let p2pkhaddressindex_with_txoutindex = scope.spawn(|| {
                Store::import(
                    &keyspace,
                    &path,
                    "p2pkhaddressindex_with_txoutindex",
                    version,
                    Some(false),
                )
            });
            let p2shaddressindex_with_txoutindex = scope.spawn(|| {
                Store::import(
                    &keyspace,
                    &path,
                    "p2shaddressindex_with_txoutindex",
                    version,
                    Some(false),
                )
            });
            let p2traddressindex_with_txoutindex = scope.spawn(|| {
                Store::import(
                    &keyspace,
                    &path,
                    "p2traddressindex_with_txoutindex",
                    version,
                    Some(false),
                )
            });
            let p2wpkhaddressindex_with_txoutindex = scope.spawn(|| {
                Store::import(
                    &keyspace,
                    &path,
                    "p2wpkhaddressindex_with_txoutindex",
                    version,
                    Some(false),
                )
            });
            let p2wshaddressindex_with_txoutindex = scope.spawn(|| {
                Store::import(
                    &keyspace,
                    &path,
                    "p2wshaddressindex_with_txoutindex",
                    version,
                    Some(false),
                )
            });

            let height_to_coinbase_tag =
                Store::import(&keyspace, &path, "height_to_coinbase_tag", version, None)?;

            Ok(Self {
                keyspace: keyspace.clone(),

                height_to_coinbase_tag,
                addressbyteshash_to_typeindex: addressbyteshash_to_typeindex.join().unwrap()?,
                blockhashprefix_to_height: blockhashprefix_to_height.join().unwrap()?,
                txidprefix_to_txindex: txidprefix_to_txindex.join().unwrap()?,
                addresstype_to_typeindex_with_txoutindex: ByAddressType {
                    p2pk65: p2pk65addressindex_with_txoutindex.join().unwrap()?,
                    p2pk33: p2pk33addressindex_with_txoutindex.join().unwrap()?,
                    p2pkh: p2pkhaddressindex_with_txoutindex.join().unwrap()?,
                    p2sh: p2shaddressindex_with_txoutindex.join().unwrap()?,
                    p2wpkh: p2wpkhaddressindex_with_txoutindex.join().unwrap()?,
                    p2wsh: p2wshaddressindex_with_txoutindex.join().unwrap()?,
                    p2tr: p2traddressindex_with_txoutindex.join().unwrap()?,
                    p2a: p2aaddressindex_with_txoutindex.join().unwrap()?,
                },
            })
        })
    }

    pub fn starting_height(&self) -> Height {
        self.as_slice()
            .into_iter()
            .map(|store| {
                // let height =
                store.height().map(Height::incremented).unwrap_or_default()
                // dbg!((height, store.name()));
            })
            .min()
            .unwrap()
    }

    pub fn commit(&mut self, height: Height) -> Result<()> {
        self.as_mut_slice()
            .into_par_iter()
            .try_for_each(|store| store.commit(height))?;

        self.keyspace
            .persist(PersistMode::SyncAll)
            .map_err(|e| e.into())
    }

    fn as_slice(&self) -> [&(dyn AnyStore + Send + Sync); 12] {
        [
            &self.addressbyteshash_to_typeindex,
            &self.addresstype_to_typeindex_with_txoutindex.p2a,
            &self.addresstype_to_typeindex_with_txoutindex.p2pk33,
            &self.addresstype_to_typeindex_with_txoutindex.p2pk65,
            &self.addresstype_to_typeindex_with_txoutindex.p2pkh,
            &self.addresstype_to_typeindex_with_txoutindex.p2sh,
            &self.addresstype_to_typeindex_with_txoutindex.p2tr,
            &self.addresstype_to_typeindex_with_txoutindex.p2wpkh,
            &self.addresstype_to_typeindex_with_txoutindex.p2wsh,
            &self.blockhashprefix_to_height,
            &self.height_to_coinbase_tag,
            &self.txidprefix_to_txindex,
        ]
    }

    fn as_mut_slice(&mut self) -> [&mut (dyn AnyStore + Send + Sync); 12] {
        [
            &mut self.addressbyteshash_to_typeindex,
            &mut self.addresstype_to_typeindex_with_txoutindex.p2a,
            &mut self.addresstype_to_typeindex_with_txoutindex.p2pk33,
            &mut self.addresstype_to_typeindex_with_txoutindex.p2pk65,
            &mut self.addresstype_to_typeindex_with_txoutindex.p2pkh,
            &mut self.addresstype_to_typeindex_with_txoutindex.p2sh,
            &mut self.addresstype_to_typeindex_with_txoutindex.p2tr,
            &mut self.addresstype_to_typeindex_with_txoutindex.p2wpkh,
            &mut self.addresstype_to_typeindex_with_txoutindex.p2wsh,
            &mut self.blockhashprefix_to_height,
            &mut self.height_to_coinbase_tag,
            &mut self.txidprefix_to_txindex,
        ]
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
                .addresstype_to_typeindex_with_txoutindex
                .p2a
                .is_empty()?
            && self
                .addresstype_to_typeindex_with_txoutindex
                .p2pk33
                .is_empty()?
            && self
                .addresstype_to_typeindex_with_txoutindex
                .p2pk65
                .is_empty()?
            && self
                .addresstype_to_typeindex_with_txoutindex
                .p2pkh
                .is_empty()?
            && self
                .addresstype_to_typeindex_with_txoutindex
                .p2sh
                .is_empty()?
            && self
                .addresstype_to_typeindex_with_txoutindex
                .p2tr
                .is_empty()?
            && self
                .addresstype_to_typeindex_with_txoutindex
                .p2wpkh
                .is_empty()?
            && self
                .addresstype_to_typeindex_with_txoutindex
                .p2wsh
                .is_empty()?
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

            (starting_indexes.height.unwrap_to_usize()..vecs.height_to_blockhash.len())
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
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2PK65));
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
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2PK33));
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
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2PKH));
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
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2SH));
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
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2TR));
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
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2WPKH));
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
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2WSH));
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
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2A));
                    self.addressbyteshash_to_typeindex.remove(hash);
                    index.increment();
                }
            }
        } else {
            self.blockhashprefix_to_height.reset()?;
            self.addressbyteshash_to_typeindex.reset()?;
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
            self.txidprefix_to_txindex.reset()?;
        }

        if starting_indexes.txoutindex != TxOutIndex::ZERO {
            let mut txoutindex_to_typeindex_iter = vecs.txoutindex_to_typeindex.into_iter();
            vecs.txoutindex_to_outputtype
                .iter_at(starting_indexes.txoutindex)
                .filter(|(_, outputtype)| outputtype.is_address())
                .for_each(|(txoutindex, outputtype)| {
                    let outputtype = outputtype.into_owned();

                    let typeindex = txoutindex_to_typeindex_iter.unwrap_get_inner(txoutindex);

                    self.addresstype_to_typeindex_with_txoutindex
                        .get_mut(outputtype)
                        .unwrap()
                        .remove(TypeIndexWithOutputindex::from((typeindex, txoutindex)));
                });
        } else {
            self.addresstype_to_typeindex_with_txoutindex.p2a.reset()?;
            self.addresstype_to_typeindex_with_txoutindex
                .p2pk33
                .reset()?;
            self.addresstype_to_typeindex_with_txoutindex
                .p2pk65
                .reset()?;
            self.addresstype_to_typeindex_with_txoutindex
                .p2pkh
                .reset()?;
            self.addresstype_to_typeindex_with_txoutindex.p2sh.reset()?;
            self.addresstype_to_typeindex_with_txoutindex.p2tr.reset()?;
            self.addresstype_to_typeindex_with_txoutindex
                .p2wpkh
                .reset()?;
            self.addresstype_to_typeindex_with_txoutindex
                .p2wsh
                .reset()?;
        }

        self.commit(starting_indexes.height.decremented().unwrap_or_default())?;

        Ok(())
    }
}
