use brk_error::{OptionData, Result};
use brk_types::{Addr, AddrIndexTxIndex, Transaction, TxIndex, Txid, Unit};
use vecdb::VecIndex;

use crate::Query;

impl Query {
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
}
