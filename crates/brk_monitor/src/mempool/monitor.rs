use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{
    AddressBytes, AddressMempoolStats, MempoolEntryInfo, MempoolInfo, RecommendedFees, TxWithHex,
    Txid, TxidPrefix,
};
use derive_deref::Deref;
use log::{error, info};
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::{FxHashMap, FxHashSet};

use super::block_builder::build_projected_blocks;
use super::entry::MempoolEntry;
use super::projected_blocks::{BlockStats, ProjectedSnapshot};
use super::types::MempoolTxIndex;

/// Max new txs to fetch full data for per update cycle (for address tracking)
const MAX_TX_FETCHES_PER_CYCLE: usize = 10_000;

/// Minimum interval between rebuilds (milliseconds)
const MIN_REBUILD_INTERVAL_MS: u64 = 1000;

/// Block building state - grouped for atomic locking.
#[derive(Default)]
struct BlockBuildingState {
    /// Slot-based entry storage
    entries: Vec<Option<MempoolEntry>>,
    /// TxidPrefix -> slot index
    txid_prefix_to_idx: FxHashMap<TxidPrefix, MempoolTxIndex>,
    /// Recycled slot indices
    free_indices: Vec<MempoolTxIndex>,
}

/// Mempool monitor.
///
/// Thread-safe wrapper around `MempoolInner`. Free to clone.
#[derive(Clone, Deref)]
pub struct Mempool(Arc<MempoolInner>);

impl Mempool {
    pub fn new(client: &Client) -> Self {
        Self(Arc::new(MempoolInner::new(client.clone())))
    }
}

/// Inner mempool state and logic.
pub struct MempoolInner {
    client: Client,

    // Mempool state
    info: RwLock<MempoolInfo>,
    txs: RwLock<FxHashMap<Txid, TxWithHex>>,
    addresses: RwLock<FxHashMap<AddressBytes, (AddressMempoolStats, FxHashSet<Txid>)>>,

    // Block building data (single lock for consistency)
    block_state: RwLock<BlockBuildingState>,

    // Projected blocks snapshot
    snapshot: RwLock<ProjectedSnapshot>,

    // Rate limiting
    dirty: AtomicBool,
    last_rebuild_ms: AtomicU64,
}

impl MempoolInner {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            info: RwLock::new(MempoolInfo::default()),
            txs: RwLock::new(FxHashMap::default()),
            addresses: RwLock::new(FxHashMap::default()),
            block_state: RwLock::new(BlockBuildingState::default()),
            snapshot: RwLock::new(ProjectedSnapshot::default()),
            dirty: AtomicBool::new(false),
            last_rebuild_ms: AtomicU64::new(0),
        }
    }

    pub fn get_info(&self) -> MempoolInfo {
        self.info.read().clone()
    }

    pub fn get_fees(&self) -> RecommendedFees {
        self.snapshot.read().fees.clone()
    }

    pub fn get_snapshot(&self) -> ProjectedSnapshot {
        self.snapshot.read().clone()
    }

    pub fn get_block_stats(&self) -> Vec<BlockStats> {
        self.snapshot.read().block_stats.clone()
    }

    pub fn get_txs(&self) -> RwLockReadGuard<'_, FxHashMap<Txid, TxWithHex>> {
        self.txs.read()
    }

    pub fn get_addresses(
        &self,
    ) -> RwLockReadGuard<'_, FxHashMap<AddressBytes, (AddressMempoolStats, FxHashSet<Txid>)>> {
        self.addresses.read()
    }

    /// Start an infinite update loop with a 1 second interval.
    pub fn start(&self) {
        loop {
            if let Err(e) = self.update() {
                error!("Error updating mempool: {}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    /// Sync with Bitcoin Core mempool and rebuild projections if needed.
    pub fn update(&self) -> Result<()> {
        // Single RPC call gets all entries with fees
        let entries_info = self.client.get_raw_mempool_verbose()?;

        let current_txids: FxHashSet<Txid> = entries_info.iter().map(|e| e.txid.clone()).collect();

        // Find new txids and fetch full tx data for address tracking
        let new_txs = self.fetch_new_txs(&current_txids);

        // Apply changes using the entry info (has fees) and full txs (for addresses)
        let has_changes = self.apply_changes(&entries_info, &new_txs);

        if has_changes {
            self.dirty.store(true, Ordering::Release);
        }

        self.rebuild_if_needed();

        Ok(())
    }

    /// Fetch full transaction data for new txids (needed for address tracking).
    fn fetch_new_txs(&self, current_txids: &FxHashSet<Txid>) -> FxHashMap<Txid, TxWithHex> {
        let txs = self.txs.read();
        current_txids
            .iter()
            .filter(|txid| !txs.contains_key(*txid))
            .take(MAX_TX_FETCHES_PER_CYCLE)
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

    /// Apply transaction additions and removals. Returns true if there were changes.
    fn apply_changes(
        &self,
        entries_info: &[MempoolEntryInfo],
        new_txs: &FxHashMap<Txid, TxWithHex>,
    ) -> bool {
        // Build lookup map for current entries
        let current_entries: FxHashMap<TxidPrefix, &MempoolEntryInfo> = entries_info
            .iter()
            .map(|e| (TxidPrefix::from(&e.txid), e))
            .collect();

        let mut info = self.info.write();
        let mut txs = self.txs.write();
        let mut addresses = self.addresses.write();
        let mut block_state = self.block_state.write();

        let mut had_removals = false;
        let had_additions = !new_txs.is_empty();

        // Remove transactions no longer in mempool
        txs.retain(|txid, tx_with_hex| {
            let prefix = TxidPrefix::from(txid);
            if current_entries.contains_key(&prefix) {
                return true;
            }

            had_removals = true;
            let tx = tx_with_hex.tx();

            info.remove(tx);
            Self::update_address_stats_on_removal(tx, txid, &mut addresses);

            // Remove from slot-based storage
            if let Some(idx) = block_state.txid_prefix_to_idx.remove(&prefix) {
                if let Some(slot) = block_state.entries.get_mut(idx.as_usize()) {
                    *slot = None;
                }
                block_state.free_indices.push(idx);
            }

            false
        });

        // Add new transactions
        for (txid, tx_with_hex) in new_txs {
            let tx = tx_with_hex.tx();
            let prefix = TxidPrefix::from(txid);

            // Get the entry info (has fee from Bitcoin Core)
            let Some(entry_info) = current_entries.get(&prefix) else {
                continue;
            };

            let entry = MempoolEntry::from_info(entry_info);

            info.add(tx);
            Self::update_address_stats_on_addition(tx, txid, &mut addresses);

            // Allocate slot
            let idx = if let Some(idx) = block_state.free_indices.pop() {
                block_state.entries[idx.as_usize()] = Some(entry);
                idx
            } else {
                let idx = MempoolTxIndex::from(block_state.entries.len());
                block_state.entries.push(Some(entry));
                idx
            };

            block_state.txid_prefix_to_idx.insert(prefix, idx);
        }
        txs.extend(new_txs.clone());

        had_removals || had_additions
    }

    /// Rebuild projected blocks if dirty and enough time has passed.
    fn rebuild_if_needed(&self) {
        if !self.dirty.load(Ordering::Acquire) {
            return;
        }

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let last = self.last_rebuild_ms.load(Ordering::Acquire);
        if now_ms.saturating_sub(last) < MIN_REBUILD_INTERVAL_MS {
            return;
        }

        // Attempt to claim the rebuild (atomic compare-exchange)
        if self
            .last_rebuild_ms
            .compare_exchange(last, now_ms, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            return; // Another thread is rebuilding
        }

        self.dirty.store(false, Ordering::Release);

        let i = Instant::now();
        self.rebuild_projected_blocks();
        info!("mempool: rebuild_projected_blocks in {:?}", i.elapsed());
    }

    /// Rebuild projected blocks snapshot.
    fn rebuild_projected_blocks(&self) {
        let block_state = self.block_state.read();

        let blocks = build_projected_blocks(&block_state.entries);
        let snapshot = ProjectedSnapshot::build(blocks, &block_state.entries);

        *self.snapshot.write() = snapshot;
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
