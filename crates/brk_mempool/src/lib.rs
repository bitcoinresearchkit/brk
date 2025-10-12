use std::{thread, time::Duration};

use bitcoin::{Transaction, consensus::encode};
use bitcoincore_rpc::{Client, RpcApi};
use log::error;
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::FxHashMap;

const MAX_FETCHES_PER_CYCLE: usize = 10_000;

pub struct Mempool {
    rpc: &'static Client,
    txs: RwLock<FxHashMap<String, Transaction>>,
}

impl Mempool {
    pub fn new(rpc: &'static Client) -> Self {
        Self {
            rpc,
            txs: RwLock::new(FxHashMap::default()),
        }
    }

    pub fn get_txs(&self) -> RwLockReadGuard<'_, FxHashMap<String, Transaction>> {
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
        let current_txids = self.rpc.get_raw_mempool()?;

        let current_set: std::collections::HashSet<String> =
            current_txids.iter().map(|t| t.to_string()).collect();

        // Fetch new transactions
        let mut new_txs = FxHashMap::default();
        let mut fetched = 0;

        for txid in current_txids {
            if fetched >= MAX_FETCHES_PER_CYCLE {
                break;
            }

            let txid_str = txid.to_string();
            if !self.txs.read().contains_key(&txid_str)
                && let Ok(hex) = self.rpc.get_raw_transaction_hex(&txid, None)
            {
                let tx: Transaction = encode::deserialize_hex(&hex)?;

                new_txs.insert(txid_str, tx);
                fetched += 1;
            }
        }

        {
            let mut mempool = self.txs.write();
            mempool.retain(|txid, _| current_set.contains(txid));
            mempool.extend(new_txs);
        }

        Ok(())
    }
}
