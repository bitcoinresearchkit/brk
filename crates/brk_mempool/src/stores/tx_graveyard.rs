use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use brk_types::{Transaction, Txid};
use rustc_hash::FxHashMap;

use super::{Entry, Tombstone};
use crate::steps::preparer::Removal;

/// How long a dropped tx stays retained after removal.
const RETENTION: Duration = Duration::from_secs(60 * 60);

/// Recently-dropped txs retained for reappearance detection (Puller can revive
/// them without RPC) and post-mine analytics (RBF/replacement chains, etc.).
#[derive(Default)]
pub struct TxGraveyard {
    tombstones: FxHashMap<Txid, Tombstone>,
    order: VecDeque<(Instant, Txid)>,
}

impl TxGraveyard {
    pub fn contains(&self, txid: &Txid) -> bool {
        self.tombstones.contains_key(txid)
    }

    pub fn get(&self, txid: &Txid) -> Option<&Tombstone> {
        self.tombstones.get(txid)
    }

    /// Tombstones marked as `Replaced { by: replacer }`. Used to walk
    /// backward through RBF history: given a tx that's still live (or
    /// in the graveyard), find every tx it displaced.
    pub fn predecessors_of<'a>(
        &'a self,
        replacer: &'a Txid,
    ) -> impl Iterator<Item = (&'a Txid, &'a Tombstone)> {
        self.tombstones
            .iter()
            .filter_map(move |(txid, ts)| (ts.replaced_by() == Some(replacer)).then_some((txid, ts)))
    }

    pub fn bury(&mut self, txid: Txid, tx: Transaction, entry: Entry, removal: Removal) {
        let now = Instant::now();
        self.tombstones
            .insert(txid.clone(), Tombstone::new(tx, entry, removal, now));
        self.order.push_back((now, txid));
    }

    /// Remove and return the tombstone, e.g. when the tx comes back to life.
    pub fn exhume(&mut self, txid: &Txid) -> Option<Tombstone> {
        self.tombstones.remove(txid)
    }

    /// Drop tombstones older than RETENTION. O(k) in the number of evictions.
    ///
    /// The order queue may carry stale entries (from re-buries or prior
    /// exhumes); the timestamp-match check skips those without disturbing
    /// live tombstones.
    pub fn evict_old(&mut self) {
        while let Some(&(t, _)) = self.order.front() {
            if t.elapsed() < RETENTION {
                break;
            }
            let (_, txid) = self.order.pop_front().unwrap();
            if let Some(ts) = self.tombstones.get(&txid)
                && ts.removed_at() == t
            {
                self.tombstones.remove(&txid);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.tombstones.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tombstones.is_empty()
    }
}
