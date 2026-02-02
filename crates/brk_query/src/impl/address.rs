use std::str::FromStr;

use bitcoin::{Network, PublicKey, ScriptBuf};
use brk_error::{Error, Result};
use brk_types::{
    Address, AddressBytes, AddressChainStats, AddressHash, AddressIndexOutPoint,
    AddressIndexTxIndex, AddressStats, AnyAddressDataIndexEnum, OutputType, Sats, TxIndex,
    TxStatus, Txid, TypeIndex, Unit, Utxo, Vout,
};
use vecdb::TypedVecIterator;

use crate::Query;

/// Maximum number of mempool txids to return
const MAX_MEMPOOL_TXIDS: usize = 50;

impl Query {
    pub fn address(&self, address: Address) -> Result<AddressStats> {
        let indexer = self.indexer();
        let computer = self.computer();
        let stores = &indexer.stores;

        let script = if let Ok(address) = bitcoin::Address::from_str(&address) {
            if !address.is_valid_for_network(Network::Bitcoin) {
                return Err(Error::InvalidNetwork);
            }
            let address = address.assume_checked();
            address.script_pubkey()
        } else if let Ok(pubkey) = PublicKey::from_str(&address) {
            ScriptBuf::new_p2pk(&pubkey)
        } else {
            return Err(Error::InvalidAddress);
        };

        dbg!(&script);

        let outputtype = OutputType::from(&script);
        dbg!(outputtype);
        let Ok(bytes) = AddressBytes::try_from((&script, outputtype)) else {
            return Err(Error::InvalidAddress);
        };
        let addresstype = outputtype;
        let hash = AddressHash::from(&bytes);
        dbg!(hash);

        let Ok(Some(type_index)) = stores
            .addresstype_to_addresshash_to_addressindex
            .get_unwrap(addresstype)
            .get(&hash)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownAddress);
        };

        let any_address_index = computer
            .distribution
            .any_address_indexes
            .get_once(outputtype, type_index)?;

        let address_data = match any_address_index.to_enum() {
            AnyAddressDataIndexEnum::Funded(index) => computer
                .distribution
                .addresses_data
                .funded
                .iter()?
                .get_unwrap(index),
            AnyAddressDataIndexEnum::Empty(index) => computer
                .distribution
                .addresses_data
                .empty
                .iter()?
                .get_unwrap(index)
                .into(),
        };

        Ok(AddressStats {
            address,
            chain_stats: AddressChainStats {
                type_index,
                funded_txo_count: address_data.funded_txo_count,
                funded_txo_sum: address_data.received,
                spent_txo_count: address_data.spent_txo_count,
                spent_txo_sum: address_data.sent,
                tx_count: address_data.tx_count,
            },
            mempool_stats: self.mempool().map(|mempool| {
                mempool
                    .get_addresses()
                    .get(&bytes)
                    .map(|(stats, _)| stats)
                    .cloned()
                    .unwrap_or_default()
            }),
        })
    }

    pub fn address_txids(
        &self,
        address: Address,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Txid>> {
        let indexer = self.indexer();
        let stores = &indexer.stores;

        let (outputtype, type_index) = self.resolve_address(&address)?;

        let store = stores
            .addresstype_to_addressindex_and_txindex
            .get(outputtype)
            .unwrap();

        let prefix = u32::from(type_index).to_be_bytes();

        let after_txindex = if let Some(after_txid) = after_txid {
            let txindex = stores
                .txidprefix_to_txindex
                .get(&after_txid.into())
                .map_err(|_| Error::UnknownTxid)?
                .ok_or(Error::UnknownTxid)?
                .into_owned();
            Some(txindex)
        } else {
            None
        };

        let txindices: Vec<TxIndex> = store
            .prefix(prefix)
            .rev()
            .filter(|(key, _): &(AddressIndexTxIndex, Unit)| {
                if let Some(after) = after_txindex {
                    key.txindex() < after
                } else {
                    true
                }
            })
            .take(limit)
            .map(|(key, _)| key.txindex())
            .collect();

        let mut txindex_to_txid_iter = indexer.vecs.transactions.txid.iter()?;
        let txids: Vec<Txid> = txindices
            .into_iter()
            .map(|txindex| txindex_to_txid_iter.get_unwrap(txindex))
            .collect();

        Ok(txids)
    }

    pub fn address_utxos(&self, address: Address) -> Result<Vec<Utxo>> {
        let indexer = self.indexer();
        let stores = &indexer.stores;
        let vecs = &indexer.vecs;

        let (outputtype, type_index) = self.resolve_address(&address)?;

        let store = stores
            .addresstype_to_addressindex_and_unspentoutpoint
            .get(outputtype)
            .unwrap();

        let prefix = u32::from(type_index).to_be_bytes();

        let outpoints: Vec<(TxIndex, Vout)> = store
            .prefix(prefix)
            .map(|(key, _): (AddressIndexOutPoint, Unit)| (key.txindex(), key.vout()))
            .collect();

        let mut txindex_to_txid_iter = vecs.transactions.txid.iter()?;
        let mut txindex_to_height_iter = vecs.transactions.height.iter()?;
        let mut txindex_to_first_txoutindex_iter = vecs.transactions.first_txoutindex.iter()?;
        let mut txoutindex_to_value_iter = vecs.outputs.value.iter()?;
        let mut height_to_blockhash_iter = vecs.blocks.blockhash.iter()?;
        let mut height_to_timestamp_iter = vecs.blocks.timestamp.iter()?;

        let utxos: Vec<Utxo> = outpoints
            .into_iter()
            .map(|(txindex, vout)| {
                let txid: Txid = txindex_to_txid_iter.get_unwrap(txindex);
                let height = txindex_to_height_iter.get_unwrap(txindex);
                let first_txoutindex = txindex_to_first_txoutindex_iter.get_unwrap(txindex);
                let txoutindex = first_txoutindex + vout;
                let value: Sats = txoutindex_to_value_iter.get_unwrap(txoutindex);
                let block_hash = height_to_blockhash_iter.get_unwrap(height);
                let block_time = height_to_timestamp_iter.get_unwrap(height);

                Utxo {
                    txid,
                    vout,
                    status: TxStatus {
                        confirmed: true,
                        block_height: Some(height),
                        block_hash: Some(block_hash),
                        block_time: Some(block_time),
                    },
                    value,
                }
            })
            .collect();

        Ok(utxos)
    }

    pub fn address_mempool_txids(&self, address: Address) -> Result<Vec<Txid>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;

        let bytes = AddressBytes::from_str(&address)?;
        let addresses = mempool.get_addresses();

        let txids: Vec<Txid> = addresses
            .get(&bytes)
            .map(|(_, txids)| txids.iter().take(MAX_MEMPOOL_TXIDS).cloned().collect())
            .unwrap_or_default();

        Ok(txids)
    }

    /// Resolve an address string to its output type and type_index
    fn resolve_address(&self, address: &Address) -> Result<(OutputType, TypeIndex)> {
        let stores = &self.indexer().stores;

        let bytes = AddressBytes::from_str(address)?;
        let outputtype = OutputType::from(&bytes);
        let hash = AddressHash::from(&bytes);

        let Ok(Some(type_index)) = stores
            .addresstype_to_addresshash_to_addressindex
            .get(outputtype)
            .unwrap()
            .get(&hash)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownAddress);
        };

        Ok((outputtype, type_index))
    }
}
