use std::{
    collections::BTreeMap,
    sync::Arc,
    thread,
    time::Duration,
};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{
    AddressBytes, AddressMempoolStats, FeeRate, MempoolInfo, RecommendedFees, TxWithHex, Txid,
    VSize,
};
use derive_deref::Deref;
use log::error;
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::{FxHashMap, FxHashSet};

const MAX_FETCHES_PER_CYCLE: usize = 10_000;

/// Target block vsize (1MB = 1_000_000 vbytes, but using 4MW weight / 4 = 1MW vbytes max)
const BLOCK_VSIZE_TARGET: u64 = 1_000_000;

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
    info: RwLock<MempoolInfo>,
    fees: RwLock<RecommendedFees>,
    /// Map of fee rate -> total vsize at that fee rate, used for fee estimation
    fee_rates: RwLock<BTreeMap<FeeRate, VSize>>,
    txs: RwLock<FxHashMap<Txid, TxWithHex>>,
    addresses: RwLock<FxHashMap<AddressBytes, (AddressMempoolStats, FxHashSet<Txid>)>>,
}

impl MempoolInner {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            info: RwLock::new(MempoolInfo::default()),
            fees: RwLock::new(RecommendedFees::default()),
            fee_rates: RwLock::new(BTreeMap::new()),
            txs: RwLock::new(FxHashMap::default()),
            addresses: RwLock::new(FxHashMap::default()),
        }
    }

    pub fn get_info(&self) -> MempoolInfo {
        self.info.read().clone()
    }

    pub fn get_fees(&self) -> RecommendedFees {
        self.fees.read().clone()
    }

    pub fn get_txs(&self) -> RwLockReadGuard<'_, FxHashMap<Txid, TxWithHex>> {
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

        let mut info = self.info.write();
        let mut txs = self.txs.write();
        let mut addresses = self.addresses.write();

        txs.retain(|txid, tx_with_hex| {
            if txids.contains(txid) {
                return true;
            }
            let tx = tx_with_hex.tx();
            info.remove(tx);

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

        new_txs.iter().for_each(|(txid, tx_with_hex)| {
            let tx = tx_with_hex.tx();
            info.add(tx);

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
