use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use brk_types::{Transaction, Txid};
use rustc_hash::FxHashMap;

mod tombstone;

pub use tombstone::TxTombstone;

use crate::{TxEntry, TxRemoval};

const RETENTION: Duration = Duration::from_hours(1);

/// Recently-dropped txs retained for reappearance detection (Puller can revive
/// them without RPC) and post-mine analytics (RBF/replacement chains, etc.).
#[derive(Default)]
pub struct TxGraveyard {
    tombstones: FxHashMap<Txid, TxTombstone>,
    order: VecDeque<(Instant, Txid)>,
}

impl TxGraveyard {
    pub fn contains(&self, txid: &Txid) -> bool {
        self.tombstones.contains_key(txid)
    }

    pub fn get(&self, txid: &Txid) -> Option<&TxTombstone> {
        self.tombstones.get(txid)
    }

    /// Tombstones marked as `Replaced { by: replacer }`. Used to walk
    /// backward through RBF history: given a tx that's still live (or
    /// in the graveyard), find every tx it displaced.
    pub fn predecessors_of<'a>(
        &'a self,
        replacer: &'a Txid,
    ) -> impl Iterator<Item = (&'a Txid, &'a TxTombstone)> {
        self.tombstones.iter().filter_map(move |(txid, ts)| {
            (ts.replaced_by() == Some(replacer)).then_some((txid, ts))
        })
    }

    /// Every `Replaced` tombstone, yielded as (predecessor_txid,
    /// replacer_txid). Caller walks the replacer chain forward to find
    /// each tree's terminal replacer.
    pub fn replaced_iter(&self) -> impl Iterator<Item = (&Txid, &Txid)> {
        self.tombstones
            .iter()
            .filter_map(|(txid, ts)| ts.replaced_by().map(|by| (txid, by)))
    }

    pub fn bury(&mut self, txid: Txid, tx: Transaction, entry: TxEntry, removal: TxRemoval) {
        let now = Instant::now();
        self.tombstones
            .insert(txid.clone(), TxTombstone::new(tx, entry, removal, now));
        self.order.push_back((now, txid));
    }

    /// Remove and return the tombstone, e.g. when the tx comes back to life.
    pub fn exhume(&mut self, txid: &Txid) -> Option<TxTombstone> {
        self.tombstones.remove(txid)
    }

    /// Drop tombstones older than RETENTION. O(k) in the number of evictions.
    ///
    /// The order queue may carry stale entries (from re-buries or prior
    /// exhumes). The timestamp-match check skips those without disturbing
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
}
