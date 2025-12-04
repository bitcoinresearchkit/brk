use std::{fs, path::Path};

use brk_error::Result;
use brk_grouper::ByAddressType;
use brk_store::{AnyStore, Mode, StoreFjallV2 as Store, Type};
use brk_types::{
    AddressHash, AddressIndexOutPoint, AddressIndexTxIndex, BlockHashPrefix, Height, OutPoint,
    OutputType, StoredString, TxIndex, TxOutIndex, TxidPrefix, TypeIndex, Unit, Version, Vout,
};
use fjall2::{CompressionType as Compression, PersistMode, TransactionalKeyspace};
use rayon::prelude::*;
use vecdb::{AnyVec, TypedVecIterator, VecIndex, VecIterator};

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
                .values()
                .map(|s| s as &dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_txindex
                .values()
                .map(|s| s as &dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_unspentoutpoint
                .values()
                .map(|s| s as &dyn AnyStore),
        )
        .map(|store| store.height().map(Height::incremented).unwrap_or_default())
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
                .par_values_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_txindex
                .par_values_mut()
                .map(|s| s as &mut dyn AnyStore),
        )
        .chain(
            self.addresstype_to_addressindex_and_unspentoutpoint
                .par_values_mut()
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
            .persist(PersistMode::SyncData)
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
                .values()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
            && self
                .addresstype_to_addressindex_and_txindex
                .values()
                .try_fold(true, |acc, s| s.is_empty().map(|empty| acc && empty))?
            && self
                .addresstype_to_addressindex_and_unspentoutpoint
                .values()
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

            // Remove address hashes for all address types starting from rollback height
            for address_type in [
                OutputType::P2PK65,
                OutputType::P2PK33,
                OutputType::P2PKH,
                OutputType::P2SH,
                OutputType::P2WPKH,
                OutputType::P2WSH,
                OutputType::P2TR,
                OutputType::P2A,
            ] {
                for hash in vecs.iter_address_hashes_from(address_type, starting_indexes.height)? {
                    self.addresstype_to_addresshash_to_addressindex
                        .get_mut_unwrap(address_type)
                        .remove(hash);
                }
            }
        } else {
            unreachable!();
        }

        if starting_indexes.txindex != TxIndex::ZERO {
            vecs.txindex_to_txid
                .iter()?
                .enumerate()
                .skip(starting_indexes.txindex.to_usize())
                .for_each(|(txindex, txid)| {
                    let txindex = TxIndex::from(txindex);
                    let txidprefix = TxidPrefix::from(&txid);

                    let is_known_dup = crate::DUPLICATE_TXID_PREFIXES
                        .iter()
                        .any(|(dup_prefix, dup_txindex)| {
                            txindex == *dup_txindex && txidprefix == *dup_prefix
                        });

                    if !is_known_dup {
                        self.txidprefix_to_txindex.remove(txidprefix);
                    }
                });
        } else {
            unreachable!();
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
        }

        self.commit(starting_indexes.height.decremented().unwrap_or_default())?;

        Ok(())
    }
}
