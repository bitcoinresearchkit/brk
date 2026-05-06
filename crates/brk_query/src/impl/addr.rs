use std::str::FromStr;

use bitcoin::{Network, PublicKey, ScriptBuf};
use brk_error::{Error, OptionData, Result};
use brk_types::{
    Addr, AddrBytes, AddrChainStats, AddrHash, AddrIndexOutPoint, AddrIndexTxIndex, AddrStats,
    AnyAddrDataIndexEnum, Dollars, Height, OutputType, Timestamp, Transaction, TxIndex, TxStatus,
    Txid, TxidPrefix, TypeIndex, Unit, Utxo, Vout,
};
use vecdb::VecIndex;

use crate::Query;

impl Query {
    pub fn addr(&self, addr: Addr) -> Result<AddrStats> {
        let computer = self.computer();

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
        let hash = AddrHash::from(&bytes);
        let type_index = self.type_index_for(output_type, &hash)?;

        if type_index >= self.safe_lengths().to_type_index(output_type) {
            return Err(Error::UnknownAddr);
        }

        let any_addr_index = computer
            .distribution
            .any_addr_indexes
            .get_once(output_type, type_index)?;

        let (addr_data, realized_price) = match any_addr_index.to_enum() {
            AnyAddrDataIndexEnum::Funded(index) => {
                let data = computer
                    .distribution
                    .addrs_data
                    .funded
                    .reader()
                    .get(usize::from(index));
                let price = data.realized_price().to_dollars();
                (data, price)
            }
            AnyAddrDataIndexEnum::Empty(index) => {
                let data = computer
                    .distribution
                    .addrs_data
                    .empty
                    .reader()
                    .get(usize::from(index))
                    .into();
                (data, Dollars::default())
            }
        };

        Ok(AddrStats {
            addr,
            addr_type: output_type,
            chain_stats: AddrChainStats {
                type_index,
                funded_txo_count: addr_data.funded_txo_count,
                funded_txo_sum: addr_data.received,
                spent_txo_count: addr_data.spent_txo_count,
                spent_txo_sum: addr_data.sent,
                tx_count: addr_data.tx_count,
                realized_price,
            },
            mempool_stats: self
                .mempool()
                .and_then(|m| m.addrs().get(&bytes).map(|e| e.stats.clone()))
                .unwrap_or_default(),
        })
    }

    /// Esplora `/address/:address/txs` first page: up to `mempool_limit`
    /// mempool entries (newest first), then chain entries fill the response
    /// up to `total_limit`. Pagination is path-style via `/txs/chain/:after_txid`.
    pub fn addr_txs(
        &self,
        addr: Addr,
        total_limit: usize,
        mempool_limit: usize,
    ) -> Result<Vec<Transaction>> {
        let mut out = if self.mempool().is_some() {
            self.addr_mempool_txs(&addr, mempool_limit)?
        } else {
            Vec::new()
        };
        let chain_limit = total_limit.saturating_sub(out.len());
        out.extend(self.addr_txs_chain(&addr, None, chain_limit)?);
        Ok(out)
    }

    pub fn addr_txs_chain(
        &self,
        addr: &Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Transaction>> {
        let txindices = self.addr_txindices(addr, after_txid, limit)?;
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
        Ok(txindices
            .into_iter()
            .map(|tx_index| txid_reader.get(tx_index.to_usize()))
            .collect())
    }

    fn addr_txindices(
        &self,
        addr: &Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<TxIndex>> {
        let stores = &self.indexer().stores;

        let (output_type, type_index) = self.resolve_addr(addr)?;

        let store = stores
            .addr_type_to_addr_index_and_tx_index
            .get(output_type)
            .data()?;

        let tx_index_len = self.safe_lengths().tx_index;

        if let Some(after_txid) = after_txid {
            let after_tx_index = self.resolve_tx_index(&after_txid)?;
            let min = AddrIndexTxIndex::min_for_addr(type_index);
            let cursor = AddrIndexTxIndex::from((type_index, after_tx_index));
            Ok(store
                .range(min..cursor)
                .rev()
                .map(|(key, _): (AddrIndexTxIndex, Unit)| key.tx_index())
                .filter(|tx_index| *tx_index < tx_index_len)
                .take(limit)
                .collect())
        } else {
            Ok(store
                .prefix(type_index)
                .rev()
                .map(|(key, _): (AddrIndexTxIndex, Unit)| key.tx_index())
                .filter(|tx_index| *tx_index < tx_index_len)
                .take(limit)
                .collect())
        }
    }

    pub fn addr_utxos(&self, addr: Addr, max_utxos: usize) -> Result<Vec<Utxo>> {
        let indexer = self.indexer();
        let stores = &indexer.stores;
        let vecs = &indexer.vecs;

        let (output_type, type_index) = self.resolve_addr(&addr)?;

        let store = stores
            .addr_type_to_addr_index_and_unspent_outpoint
            .get(output_type)
            .data()?;

        let tx_index_len = self.safe_lengths().tx_index;
        let outpoints: Vec<(TxIndex, Vout)> = store
            .prefix(type_index)
            .map(|(key, _): (AddrIndexOutPoint, Unit)| (key.tx_index(), key.vout()))
            .filter(|(tx_index, _)| *tx_index < tx_index_len)
            .take(max_utxos + 1)
            .collect();
        if outpoints.len() > max_utxos {
            return Err(Error::TooManyUtxos);
        }

        let txid_reader = vecs.transactions.txid.reader();
        let first_txout_index_reader = vecs.transactions.first_txout_index.reader();
        let value_reader = vecs.outputs.value.reader();

        let mut cached_status: Option<(Height, TxStatus)> = None;
        let mut utxos = Vec::with_capacity(outpoints.len());

        for (tx_index, vout) in outpoints {
            let txid = txid_reader.get(tx_index.to_usize());
            let first_txout_index = first_txout_index_reader.get(tx_index.to_usize());
            let value = value_reader.get(usize::from(first_txout_index + vout));

            let height = self.confirmed_status_height(tx_index)?;
            let status = if let Some((h, ref s)) = cached_status
                && h == height
            {
                s.clone()
            } else {
                let s = self.confirmed_status_at(height)?;
                cached_status = Some((height, s.clone()));
                s
            };

            utxos.push(Utxo {
                txid,
                vout,
                status,
                value,
            });
        }

        Ok(utxos)
    }

    pub fn addr_mempool_hash(&self, addr: &Addr) -> Option<u64> {
        let mempool = self.mempool()?;
        let bytes = AddrBytes::from_str(addr).ok()?;
        Some(mempool.addr_state_hash(&bytes))
    }

    pub fn addr_mempool_txs(&self, addr: &Addr, limit: usize) -> Result<Vec<Transaction>> {
        let bytes = AddrBytes::from_str(addr)?;
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let addrs = mempool.addrs();
        let Some(entry) = addrs.get(&bytes) else {
            return Ok(vec![]);
        };
        let entries = mempool.entries();
        let mut ordered: Vec<(Timestamp, &Txid)> = entry
            .txids
            .iter()
            .map(|txid| {
                let first_seen = entries
                    .get(&TxidPrefix::from(txid))
                    .map(|e| e.first_seen)
                    .unwrap_or_default();
                (first_seen, txid)
            })
            .collect();
        ordered.sort_unstable_by_key(|b| std::cmp::Reverse(b.0));
        let txs = mempool.txs();
        Ok(ordered
            .into_iter()
            .filter_map(|(_, txid)| txs.get(txid).cloned())
            .take(limit)
            .collect())
    }

    /// Height of the last on-chain activity for an address (last tx_index → height).
    pub fn addr_last_activity_height(&self, addr: &Addr) -> Result<Height> {
        let (output_type, type_index) = self.resolve_addr(addr)?;
        let store = self
            .indexer()
            .stores
            .addr_type_to_addr_index_and_tx_index
            .get(output_type)
            .data()?;
        let tx_index_len = self.safe_lengths().tx_index;
        let last_tx_index = store
            .prefix(type_index)
            .rev()
            .map(|(key, _): (AddrIndexTxIndex, Unit)| key.tx_index())
            .find(|tx_index| *tx_index < tx_index_len)
            .ok_or(Error::UnknownAddr)?;
        self.confirmed_status_height(last_tx_index)
    }

    fn resolve_addr(&self, addr: &Addr) -> Result<(OutputType, TypeIndex)> {
        let bytes = AddrBytes::from_str(addr)?;
        let output_type = OutputType::from(&bytes);
        let hash = AddrHash::from(&bytes);
        let type_index = self.type_index_for(output_type, &hash)?;
        Ok((output_type, type_index))
    }

    /// Lookup the per-type index of an address by `(output_type, hash)`.
    /// Returns `UnknownAddr` if the hash is absent from the type's index.
    fn type_index_for(&self, output_type: OutputType, hash: &AddrHash) -> Result<TypeIndex> {
        self.indexer()
            .stores
            .addr_type_to_addr_hash_to_addr_index
            .get(output_type)
            .data()?
            .get(hash)?
            .map(|cow| cow.into_owned())
            .ok_or(Error::UnknownAddr)
    }
}
