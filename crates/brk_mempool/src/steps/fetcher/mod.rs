mod fetched;

pub use fetched::Fetched;

use brk_error::Result;
use brk_rpc::{Client, MempoolState};
use brk_types::{MempoolEntryInfo, Timestamp, Txid, VSize};
use parking_lot::RwLock;
use rustc_hash::FxHashSet;

use crate::State;

/// Cap before the batch RPC so we never hand bitcoind an unbounded batch.
/// GBT-synthesized entries are not subject to this cap; they're bounded
/// by the block weight limit Core enforces on its own template.
const MAX_TX_FETCHES_PER_CYCLE: usize = 10_000;

/// Two batched round-trips per cycle, scaling with churn rather than
/// mempool size: `getblocktemplate` + `getrawmempool false` +
/// `getmempoolinfo` in one mixed batch; then `getmempoolentry` +
/// `getrawtransaction` for *new* non-GBT txids in a second mixed batch.
///
/// GBT entries already carry the full tx body and stats, so any GBT tx
/// not yet in the local pool is materialized inline from the GBT
/// payload instead of being refetched. That removes the GBT/listing
/// race that used to skip cycles when a tx vanished from the mempool
/// between the GBT and `getrawmempool` calls: block 0 always reflects
/// Core's exact selection because we never ask for that data twice.
///
/// Confirmed prevouts are resolved post-apply by the caller-supplied
/// resolver passed to `Mempool::update_with`, so the in-crate path no
/// longer issues a third batch for parents.
pub struct Fetcher;

impl Fetcher {
    pub fn fetch(client: &Client, lock: &RwLock<State>) -> Result<Fetched> {
        let MempoolState {
            live_txids,
            gbt,
            min_fee,
        } = client.fetch_mempool_state()?;

        // One read snapshot decides both the RPC fetch list and the
        // GBT-synthesis set, so they agree on what's "already known".
        // Graveyard txs are treated as known so a re-broadcast still
        // flows through `Preparer::classify_addition` and lands as
        // [`crate::TxAddition::Revived`].
        let (new_txids, gbt_synth_set) = {
            let state = lock.read();
            let mut gbt_txids: FxHashSet<Txid> =
                FxHashSet::with_capacity_and_hasher(gbt.len(), Default::default());
            let mut gbt_synth_set: FxHashSet<Txid> = FxHashSet::default();
            for g in &gbt {
                gbt_txids.insert(g.txid);
                if !state.txs.contains(&g.txid) {
                    gbt_synth_set.insert(g.txid);
                }
            }
            let new_txids: Vec<Txid> = live_txids
                .iter()
                .filter(|t| !state.txs.contains(t) && !gbt_txids.contains(t))
                .take(MAX_TX_FETCHES_PER_CYCLE)
                .copied()
                .collect();
            (new_txids, gbt_synth_set)
        };

        let (mut new_entries, mut new_txs) = client.fetch_new_pool_data(&new_txids)?;
        new_entries.reserve(gbt_synth_set.len());
        new_txs.reserve(gbt_synth_set.len());

        // Consume `gbt` by value: GBT-only txs move their body and
        // depends into the synthesis path (no clones), and the GBT
        // ordering is captured as a `Vec<Txid>` for the Rebuilder, which
        // is the only downstream consumer and only reads txids.
        //
        // GBT carries no per-tx arrival timestamp. `now` is correct to
        // within ~1 cycle for a tx that just entered Core's mempool
        // (the only kind that triggers synthesis: not in our pool yet
        // means it just appeared this cycle).
        let now = Timestamp::now();
        let gbt_txids: Vec<Txid> = gbt
            .into_iter()
            .map(|g| {
                let txid = g.txid;
                if gbt_synth_set.contains(&txid) {
                    new_entries.push(MempoolEntryInfo {
                        txid,
                        vsize: VSize::from(g.weight),
                        weight: g.weight,
                        fee: g.fee,
                        first_seen: now,
                        depends: g.depends,
                    });
                    new_txs.insert(txid, g.tx);
                }
                txid
            })
            .collect();

        Ok(Fetched {
            live_txids,
            new_entries,
            new_txs,
            gbt_txids,
            min_fee,
        })
    }
}
