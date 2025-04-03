use std::{path::Path, thread};

use brk_core::{
    AddressHash, Addressbytes, Addressindex, Addresstype, BlockHashPrefix, Height, TxidPrefix,
    Txindex,
};
use brk_vec::{Value, Version};

use crate::Indexes;

mod base;
mod meta;

pub use base::*;
pub use meta::*;

use super::Vecs;

#[derive(Clone)]
pub struct Stores {
    pub addresshash_to_addressindex: Store<AddressHash, Addressindex>,
    pub blockhash_prefix_to_height: Store<BlockHashPrefix, Height>,
    pub txid_prefix_to_txindex: Store<TxidPrefix, Txindex>,
}

impl Stores {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        thread::scope(|scope| {
            let addresshash_to_addressindex = scope
                .spawn(|| Store::import(&path.join("addresshash_to_addressindex"), Version::ONE));
            let blockhash_prefix_to_height = scope
                .spawn(|| Store::import(&path.join("blockhash_prefix_to_height"), Version::ONE));
            let txid_prefix_to_txindex =
                scope.spawn(|| Store::import(&path.join("txid_prefix_to_txindex"), Version::ONE));

            Ok(Self {
                addresshash_to_addressindex: addresshash_to_addressindex.join().unwrap()?,
                blockhash_prefix_to_height: blockhash_prefix_to_height.join().unwrap()?,
                txid_prefix_to_txindex: txid_prefix_to_txindex.join().unwrap()?,
            })
        })
    }

    pub fn rollback_if_needed(
        &mut self,
        vecs: &mut Vecs,
        starting_indexes: &Indexes,
    ) -> color_eyre::Result<()> {
        vecs.height_to_blockhash
            .iter_from(starting_indexes.height, |(_, blockhash, ..)| {
                let blockhash_prefix = BlockHashPrefix::from(blockhash);
                self.blockhash_prefix_to_height.remove(blockhash_prefix);
                Ok(())
            })?;

        vecs.txindex_to_txid
            .iter_from(starting_indexes.txindex, |(_txindex, txid, ..)| {
                let txid_prefix = TxidPrefix::from(txid);
                self.txid_prefix_to_txindex.remove(txid_prefix);
                Ok(())
            })?;

        if let Some(index) = vecs
            .height_to_first_p2pk65index
            .get(starting_indexes.height)?
        {
            let mut index = index.into_inner();
            while let Some(typedbytes) = vecs
                .p2pk65index_to_p2pk65addressbytes
                .get(index)?
                .map(Value::into_inner)
            {
                let bytes = Addressbytes::from(typedbytes);
                let hash = AddressHash::from((&bytes, Addresstype::P2PK65));
                self.addresshash_to_addressindex.remove(hash);
                index.increment();
            }
        }

        if let Some(index) = vecs
            .height_to_first_p2pk33index
            .get(starting_indexes.height)?
        {
            let mut index = index.into_inner();
            while let Some(typedbytes) = vecs
                .p2pk33index_to_p2pk33addressbytes
                .get(index)?
                .map(Value::into_inner)
            {
                let bytes = Addressbytes::from(typedbytes);
                let hash = AddressHash::from((&bytes, Addresstype::P2PK33));
                self.addresshash_to_addressindex.remove(hash);
                index.increment();
            }
        }

        if let Some(index) = vecs
            .height_to_first_p2pkhindex
            .get(starting_indexes.height)?
        {
            let mut index = index.into_inner();
            while let Some(typedbytes) = vecs
                .p2pkhindex_to_p2pkhaddressbytes
                .get(index)?
                .map(Value::into_inner)
            {
                let bytes = Addressbytes::from(typedbytes);
                let hash = AddressHash::from((&bytes, Addresstype::P2PKH));
                self.addresshash_to_addressindex.remove(hash);
                index.increment();
            }
        }

        if let Some(index) = vecs
            .height_to_first_p2shindex
            .get(starting_indexes.height)?
        {
            let mut index = index.into_inner();
            while let Some(typedbytes) = vecs
                .p2shindex_to_p2shaddressbytes
                .get(index)?
                .map(Value::into_inner)
            {
                let bytes = Addressbytes::from(typedbytes);
                let hash = AddressHash::from((&bytes, Addresstype::P2SH));
                self.addresshash_to_addressindex.remove(hash);
                index.increment();
            }
        }

        if let Some(index) = vecs
            .height_to_first_p2trindex
            .get(starting_indexes.height)?
        {
            let mut index = index.into_inner();
            while let Some(typedbytes) = vecs
                .p2trindex_to_p2traddressbytes
                .get(index)?
                .map(Value::into_inner)
            {
                let bytes = Addressbytes::from(typedbytes);
                let hash = AddressHash::from((&bytes, Addresstype::P2TR));
                self.addresshash_to_addressindex.remove(hash);
                index.increment();
            }
        }

        if let Some(index) = vecs
            .height_to_first_p2wpkhindex
            .get(starting_indexes.height)?
        {
            let mut index = index.into_inner();
            while let Some(typedbytes) = vecs
                .p2wpkhindex_to_p2wpkhaddressbytes
                .get(index)?
                .map(Value::into_inner)
            {
                let bytes = Addressbytes::from(typedbytes);
                let hash = AddressHash::from((&bytes, Addresstype::P2WPKH));
                self.addresshash_to_addressindex.remove(hash);
                index.increment();
            }
        }

        if let Some(index) = vecs
            .height_to_first_p2wshindex
            .get(starting_indexes.height)?
        {
            let mut index = index.into_inner();
            while let Some(typedbytes) = vecs
                .p2wshindex_to_p2wshaddressbytes
                .get(index)?
                .map(Value::into_inner)
            {
                let bytes = Addressbytes::from(typedbytes);
                let hash = AddressHash::from((&bytes, Addresstype::P2WSH));
                self.addresshash_to_addressindex.remove(hash);
                index.increment();
            }
        }

        self.commit(starting_indexes.height.decremented().unwrap_or_default())?;

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
            let txid_prefix_to_txindex_commit_handle =
                scope.spawn(|| self.txid_prefix_to_txindex.commit(height));

            addresshash_to_addressindex_commit_handle.join().unwrap()?;
            blockhash_prefix_to_height_commit_handle.join().unwrap()?;
            txid_prefix_to_txindex_commit_handle.join().unwrap()?;

            Ok(())
        })
    }

    pub fn rotate_memtables(&self) {
        self.addresshash_to_addressindex.rotate_memtable();
        self.blockhash_prefix_to_height.rotate_memtable();
        self.txid_prefix_to_txindex.rotate_memtable();
    }
}
