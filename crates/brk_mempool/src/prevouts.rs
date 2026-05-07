//! Prevout fill plumbing.
//!
//! A fresh tx can land in the store with `prevout: None` on some
//! inputs when the Preparer can't see the parent (parent arrived in
//! the same cycle as the child, or parent is confirmed and we don't
//! have an indexer hooked up). [`fill`] runs after each successful
//! `Applier::apply` and closes both gaps in one pass:
//!
//! 1. Snapshot under a read guard, walking `txs.unresolved()` once.
//!    For each hole, if the parent is also in the live pool we record
//!    a fill directly (cheap, lock-local). Otherwise we record the
//!    hole for external resolution.
//! 2. Drop the read guard. Call `resolver` on the remaining holes
//!    (typically `getrawtransaction` or an indexer lookup); failures
//!    are simply skipped and retried next cycle.
//! 3. Take the write guard once and fold both fill batches into the
//!    `TxStore` via `apply_fills` -> `add_input`. Idempotent: each
//!    fill checks `prevout.is_none()` and bails if the tx was already
//!    removed or filled between phases.

use std::sync::atomic::{AtomicBool, Ordering};

use brk_rpc::Client;
use brk_types::{TxOut, Txid, TxidPrefix, Vin, Vout};
use parking_lot::RwLock;
use tracing::warn;

use crate::{State, stores::TxStore};

/// Default resolver: per-call `getrawtransaction` against the bitcoind
/// RPC client `Mempool` already holds. Requires `txindex=1`. On any
/// failure logs once with a hint, then returns `None`; the next cycle
/// retries automatically.
pub(crate) fn rpc_resolver(client: Client) -> impl Fn(&Txid, Vout) -> Option<TxOut> {
    let warned = AtomicBool::new(false);
    move |txid, vout| {
        let bt: &bitcoin::Txid = txid.into();
        match client.get_raw_transaction(bt, None as Option<&bitcoin::BlockHash>) {
            Ok(tx) => tx
                .output
                .get(usize::from(vout))
                .map(|o| TxOut::from((o.script_pubkey.clone(), o.value.into()))),
            Err(_) => {
                if !warned.swap(true, Ordering::Relaxed) {
                    warn!(
                        "mempool: getrawtransaction missed for {txid}; ensure bitcoind is running with txindex=1"
                    );
                }
                None
            }
        }
    }
}

type Fills = Vec<(Vin, TxOut)>;
type Holes = Vec<(Vin, Txid, Vout)>;
type FillBatch = Vec<(TxidPrefix, Txid, Fills)>;
type HoleBatch = Vec<(TxidPrefix, Txid, Holes)>;

/// Fill every unfilled prevout the cycle can resolve. Same-cycle
/// in-mempool parents are filled lock-locally; the remainder go
/// through `resolver` outside any lock. Returns true iff anything
/// was written.
pub(crate) fn fill<F>(lock: &RwLock<State>, resolver: F) -> bool
where
    F: Fn(&Txid, Vout) -> Option<TxOut>,
{
    let (in_mempool, holes) = {
        let state = lock.read();
        gather(&state.txs)
    };
    let external = resolve_external(holes, resolver);

    if in_mempool.is_empty() && external.is_empty() {
        return false;
    }

    let mut state = lock.write();
    write_fills(&mut state, in_mempool);
    write_fills(&mut state, external);
    true
}

/// Single pass over `txs.unresolved()`: bucket each hole into a
/// same-cycle in-mempool fill (parent is live) or an external hole
/// (parent is confirmed or unknown).
fn gather(txs: &TxStore) -> (FillBatch, HoleBatch) {
    if txs.unresolved().is_empty() {
        return (Vec::new(), Vec::new());
    }
    let mut filled: FillBatch = Vec::new();
    let mut holes: HoleBatch = Vec::new();
    for prefix in txs.unresolved() {
        let Some(record) = txs.record_by_prefix(prefix) else {
            continue;
        };
        let mut tx_fills: Fills = Vec::new();
        let mut tx_holes: Holes = Vec::new();
        for (i, txin) in record.tx.input.iter().enumerate() {
            if txin.prevout.is_some() {
                continue;
            }
            let vin = Vin::from(i);
            if let Some(parent) = txs.get(&txin.txid)
                && let Some(out) = parent.output.get(usize::from(txin.vout))
            {
                tx_fills.push((vin, out.clone()));
            } else {
                tx_holes.push((vin, txin.txid, txin.vout));
            }
        }
        let txid = record.entry.txid;
        if !tx_fills.is_empty() {
            filled.push((*prefix, txid, tx_fills));
        }
        if !tx_holes.is_empty() {
            holes.push((*prefix, txid, tx_holes));
        }
    }
    (filled, holes)
}

fn resolve_external<F>(holes: HoleBatch, resolver: F) -> FillBatch
where
    F: Fn(&Txid, Vout) -> Option<TxOut>,
{
    holes
        .into_iter()
        .filter_map(|(prefix, txid, holes)| {
            let fills: Fills = holes
                .into_iter()
                .filter_map(|(vin, prev_txid, vout)| resolver(&prev_txid, vout).map(|o| (vin, o)))
                .collect();
            (!fills.is_empty()).then_some((prefix, txid, fills))
        })
        .collect()
}

fn write_fills(state: &mut State, fills: FillBatch) {
    for (prefix, txid, tx_fills) in fills {
        for prevout in state.txs.apply_fills(&prefix, tx_fills) {
            state.addrs.add_input(&txid, &prevout);
        }
    }
}
