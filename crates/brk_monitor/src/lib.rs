use std::{sync::Arc, thread, time::Duration};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{
    AddressBytes, AddressMempoolStats, MempoolInfo, RecommendedFees, TxWithHex, Txid,
};
use derive_deref::Deref;
use log::error;
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::{FxHashMap, FxHashSet};

mod mempool;

use mempool::{ProjectedBlocks, TxGraph};

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
    info: RwLock<MempoolInfo>,
    fees: RwLock<RecommendedFees>,
    graph: RwLock<TxGraph>,
    projected_blocks: RwLock<ProjectedBlocks>,
    txs: RwLock<FxHashMap<Txid, TxWithHex>>,
    addresses: RwLock<FxHashMap<AddressBytes, (AddressMempoolStats, FxHashSet<Txid>)>>,
}

impl MempoolInner {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            info: RwLock::new(MempoolInfo::default()),
            fees: RwLock::new(RecommendedFees::default()),
            graph: RwLock::new(TxGraph::new()),
            projected_blocks: RwLock::new(ProjectedBlocks::default()),
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

    pub fn get_projected_blocks(&self) -> ProjectedBlocks {
        self.projected_blocks.read().clone()
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
        let current_txids = self
            .client
            .get_raw_mempool()?
            .into_iter()
            .collect::<FxHashSet<_>>();

        let new_txs = self.fetch_new_txs(&current_txids);
        let has_changes = self.apply_changes(&current_txids, &new_txs);

        if has_changes {
            self.rebuild_projected_blocks();
        }

        Ok(())
    }

    /// Fetch transactions that are new to our mempool
    fn fetch_new_txs(&self, current_txids: &FxHashSet<Txid>) -> FxHashMap<Txid, TxWithHex> {
        let txs = self.txs.read();
        current_txids
            .iter()
            .filter(|txid| !txs.contains_key(*txid))
            .take(MAX_FETCHES_PER_CYCLE)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|txid| {
                self.client
                    .get_mempool_transaction(&txid)
                    .ok()
                    .map(|tx| (txid, tx))
            })
            .collect()
    }

    /// Apply transaction additions and removals, returns true if there were changes
    fn apply_changes(
        &self,
        current_txids: &FxHashSet<Txid>,
        new_txs: &FxHashMap<Txid, TxWithHex>,
    ) -> bool {
        let mut info = self.info.write();
        let mut graph = self.graph.write();
        let mut txs = self.txs.write();
        let mut addresses = self.addresses.write();

        let mut had_removals = false;
        let had_additions = !new_txs.is_empty();

        // Remove transactions no longer in mempool
        txs.retain(|txid, tx_with_hex| {
            if current_txids.contains(txid) {
                return true;
            }

            had_removals = true;
            let tx = tx_with_hex.tx();

            info.remove(tx);
            graph.remove(txid);
            Self::update_address_stats_on_removal(tx, txid, &mut addresses);

            false
        });

        // Add new transactions
        for (txid, tx_with_hex) in new_txs {
            let tx = tx_with_hex.tx();

            info.add(tx);
            graph.insert(tx);
            Self::update_address_stats_on_addition(tx, txid, &mut addresses);
        }
        txs.extend(new_txs.clone());

        had_removals || had_additions
    }

    /// Rebuild projected blocks and update recommended fees
    fn rebuild_projected_blocks(&self) {
        let graph = self.graph.read();
        let projected = ProjectedBlocks::build(&graph);
        let fees = projected.recommended_fees();

        *self.projected_blocks.write() = projected;
        *self.fees.write() = fees;
    }

    fn update_address_stats_on_removal(
        tx: &brk_types::Transaction,
        txid: &Txid,
        addresses: &mut FxHashMap<AddressBytes, (AddressMempoolStats, FxHashSet<Txid>)>,
    ) {
        // Inputs: undo "sending" state
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

        // Outputs: undo "receiving" state
        tx.output
            .iter()
            .flat_map(|txout| txout.address_bytes().map(|bytes| (txout, bytes)))
            .for_each(|(txout, bytes)| {
                let (stats, set) = addresses.entry(bytes).or_default();
                set.remove(txid);
                stats.received(txout);
                stats.update_tx_count(set.len() as u32);
            });
    }

    fn update_address_stats_on_addition(
        tx: &brk_types::Transaction,
        txid: &Txid,
        addresses: &mut FxHashMap<AddressBytes, (AddressMempoolStats, FxHashSet<Txid>)>,
    ) {
        // Inputs: mark as "sending"
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

        // Outputs: mark as "receiving"
        tx.output
            .iter()
            .flat_map(|txout| txout.address_bytes().map(|bytes| (txout, bytes)))
            .for_each(|(txout, bytes)| {
                let (stats, set) = addresses.entry(bytes).or_default();
                set.insert(txid.clone());
                stats.receiving(txout);
                stats.update_tx_count(set.len() as u32);
            });
    }
}
