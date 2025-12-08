use std::{sync::Arc, thread, time::Duration};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{AddressBytes, AddressMempoolStats, Transaction, Txid};
use derive_deref::Deref;
use log::error;
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::{FxHashMap, FxHashSet};

const MAX_FETCHES_PER_CYCLE: usize = 10_000;

///
/// Mempool monitor
///
/// Thread safe and free to clone
///
#[derive(Clone, Deref)]
pub struct Mempool(Arc<MempoolInner>);

impl Mempool {
    pub fn new(client: &Client) -> Self {
        Self(Arc::new(MempoolInner::new(client.clone())))
    }
}

pub struct MempoolInner {
    client: Client,
    txs: RwLock<FxHashMap<Txid, Transaction>>,
    addresses: RwLock<FxHashMap<AddressBytes, (AddressMempoolStats, FxHashSet<Txid>)>>,
}

impl MempoolInner {
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

    /// Start an infinite update loop with a 1 second interval
    pub fn start(&self) {
        loop {
            if let Err(e) = self.update() {
                error!("Error updating mempool: {}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn update(&self) -> Result<()> {
        let txids = self
            .client
            .get_raw_mempool()?
            .into_iter()
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
                .get_mempool_transaction(&txid)
                .ok()
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
