use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{MempoolEntryInfo, MempoolInfo, TxWithHex, Txid, TxidPrefix};
use derive_deref::Deref;
use log::{debug, error};
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::FxHashMap;

use crate::{
    addresses::AddressTracker,
    block_builder::build_projected_blocks,
    entry::Entry,
    entry_pool::EntryPool,
    projected_blocks::{BlockStats, RecommendedFees, Snapshot},
    tx_store::TxStore,
};

/// Max new txs to fetch full data for per update cycle (for address tracking).
const MAX_TX_FETCHES_PER_CYCLE: usize = 10_000;

/// Minimum interval between rebuilds (milliseconds).
const MIN_REBUILD_INTERVAL_MS: u64 = 1000;

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

    info: RwLock<MempoolInfo>,
    txs: RwLock<TxStore>,
    addresses: RwLock<AddressTracker>,
    entries: RwLock<EntryPool>,

    snapshot: RwLock<Snapshot>,

    dirty: AtomicBool,
    last_rebuild_ms: AtomicU64,
}

impl MempoolInner {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            info: RwLock::new(MempoolInfo::default()),
            txs: RwLock::new(TxStore::default()),
            addresses: RwLock::new(AddressTracker::default()),
            entries: RwLock::new(EntryPool::default()),
            snapshot: RwLock::new(Snapshot::default()),
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

    pub fn get_snapshot(&self) -> Snapshot {
        self.snapshot.read().clone()
    }

    pub fn get_block_stats(&self) -> Vec<BlockStats> {
        self.snapshot.read().block_stats.clone()
    }

    pub fn get_txs(&self) -> RwLockReadGuard<'_, TxStore> {
        self.txs.read()
    }

    pub fn get_addresses(&self) -> RwLockReadGuard<'_, AddressTracker> {
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
        let entries_info = self.client.get_raw_mempool_verbose()?;

        let new_txs = self.fetch_new_txs(&entries_info);
        let has_changes = self.apply_changes(&entries_info, new_txs);

        if has_changes {
            self.dirty.store(true, Ordering::Release);
        }

        self.rebuild_if_needed();

        Ok(())
    }

    /// Fetch full transaction data for new txids (needed for address tracking).
    fn fetch_new_txs(&self, entries_info: &[MempoolEntryInfo]) -> FxHashMap<Txid, TxWithHex> {
        let txids_to_fetch: Vec<Txid> = {
            let txs = self.txs.read();
            entries_info
                .iter()
                .map(|e| &e.txid)
                .filter(|txid| !txs.contains(txid))
                .take(MAX_TX_FETCHES_PER_CYCLE)
                .cloned()
                .collect()
        };

        txids_to_fetch
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
        new_txs: FxHashMap<Txid, TxWithHex>,
    ) -> bool {
        let entries_by_prefix: FxHashMap<TxidPrefix, &MempoolEntryInfo> = entries_info
            .iter()
            .map(|e| (TxidPrefix::from(&e.txid), e))
            .collect();

        let mut info = self.info.write();
        let mut txs = self.txs.write();
        let mut addresses = self.addresses.write();
        let mut entries = self.entries.write();

        let mut had_removals = false;
        let had_additions = !new_txs.is_empty();

        // Remove transactions no longer in mempool
        txs.retain_or_remove(
            |txid| entries_by_prefix.contains_key(&TxidPrefix::from(txid)),
            |txid, tx_with_hex| {
                had_removals = true;
                let tx = tx_with_hex.tx();
                let prefix = TxidPrefix::from(txid);

                // Get fee from entries (before removing) - this is the authoritative fee from Bitcoin Core
                let fee = entries.get(&prefix).map(|e| e.fee).unwrap_or_default();
                info.remove(tx, fee);
                addresses.remove_tx(tx, txid);
                entries.remove(&prefix);
            },
        );

        // Add new transactions
        for (txid, tx_with_hex) in &new_txs {
            let tx = tx_with_hex.tx();
            let prefix = TxidPrefix::from(txid);

            let Some(entry_info) = entries_by_prefix.get(&prefix) else {
                continue;
            };

            info.add(tx, entry_info.fee);
            addresses.add_tx(tx, txid);
            entries.insert(prefix, Entry::from_info(entry_info));
        }
        txs.extend(new_txs);

        had_removals || had_additions
    }

    /// Rebuild projected blocks if dirty and enough time has passed.
    fn rebuild_if_needed(&self) {
        if !self.dirty.load(Ordering::Acquire) {
            return;
        }

        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let last = self.last_rebuild_ms.load(Ordering::Acquire);
        if now_ms.saturating_sub(last) < MIN_REBUILD_INTERVAL_MS {
            return;
        }

        if self
            .last_rebuild_ms
            .compare_exchange(last, now_ms, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            return;
        }

        self.dirty.store(false, Ordering::Release);

        let i = Instant::now();
        self.rebuild_projected_blocks();
        debug!("mempool: rebuild_projected_blocks in {:?}", i.elapsed());
    }

    /// Rebuild projected blocks snapshot.
    fn rebuild_projected_blocks(&self) {
        let entries = self.entries.read();
        let entries_slice = entries.entries();

        let blocks = build_projected_blocks(entries_slice);
        let snapshot = Snapshot::build(blocks, entries_slice);

        *self.snapshot.write() = snapshot;
    }
}
