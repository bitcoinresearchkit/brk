use brk_error::{Error, OptionData, Result};
use brk_types::{Addr, AddrIndexOutPoint, Height, TxIndex, TxStatus, Unit, Utxo, Vout};
use vecdb::VecIndex;

use crate::Query;

impl Query {
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
}
