use std::{thread, time::Duration};

use bitcoin::consensus::encode;
use brk_rpc::Client;
use brk_structs::{AddressBytes, AddressMempoolStats, Transaction, Txid};
use log::error;
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::{FxHashMap, FxHashSet};

const MAX_FETCHES_PER_CYCLE: usize = 10_000;

pub struct Mempool {
    client: Client,
    txs: RwLock<FxHashMap<Txid, Transaction>>,
    addresses: RwLock<FxHashMap<AddressBytes, (AddressMempoolStats, FxHashSet<Txid>)>>,
}

impl Mempool {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            txs: RwLock::new(FxHashMap::default()),
            addresses: RwLock::new(FxHashMap::default()),
        }
    }

    pub fn get_txs(&self) -> RwLockReadGuard<'_, FxHashMap<Txid, Transaction>> {
        self.txs.read()
    }

    pub fn get_addresses(
        &self,
    ) -> RwLockReadGuard<'_, FxHashMap<AddressBytes, (AddressMempoolStats, FxHashSet<Txid>)>> {
        self.addresses.read()
    }

    pub fn start(&self) {
        loop {
            if let Err(e) = self.update() {
                error!("Error updating mempool: {}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let txids = self
            .client
            .get_raw_mempool()?
            .into_iter()
            .map(Txid::from)
            .collect::<FxHashSet<_>>();

        let new_txs = {
            let txs = self.txs.read();
            txids
                .iter()
                .filter(|txid| !txs.contains_key(*txid))
                .take(MAX_FETCHES_PER_CYCLE)
                .cloned()
                .collect::<Vec<_>>()
        }
        .into_iter()
        .filter_map(|txid| {
            self.client
                .get_raw_transaction_hex(&bitcoin::Txid::from(&txid), None)
                .ok()
                .and_then(|hex| encode::deserialize_hex::<bitcoin::Transaction>(&hex).ok())
                .map(|tx| Transaction::from_mempool(tx, self.client))
                .map(|tx| (txid, tx))
        })
        .collect::<FxHashMap<_, _>>();

        let mut txs = self.txs.write();
        let mut addresses = self.addresses.write();
        txs.retain(|txid, tx| {
            if txids.contains(txid) {
                return true;
            }
            tx.input
                .iter()
                .flat_map(|txin| txin.prevout.as_ref())
                .flat_map(|txout| txout.address_bytes().map(|bytes| (txout, bytes)))
                .for_each(|(txout, bytes)| {
                    let (stats, set) = addresses.entry(bytes).or_default();
                    set.remove(txid);
                    stats.sent(txout);
                    stats.update_tx_count(set.len() as u32);
                });
            tx.output
                .iter()
                .flat_map(|txout| txout.address_bytes().map(|bytes| (txout, bytes)))
                .for_each(|(txout, bytes)| {
                    let (stats, set) = addresses.entry(bytes).or_default();
                    set.remove(txid);
                    stats.received(txout);
                    stats.update_tx_count(set.len() as u32);
                });
            false
        });
        new_txs.iter().for_each(|(txid, tx)| {
            tx.input
                .iter()
                .flat_map(|txin| txin.prevout.as_ref())
                .flat_map(|txout| txout.address_bytes().map(|bytes| (txout, bytes)))
                .for_each(|(txout, bytes)| {
                    let (stats, set) = addresses.entry(bytes).or_default();
                    set.insert(txid.clone());
                    stats.sending(txout);
                    stats.update_tx_count(set.len() as u32);
                });
            tx.output
                .iter()
                .flat_map(|txout| txout.address_bytes().map(|bytes| (txout, bytes)))
                .for_each(|(txout, bytes)| {
                    let (stats, set) = addresses.entry(bytes).or_default();
                    set.insert(txid.clone());
                    stats.receiving(txout);
                    stats.update_tx_count(set.len() as u32);
                });
        });
        txs.extend(new_txs);

        Ok(())
    }
}
