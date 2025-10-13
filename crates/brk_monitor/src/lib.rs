use std::{thread, time::Duration};

use bitcoin::{Transaction, Txid, consensus::encode};
use bitcoincore_rpc::{Client, RpcApi};
use log::error;
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::{FxHashMap, FxHashSet};

const MAX_FETCHES_PER_CYCLE: usize = 10_000;

pub struct Mempool {
    rpc: &'static Client,
    txs: RwLock<FxHashMap<Txid, Transaction>>,
}

impl Mempool {
    pub fn new(rpc: &'static Client) -> Self {
        Self {
            rpc,
            txs: RwLock::new(FxHashMap::default()),
        }
    }

    pub fn get_txs(&self) -> RwLockReadGuard<'_, FxHashMap<Txid, Transaction>> {
        self.txs.read()
    }

    pub fn start(&self) {
        loop {
            if let Err(e) = self.update() {
                error!("Error updating mempool: {}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let txids = self
            .rpc
            .get_raw_mempool()?
            .into_iter()
            .collect::<FxHashSet<_>>();

        let missing_txids = {
            let txs = self.txs.read();
            txids
                .iter()
                .filter(|txid| !txs.contains_key(*txid))
                .take(MAX_FETCHES_PER_CYCLE)
                .collect::<Vec<_>>()
        };

        let new_txs = missing_txids
            .into_iter()
            .filter_map(|txid| {
                self.rpc
                    .get_raw_transaction_hex(txid, None)
                    .ok()
                    .and_then(|hex| encode::deserialize_hex(&hex).ok())
                    .map(|tx| (*txid, tx))
            })
            .collect::<FxHashMap<_, _>>();

        let mut txs = self.txs.write();
        txs.retain(|txid, _| txids.contains(txid));
        txs.extend(new_txs);

        Ok(())
    }
}
