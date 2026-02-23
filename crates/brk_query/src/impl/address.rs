use std::str::FromStr;

use bitcoin::{Network, PublicKey, ScriptBuf};
use brk_error::{Error, Result};
use brk_types::{
    Address, AddressBytes, AddressChainStats, AddressHash, AddressIndexOutPoint,
    AddressIndexTxIndex, AddressStats, AnyAddressDataIndexEnum, OutputType, Sats, TxIndex,
    TxStatus, Txid, TypeIndex, Unit, Utxo, Vout,
};
use vecdb::{ReadableVec, VecIndex};

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
                .reader()
                .get(usize::from(index)),
            AnyAddressDataIndexEnum::Empty(index) => computer
                .distribution
                .addresses_data
                .empty
                .reader()
                .get(usize::from(index))
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

        let txid_reader = indexer.vecs.transactions.txid.reader();
        let txids: Vec<Txid> = txindices
            .into_iter()
            .map(|txindex| txid_reader.get(txindex.to_usize()))
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

        let txid_reader = vecs.transactions.txid.reader();
        let first_txoutindex_reader = vecs.transactions.first_txoutindex.reader();
        let value_reader = vecs.outputs.value.reader();
        let blockhash_reader = vecs.blocks.blockhash.reader();

        let utxos: Vec<Utxo> = outpoints
            .into_iter()
            .map(|(txindex, vout)| {
                let txid: Txid = txid_reader.get(txindex.to_usize());
                let height = vecs.transactions.height.collect_one_at(txindex.to_usize()).unwrap();
                let first_txoutindex = first_txoutindex_reader.get(txindex.to_usize());
                let txoutindex = first_txoutindex + vout;
                let value: Sats = value_reader.get(usize::from(txoutindex));
                let block_hash = blockhash_reader.get(usize::from(height));
                let block_time = vecs.blocks.timestamp.collect_one_at(usize::from(height)).unwrap();

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

    pub fn address_mempool_hash(&self, address: &Address) -> u64 {
        let Some(mempool) = self.mempool() else {
            return 0;
        };
        let Ok(bytes) = AddressBytes::from_str(address) else {
            return 0;
        };
        mempool.address_hash(&bytes)
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
