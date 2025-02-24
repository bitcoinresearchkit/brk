use std::{path::Path, thread};

use brk_core::{AddressHash, Addressbytes, Addressindex, Addresstype, BlockHashPrefix, Height, TxidPrefix, Txindex};
use storable_vec::{CACHED_GETS, Value, Version};

use crate::Indexes;

mod base;
mod meta;

pub use base::*;
pub use meta::*;

use super::StorableVecs;

pub struct Fjalls {
    pub addresshash_to_addressindex: Store<AddressHash, Addressindex>,
    pub blockhash_prefix_to_height: Store<BlockHashPrefix, Height>,
    pub txid_prefix_to_txindex: Store<TxidPrefix, Txindex>,
}

impl Fjalls {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        let addresshash_to_addressindex = Store::import(&path.join("addresshash_to_addressindex"), Version::from(1))?;
        let blockhash_prefix_to_height = Store::import(&path.join("blockhash_prefix_to_height"), Version::from(1))?;
        let txid_prefix_to_txindex = Store::import(&path.join("txid_prefix_to_txindex"), Version::from(1))?;

        Ok(Self {
            addresshash_to_addressindex,
            blockhash_prefix_to_height,
            txid_prefix_to_txindex,
        })
    }

    pub fn rollback(&mut self, vecs: &StorableVecs<CACHED_GETS>, starting_indexes: &Indexes) -> color_eyre::Result<()> {
        vecs.height_to_blockhash
            .iter_from(starting_indexes.height, |(_, blockhash)| {
                let blockhash = blockhash.as_ref();
                let blockhash_prefix = BlockHashPrefix::from(blockhash);
                self.blockhash_prefix_to_height.remove(blockhash_prefix);
                Ok(())
            })?;

        vecs.txindex_to_txid.iter_from(starting_indexes.txindex, |(_, txid)| {
            let txid = txid.as_ref();
            let txid_prefix = TxidPrefix::from(txid);
            self.txid_prefix_to_txindex.remove(txid_prefix);
            Ok(())
        })?;

        vecs.height_to_first_p2pk65index
            .iter_from(starting_indexes.height, |(_, index)| {
                if let Some(typedbytes) = vecs
                    .p2pk65index_to_p2pk65addressbytes
                    .get(index.into_inner())?
                    .map(Value::into_inner)
                {
                    let bytes = Addressbytes::from(typedbytes);
                    let hash = AddressHash::from((&bytes, Addresstype::P2PK65));
                    self.addresshash_to_addressindex.remove(hash);
                }
                Ok(())
            })?;

        vecs.height_to_first_p2pk33index
            .iter_from(starting_indexes.height, |(_, index)| {
                if let Some(typedbytes) = vecs
                    .p2pk33index_to_p2pk33addressbytes
                    .get(index.into_inner())?
                    .map(Value::into_inner)
                {
                    let bytes = Addressbytes::from(typedbytes);
                    let hash = AddressHash::from((&bytes, Addresstype::P2PK33));
                    self.addresshash_to_addressindex.remove(hash);
                }
                Ok(())
            })?;

        vecs.height_to_first_p2pkhindex
            .iter_from(starting_indexes.height, |(_, index)| {
                if let Some(typedbytes) = vecs
                    .p2pkhindex_to_p2pkhaddressbytes
                    .get(index.into_inner())?
                    .map(Value::into_inner)
                {
                    let bytes = Addressbytes::from(typedbytes);
                    let hash = AddressHash::from((&bytes, Addresstype::P2PKH));
                    self.addresshash_to_addressindex.remove(hash);
                }
                Ok(())
            })?;

        vecs.height_to_first_p2shindex
            .iter_from(starting_indexes.height, |(_, index)| {
                if let Some(typedbytes) = vecs
                    .p2shindex_to_p2shaddressbytes
                    .get(index.into_inner())?
                    .map(Value::into_inner)
                {
                    let bytes = Addressbytes::from(typedbytes);
                    let hash = AddressHash::from((&bytes, Addresstype::P2SH));
                    self.addresshash_to_addressindex.remove(hash);
                }
                Ok(())
            })?;

        vecs.height_to_first_p2trindex
            .iter_from(starting_indexes.height, |(_, index)| {
                if let Some(typedbytes) = vecs
                    .p2trindex_to_p2traddressbytes
                    .get(index.into_inner())?
                    .map(Value::into_inner)
                {
                    let bytes = Addressbytes::from(typedbytes);
                    let hash = AddressHash::from((&bytes, Addresstype::P2TR));
                    self.addresshash_to_addressindex.remove(hash);
                }
                Ok(())
            })?;

        vecs.height_to_first_p2wpkhindex
            .iter_from(starting_indexes.height, |(_, index)| {
                if let Some(typedbytes) = vecs
                    .p2wpkhindex_to_p2wpkhaddressbytes
                    .get(index.into_inner())?
                    .map(Value::into_inner)
                {
                    let bytes = Addressbytes::from(typedbytes);
                    let hash = AddressHash::from((&bytes, Addresstype::P2WPKH));
                    self.addresshash_to_addressindex.remove(hash);
                }
                Ok(())
            })?;

        vecs.height_to_first_p2wshindex
            .iter_from(starting_indexes.height, |(_, index)| {
                if let Some(typedbytes) = vecs
                    .p2wshindex_to_p2wshaddressbytes
                    .get(index.into_inner())?
                    .map(Value::into_inner)
                {
                    let bytes = Addressbytes::from(typedbytes);
                    let hash = AddressHash::from((&bytes, Addresstype::P2WSH));
                    self.addresshash_to_addressindex.remove(hash);
                }
                Ok(())
            })?;

        self.commit(starting_indexes.height.decremented())?;

        Ok(())
    }

    pub fn starting_height(&self) -> Height {
        [
            self.addresshash_to_addressindex.height(),
            self.blockhash_prefix_to_height.height(),
            self.txid_prefix_to_txindex.height(),
        ]
        .into_iter()
        .map(|height| height.map(Height::incremented).unwrap_or_default())
        .min()
        .unwrap()
    }

    pub fn commit(&mut self, height: Height) -> fjall::Result<()> {
        thread::scope(|scope| {
            let addresshash_to_addressindex_commit_handle =
                scope.spawn(|| self.addresshash_to_addressindex.commit(height));
            let blockhash_prefix_to_height_commit_handle =
                scope.spawn(|| self.blockhash_prefix_to_height.commit(height));
            let txid_prefix_to_txindex_commit_handle = scope.spawn(|| self.txid_prefix_to_txindex.commit(height));

            addresshash_to_addressindex_commit_handle.join().unwrap()?;
            blockhash_prefix_to_height_commit_handle.join().unwrap()?;
            txid_prefix_to_txindex_commit_handle.join().unwrap()?;

            Ok(())
        })
    }
}
