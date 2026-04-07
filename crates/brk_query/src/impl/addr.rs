use std::str::FromStr;

use bitcoin::{Network, PublicKey, ScriptBuf};
use brk_error::{Error, Result};
use brk_types::{
    Addr, AddrBytes, AddrChainStats, AddrHash, AddrIndexOutPoint, AddrIndexTxIndex, AddrStats,
    AnyAddrDataIndexEnum, BlockHash, Dollars, Height, OutputType, Timestamp, Transaction, TxIndex,
    TxStatus, Txid, TypeIndex, Unit, Utxo, Vout,
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

        let realized_price = match &any_addr_index.to_enum() {
            AnyAddrDataIndexEnum::Funded(_) => addr_data.realized_price().to_dollars(),
            AnyAddrDataIndexEnum::Empty(_) => Dollars::default(),
        };

        Ok(AddrStats {
            addr,
            addr_type,
            chain_stats: AddrChainStats {
                type_index,
                funded_txo_count: addr_data.funded_txo_count,
                funded_txo_sum: addr_data.received,
                spent_txo_count: addr_data.spent_txo_count,
                spent_txo_sum: addr_data.sent,
                tx_count: addr_data.tx_count,
                realized_price,
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
        self.transactions_by_indices(&txindices)
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

        if let Some(after_txid) = after_txid {
            let after_tx_index = stores
                .txid_prefix_to_tx_index
                .get(&after_txid.into())
                .map_err(|_| Error::UnknownTxid)?
                .ok_or(Error::UnknownTxid)?
                .into_owned();

            // Seek directly to after_tx_index and iterate backward — O(limit)
            let min = AddrIndexTxIndex::min_for_addr(type_index);
            let bound = AddrIndexTxIndex::from((type_index, after_tx_index));
            Ok(store
                .range(min..bound)
                .rev()
                .take(limit)
                .map(|(key, _): (AddrIndexTxIndex, Unit)| key.tx_index())
                .collect())
        } else {
            // No pagination — scan from end of prefix
            let prefix = u32::from(type_index).to_be_bytes();
            Ok(store
                .prefix(prefix)
                .rev()
                .take(limit)
                .map(|(key, _): (AddrIndexTxIndex, Unit)| key.tx_index())
                .collect())
        }
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

        let mut cached_block: Option<(Height, BlockHash, Timestamp)> = None;
        let mut utxos = Vec::with_capacity(outpoints.len());

        for (tx_index, vout) in outpoints {
            let txid = txid_reader.get(tx_index.to_usize());
            let height: Height = height_cursor.get(tx_index.to_usize()).unwrap();
            let first_txout_index = first_txout_index_reader.get(tx_index.to_usize());
            let value = value_reader.get(usize::from(first_txout_index + vout));

            let (block_hash, block_time) = if let Some((h, ref bh, bt)) = cached_block
                && h == height
            {
                (bh.clone(), bt)
            } else {
                let bh = blockhash_reader.get(height.to_usize());
                let bt = block_ts_cursor.get(height.to_usize()).unwrap();
                cached_block = Some((height, bh.clone(), bt));
                (bh, bt)
            };

            utxos.push(Utxo {
                txid,
                vout,
                status: TxStatus {
                    confirmed: true,
                    block_height: Some(height),
                    block_hash: Some(block_hash),
                    block_time: Some(block_time),
                },
                value,
            });
        }

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
