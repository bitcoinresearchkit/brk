use std::{fs, path::Path, thread};

use brk_core::{
    AddressBytes, AddressBytesHash, BlockHashPrefix, Height, OutputType, OutputTypeIndex, TxIndex,
    TxidPrefix,
};
use brk_vec::{Value, Version};
use fjall::{PersistMode, TransactionalKeyspace};

use crate::Indexes;

mod base;
mod meta;

pub use base::*;
pub use meta::*;

use super::Vecs;

#[derive(Clone)]
pub struct Stores {
    pub keyspace: TransactionalKeyspace,
    pub addressbyteshash_to_outputtypeindex: Store<AddressBytesHash, OutputTypeIndex>,
    pub blockhashprefix_to_height: Store<BlockHashPrefix, Height>,
    pub txidprefix_to_txindex: Store<TxidPrefix, TxIndex>,
}

impl Stores {
    pub fn forced_import(path: &Path) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        let keyspace = match Self::open_keyspace(path) {
            Ok(keyspace) => keyspace,
            Err(_) => {
                fs::remove_dir_all(path)?;
                return Self::forced_import(path);
            }
        };

        thread::scope(|scope| {
            let addressbyteshash_to_outputtypeindex = scope.spawn(|| {
                Store::import(
                    keyspace.clone(),
                    path,
                    "addressbyteshash_to_outputtypeindex",
                    Version::ZERO,
                )
            });
            let blockhashprefix_to_height = scope.spawn(|| {
                Store::import(
                    keyspace.clone(),
                    path,
                    "blockhashprefix_to_height",
                    Version::ZERO,
                )
            });
            let txidprefix_to_txindex = scope.spawn(|| {
                Store::import(
                    keyspace.clone(),
                    path,
                    "txidprefix_to_txindex",
                    Version::ZERO,
                )
            });

            Ok(Self {
                keyspace: keyspace.clone(),
                addressbyteshash_to_outputtypeindex: addressbyteshash_to_outputtypeindex
                    .join()
                    .unwrap()?,
                blockhashprefix_to_height: blockhashprefix_to_height.join().unwrap()?,
                txidprefix_to_txindex: txidprefix_to_txindex.join().unwrap()?,
            })
        })
    }

    pub fn rollback_if_needed(
        &mut self,
        vecs: &mut Vecs,
        starting_indexes: &Indexes,
    ) -> color_eyre::Result<()> {
        if self.addressbyteshash_to_outputtypeindex.is_empty()
            && self.blockhashprefix_to_height.is_empty()
            && self.txidprefix_to_txindex.is_empty()
        {
            return Ok(());
        }

        if starting_indexes.height != Height::ZERO {
            vecs.height_to_blockhash
                .iter_at(starting_indexes.height)
                .for_each(|(_, v)| {
                    let blockhashprefix = BlockHashPrefix::from(Value::into_inner(v));
                    self.blockhashprefix_to_height.remove(blockhashprefix);
                });

            if let Some(mut index) = vecs
                .height_to_first_p2pk65index
                .get(starting_indexes.height)?
                .map(Value::into_inner)
            {
                while let Some(typedbytes) = vecs
                    .p2pk65index_to_p2pk65bytes
                    .get(index)?
                    .map(Value::into_inner)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2PK65));
                    self.addressbyteshash_to_outputtypeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2pk33index
                .get(starting_indexes.height)?
                .map(Value::into_inner)
            {
                while let Some(typedbytes) = vecs
                    .p2pk33index_to_p2pk33bytes
                    .get(index)?
                    .map(Value::into_inner)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2PK33));
                    self.addressbyteshash_to_outputtypeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2pkhindex
                .get(starting_indexes.height)?
                .map(Value::into_inner)
            {
                while let Some(typedbytes) = vecs
                    .p2pkhindex_to_p2pkhbytes
                    .get(index)?
                    .map(Value::into_inner)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2PKH));
                    self.addressbyteshash_to_outputtypeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2shindex
                .get(starting_indexes.height)?
                .map(Value::into_inner)
            {
                while let Some(typedbytes) = vecs
                    .p2shindex_to_p2shbytes
                    .get(index)?
                    .map(Value::into_inner)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2SH));
                    self.addressbyteshash_to_outputtypeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2trindex
                .get(starting_indexes.height)?
                .map(Value::into_inner)
            {
                while let Some(typedbytes) = vecs
                    .p2trindex_to_p2trbytes
                    .get(index)?
                    .map(Value::into_inner)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2TR));
                    self.addressbyteshash_to_outputtypeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2wpkhindex
                .get(starting_indexes.height)?
                .map(Value::into_inner)
            {
                while let Some(typedbytes) = vecs
                    .p2wpkhindex_to_p2wpkhbytes
                    .get(index)?
                    .map(Value::into_inner)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2WPKH));
                    self.addressbyteshash_to_outputtypeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2wshindex
                .get(starting_indexes.height)?
                .map(Value::into_inner)
            {
                while let Some(typedbytes) = vecs
                    .p2wshindex_to_p2wshbytes
                    .get(index)?
                    .map(Value::into_inner)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2WSH));
                    self.addressbyteshash_to_outputtypeindex.remove(hash);
                    index.increment();
                }
            }

            if let Some(mut index) = vecs
                .height_to_first_p2aindex
                .get(starting_indexes.height)?
                .map(Value::into_inner)
            {
                while let Some(typedbytes) =
                    vecs.p2aindex_to_p2abytes.get(index)?.map(Value::into_inner)
                {
                    let bytes = AddressBytes::from(typedbytes);
                    let hash = AddressBytesHash::from((&bytes, OutputType::P2A));
                    self.addressbyteshash_to_outputtypeindex.remove(hash);
                    index.increment();
                }
            }
        } else {
            self.blockhashprefix_to_height.reset_partition()?;
            self.addressbyteshash_to_outputtypeindex.reset_partition()?;
        }

        if starting_indexes.txindex != TxIndex::ZERO {
            vecs.txindex_to_txid
                .iter_at(starting_indexes.txindex)
                .for_each(|(txindex, txid)| {
                    let txidprefix = TxidPrefix::from(&txid.into_inner());

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
            self.txidprefix_to_txindex.reset_partition()?;
        }

        self.commit(starting_indexes.height.decremented().unwrap_or_default())?;

        Ok(())
    }

    pub fn starting_height(&self) -> Height {
        [
            self.addressbyteshash_to_outputtypeindex.height(),
            self.blockhashprefix_to_height.height(),
            self.txidprefix_to_txindex.height(),
        ]
        .into_iter()
        .map(|height| height.map(Height::incremented).unwrap_or_default())
        .min()
        .unwrap()
    }

    pub fn commit(&mut self, height: Height) -> fjall::Result<()> {
        thread::scope(|scope| -> fjall::Result<()> {
            let addressbyteshash_to_outputtypeindex_commit_handle =
                scope.spawn(|| self.addressbyteshash_to_outputtypeindex.commit(height));
            let blockhashprefix_to_height_commit_handle =
                scope.spawn(|| self.blockhashprefix_to_height.commit(height));
            let txidprefix_to_txindex_commit_handle =
                scope.spawn(|| self.txidprefix_to_txindex.commit(height));

            addressbyteshash_to_outputtypeindex_commit_handle
                .join()
                .unwrap()?;
            blockhashprefix_to_height_commit_handle.join().unwrap()?;
            txidprefix_to_txindex_commit_handle.join().unwrap()?;

            Ok(())
        })?;

        self.keyspace.persist(PersistMode::SyncAll)
    }

    pub fn rotate_memtables(&self) {
        self.addressbyteshash_to_outputtypeindex.rotate_memtable();
        self.blockhashprefix_to_height.rotate_memtable();
        self.txidprefix_to_txindex.rotate_memtable();
    }

    fn open_keyspace(path: &Path) -> fjall::Result<TransactionalKeyspace> {
        fjall::Config::new(path.join("fjall"))
            .max_write_buffer_size(32 * 1024 * 1024)
            .open_transactional()
    }
}
