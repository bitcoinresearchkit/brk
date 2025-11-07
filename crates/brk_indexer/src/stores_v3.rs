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
    pub addresstype_to_addressindex_and_unspentoutpoint:
        Store<AddressTypeAddressIndexOutPoint, Unit>,
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
            addresstype_to_addressindex_and_unspentoutpoint: Store::import(
                database_ref,
                path,
                "addresstype_to_addressindex_and_unspentoutpoint",
                version,
                Mode3::Any,
                Kind3::Vec,
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
            &mut self.addresstype_to_addressindex_and_txindex,
            &mut self.addresstype_to_addressindex_and_unspentoutpoint,
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
            &self.addresstype_to_addressindex_and_unspentoutpoint,
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
            && self
                .addresstype_to_addressindex_and_unspentoutpoint
                .is_empty()?
        {
            return Ok(());
        }

        Ok(())
    }
}
