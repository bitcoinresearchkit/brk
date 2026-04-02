use std::str::FromStr;

use bitcoin::{Network, PublicKey, ScriptBuf};
use brk_error::{Error, Result};
use brk_types::{
    Addr, AddrBytes, AddrChainStats, AddrHash, AddrIndexOutPoint, AddrIndexTxIndex, AddrStats,
    AnyAddrDataIndexEnum, Height, OutputType, Transaction, TxIndex, TxStatus, Txid, TypeIndex,
    Unit, Utxo, Vout,
};
use vecdb::{ReadableVec, VecIndex};

use crate::Query;

/// Maximum number of mempool txids to return
const MAX_MEMPOOL_TXIDS: usize = 50;

impl Query {
    pub fn addr(&self, addr: Addr) -> Result<AddrStats> {
        let indexer = self.indexer();
        let computer = self.computer();
        let stores = &indexer.stores;

        let script = if let Ok(addr) = bitcoin::Address::from_str(&addr) {
            if !addr.is_valid_for_network(Network::Bitcoin) {
                return Err(Error::InvalidNetwork);
            }
            let addr = addr.assume_checked();
            addr.script_pubkey()
        } else if let Ok(pubkey) = PublicKey::from_str(&addr) {
            ScriptBuf::new_p2pk(&pubkey)
        } else {
            return Err(Error::InvalidAddr);
        };

        let output_type = OutputType::from(&script);
        let Ok(bytes) = AddrBytes::try_from((&script, output_type)) else {
            return Err(Error::InvalidAddr);
        };
        let addr_type = output_type;
        let hash = AddrHash::from(&bytes);

        let Ok(Some(type_index)) = stores
            .addr_type_to_addr_hash_to_addr_index
            .get_unwrap(addr_type)
            .get(&hash)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownAddr);
        };

        let any_addr_index = computer
            .distribution
            .any_addr_indexes
            .get_once(output_type, type_index)?;

        let addr_data = match any_addr_index.to_enum() {
            AnyAddrDataIndexEnum::Funded(index) => computer
                .distribution
                .addrs_data
                .funded
                .reader()
                .get(usize::from(index)),
            AnyAddrDataIndexEnum::Empty(index) => computer
                .distribution
                .addrs_data
                .empty
                .reader()
                .get(usize::from(index))
                .into(),
        };

        Ok(AddrStats {
            addr,
            chain_stats: AddrChainStats {
                type_index,
                funded_txo_count: addr_data.funded_txo_count,
                funded_txo_sum: addr_data.received,
                spent_txo_count: addr_data.spent_txo_count,
                spent_txo_sum: addr_data.sent,
                tx_count: addr_data.tx_count,
            },
            mempool_stats: self.mempool().map(|mempool| {
                mempool
                    .get_addrs()
                    .get(&bytes)
                    .map(|(stats, _)| stats)
                    .cloned()
                    .unwrap_or_default()
            }),
        })
    }

    pub fn addr_txs(
        &self,
        addr: Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Transaction>> {
        let txindices = self.addr_txindices(&addr, after_txid, limit)?;
        txindices
            .into_iter()
            .map(|tx_index| self.transaction_by_index(tx_index))
            .collect()
    }

    pub fn addr_txids(
        &self,
        addr: Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Txid>> {
        let txindices = self.addr_txindices(&addr, after_txid, limit)?;
        let txid_reader = self.indexer().vecs.transactions.txid.reader();
        let txids = txindices
            .into_iter()
            .map(|tx_index| txid_reader.get(tx_index.to_usize()))
            .collect();
        Ok(txids)
    }

    fn addr_txindices(
        &self,
        addr: &Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<TxIndex>> {
        let indexer = self.indexer();
        let stores = &indexer.stores;

        let (output_type, type_index) = self.resolve_addr(addr)?;

        let store = stores
            .addr_type_to_addr_index_and_tx_index
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
            .filter(|(key, _): &(AddrIndexTxIndex, Unit)| {
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

    pub fn addr_utxos(&self, addr: Addr) -> Result<Vec<Utxo>> {
        let indexer = self.indexer();
        let stores = &indexer.stores;
        let vecs = &indexer.vecs;

        let (output_type, type_index) = self.resolve_addr(&addr)?;

        let store = stores
            .addr_type_to_addr_index_and_unspent_outpoint
            .get(output_type)
            .unwrap();

        let prefix = u32::from(type_index).to_be_bytes();

        let outpoints: Vec<(TxIndex, Vout)> = store
            .prefix(prefix)
            .map(|(key, _): (AddrIndexOutPoint, Unit)| (key.tx_index(), key.vout()))
            .collect();

        let txid_reader = vecs.transactions.txid.reader();
        let first_txout_index_reader = vecs.transactions.first_txout_index.reader();
        let value_reader = vecs.outputs.value.reader();
        let blockhash_reader = vecs.blocks.blockhash.reader();
        let mut height_cursor = vecs.transactions.height.cursor();
        let mut block_ts_cursor = vecs.blocks.timestamp.cursor();

        let utxos: Vec<Utxo> = outpoints
            .into_iter()
            .map(|(tx_index, vout)| {
                let txid = txid_reader.get(tx_index.to_usize());
                let height = height_cursor.get(tx_index.to_usize()).unwrap();
                let first_txout_index = first_txout_index_reader.get(tx_index.to_usize());
                let txout_index = first_txout_index + vout;
                let value = value_reader.get(usize::from(txout_index));
                let block_hash = blockhash_reader.get(usize::from(height));
                let block_time = block_ts_cursor.get(height.to_usize()).unwrap();

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

    pub fn addr_mempool_hash(&self, addr: &Addr) -> u64 {
        let Some(mempool) = self.mempool() else {
            return 0;
        };
        let Ok(bytes) = AddrBytes::from_str(addr) else {
            return 0;
        };
        mempool.addr_hash(&bytes)
    }

    pub fn addr_mempool_txids(&self, addr: Addr) -> Result<Vec<Txid>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;

        let bytes = AddrBytes::from_str(&addr)?;
        let addrs = mempool.get_addrs();

        let txids: Vec<Txid> = addrs
            .get(&bytes)
            .map(|(_, txids)| txids.iter().take(MAX_MEMPOOL_TXIDS).cloned().collect())
            .unwrap_or_default();

        Ok(txids)
    }

    /// Height of the last on-chain activity for an address (last tx_index → height).
    pub fn addr_last_activity_height(&self, addr: &Addr) -> Result<Height> {
        let (output_type, type_index) = self.resolve_addr(addr)?;
        let store = self
            .indexer()
            .stores
            .addr_type_to_addr_index_and_tx_index
            .get(output_type)
            .unwrap();
        let prefix = u32::from(type_index).to_be_bytes();
        let last_tx_index = store
            .prefix(prefix)
            .next_back()
            .map(|(key, _): (AddrIndexTxIndex, Unit)| key.tx_index())
            .ok_or(Error::UnknownAddr)?;
        self.indexer()
            .vecs
            .transactions
            .height
            .collect_one(last_tx_index)
            .ok_or(Error::UnknownAddr)
    }

    /// Resolve an address string to its output type and type_index
    fn resolve_addr(&self, addr: &Addr) -> Result<(OutputType, TypeIndex)> {
        let stores = &self.indexer().stores;

        let bytes = AddrBytes::from_str(addr)?;
        let output_type = OutputType::from(&bytes);
        let hash = AddrHash::from(&bytes);

        let Ok(Some(type_index)) = stores
            .addr_type_to_addr_hash_to_addr_index
            .get(output_type)
            .unwrap()
            .get(&hash)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownAddr);
        };

        Ok((output_type, type_index))
    }
}
