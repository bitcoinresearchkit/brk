use std::str::FromStr;

use bitcoin::{Network, PublicKey, ScriptBuf};
use brk_error::{Error, Result};
use brk_types::{
    Address, AddressBytes, AddressChainStats, AddressHash, AddressIndexOutPoint,
    AddressIndexTxIndex, AddressStats, AnyAddressDataIndexEnum, OutputType, Sats, Transaction,
    TxIndex, TxStatus, Txid, TypeIndex, Unit, Utxo, Vout,
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

        let output_type = OutputType::from(&script);
        let Ok(bytes) = AddressBytes::try_from((&script, output_type)) else {
            return Err(Error::InvalidAddress);
        };
        let address_type = output_type;
        let hash = AddressHash::from(&bytes);

        let Ok(Some(type_index)) = stores
            .address_type_to_address_hash_to_address_index
            .get_unwrap(address_type)
            .get(&hash)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownAddress);
        };

        let any_address_index = computer
            .distribution
            .any_address_indexes
            .get_once(output_type, type_index)?;

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

    pub fn address_txs(
        &self,
        address: Address,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Transaction>> {
        let txindices = self.address_txindices(&address, after_txid, limit)?;
        txindices
            .into_iter()
            .map(|tx_index| self.transaction_by_index(tx_index))
            .collect()
    }

    pub fn address_txids(
        &self,
        address: Address,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Txid>> {
        let txindices = self.address_txindices(&address, after_txid, limit)?;
        let txid_reader = self.indexer().vecs.transactions.txid.reader();
        let txids = txindices
            .into_iter()
            .map(|tx_index| txid_reader.get(tx_index.to_usize()))
            .collect();
        Ok(txids)
    }

    fn address_txindices(
        &self,
        address: &Address,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<TxIndex>> {
        let indexer = self.indexer();
        let stores = &indexer.stores;

        let (output_type, type_index) = self.resolve_address(address)?;

        let store = stores
            .address_type_to_address_index_and_tx_index
            .get(output_type)
            .unwrap();

        let prefix = u32::from(type_index).to_be_bytes();

        let after_tx_index = if let Some(after_txid) = after_txid {
            let tx_index = stores
                .txid_prefix_to_tx_index
                .get(&after_txid.into())
                .map_err(|_| Error::UnknownTxid)?
                .ok_or(Error::UnknownTxid)?
                .into_owned();
            Some(tx_index)
        } else {
            None
        };

        Ok(store
            .prefix(prefix)
            .rev()
            .filter(|(key, _): &(AddressIndexTxIndex, Unit)| {
                if let Some(after) = after_tx_index {
                    key.tx_index() < after
                } else {
                    true
                }
            })
            .take(limit)
            .map(|(key, _)| key.tx_index())
            .collect())
    }

    pub fn address_utxos(&self, address: Address) -> Result<Vec<Utxo>> {
        let indexer = self.indexer();
        let stores = &indexer.stores;
        let vecs = &indexer.vecs;

        let (output_type, type_index) = self.resolve_address(&address)?;

        let store = stores
            .address_type_to_address_index_and_unspent_outpoint
            .get(output_type)
            .unwrap();

        let prefix = u32::from(type_index).to_be_bytes();

        let outpoints: Vec<(TxIndex, Vout)> = store
            .prefix(prefix)
            .map(|(key, _): (AddressIndexOutPoint, Unit)| (key.tx_index(), key.vout()))
            .collect();

        let txid_reader = vecs.transactions.txid.reader();
        let first_txout_index_reader = vecs.transactions.first_txout_index.reader();
        let value_reader = vecs.outputs.value.reader();
        let blockhash_reader = vecs.blocks.blockhash.reader();

        let utxos: Vec<Utxo> = outpoints
            .into_iter()
            .map(|(tx_index, vout)| {
                let txid: Txid = txid_reader.get(tx_index.to_usize());
                let height = vecs
                    .transactions
                    .height
                    .collect_one_at(tx_index.to_usize())
                    .unwrap();
                let first_txout_index = first_txout_index_reader.get(tx_index.to_usize());
                let txout_index = first_txout_index + vout;
                let value: Sats = value_reader.get(usize::from(txout_index));
                let block_hash = blockhash_reader.get(usize::from(height));
                let block_time = vecs
                    .blocks
                    .timestamp
                    .collect_one_at(usize::from(height))
                    .unwrap();

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
        let output_type = OutputType::from(&bytes);
        let hash = AddressHash::from(&bytes);

        let Ok(Some(type_index)) = stores
            .address_type_to_address_hash_to_address_index
            .get(output_type)
            .unwrap()
            .get(&hash)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownAddress);
        };

        Ok((output_type, type_index))
    }
}
