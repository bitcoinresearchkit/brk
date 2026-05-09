use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use brk_types::{FeeRate, Transaction, Txid};
use rustc_hash::FxHashMap;

mod tombstone;

pub(crate) use tombstone::TxTombstone;

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
    pub fn tombstones_len(&self) -> usize {
        self.tombstones.len()
    }

    pub fn order_len(&self) -> usize {
        self.order.len()
    }

    pub fn get(&self, txid: &Txid) -> Option<&TxTombstone> {
        self.tombstones.get(txid)
    }

    /// Tombstone iff the tx vanished from the pool (mined, expired, or
    /// dropped). `Replaced` tombstones return `None` because the tx
    /// will not confirm.
    pub fn get_vanished(&self, txid: &Txid) -> Option<&TxTombstone> {
        let tomb = self.tombstones.get(txid)?;
        matches!(tomb.removal, TxRemoval::Vanished).then_some(tomb)
    }

    /// Walk forward through `Replaced { by }` to the terminal replacer.
    /// Returns `txid` itself if it isn't replaced (live or `Vanished`).
    pub fn replacement_root_of(&self, mut txid: Txid) -> Txid {
        while let Some(TxRemoval::Replaced { by }) =
            self.tombstones.get(&txid).map(|t| &t.removal)
        {
            txid = *by;
        }
        txid
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
    /// replacer_txid) in reverse bury order (most recent replacement
    /// event first). Caller walks the replacer chain forward to find
    /// each tree's terminal replacer.
    ///
    /// `order` may carry stale entries (re-buries, prior exhumes); the
    /// `removed_at == t` check skips those.
    pub fn replaced_iter_recent_first(&self) -> impl Iterator<Item = (&Txid, &Txid)> {
        self.order.iter().rev().filter_map(|(t, txid)| {
            let ts = self.tombstones.get(txid)?;
            if ts.removed_at != *t {
                return None;
            }
            Some((txid, ts.replaced_by()?))
        })
    }

    pub fn bury(
        &mut self,
        tx: Transaction,
        entry: TxEntry,
        chunk_rate: FeeRate,
        removal: TxRemoval,
    ) {
        let txid = entry.txid;
        let removed_at = Instant::now();
        self.tombstones.insert(
            txid,
            TxTombstone {
                tx,
                entry,
                chunk_rate,
                removal,
                removed_at,
            },
        );
        self.order.push_back((removed_at, txid));
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
                && ts.removed_at == t
            {
                self.tombstones.remove(&txid);
            }
        }
    }
}
