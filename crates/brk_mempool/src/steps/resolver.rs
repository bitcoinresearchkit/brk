//! Prevout resolution for live mempool txs.
//!
//! A fresh tx can land in the store with `prevout: None` on some
//! inputs when the Preparer can't see the parent (parent arrived in
//! the same cycle as the child, or parent is confirmed and Core
//! lacks `-txindex`). Two paths fix that, both writing through the
//! same `apply_fills` -> `add_input` plumbing:
//!
//! - [`Resolver::resolve_in_mempool`]: same-cycle parents from the
//!   live `txs` map. Run by the orchestrator after each successful
//!   `MempoolState::apply`. No external dependency.
//! - [`Resolver::resolve_external`]: caller-supplied resolver
//!   (typically the brk indexer). Run on demand by API consumers
//!   that have a confirmed-tx data source. Lock-free during the
//!   resolver call.
//!
//! Both phases:
//! 1. Snapshot under `txs.read()`, gather work for unresolved txs
//!    (early-exit if `txs.unresolved()` is empty).
//! 2. (external only) Call the resolver outside any lock.
//! 3. Write fills under `txs.write()` + `addrs.write()`, in that
//!    order to match the Applier's lock order.
//!
//! Idempotent: `apply_fills` checks `prevout.is_none()` per input
//! and bails if the tx was removed between phases.

use brk_types::{TxOut, Txid, Vin, Vout};

use crate::stores::MempoolState;

/// Per-tx fills to apply: (vin index, resolved prevout).
type Fills = Vec<(Vin, TxOut)>;
/// Per-tx holes to resolve: (vin index, parent txid, parent vout).
type Holes = Vec<(Vin, Txid, Vout)>;

pub struct Resolver;

impl Resolver {
    /// Fill prevouts whose parent is also live in the mempool.
    ///
    /// Called by the orchestrator after each successful
    /// `MempoolState::apply`. Catches parent/child pairs that arrived
    /// in the same cycle: the Preparer resolves against a snapshot
    /// taken before the cycle's adds were applied, so neither parent
    /// nor child is in it; both are in `txs` by the time we run.
    pub fn resolve_in_mempool(state: &MempoolState) -> bool {
        let filled: Vec<(Txid, Fills)> = {
            let txs = state.txs.read();
            if txs.unresolved().is_empty() {
                return false;
            }
            txs.unresolved()
                .iter()
                .filter_map(|txid| {
                    let tx = txs.get(txid)?;
                    let fills: Fills = tx
                        .input
                        .iter()
                        .enumerate()
                        .filter(|(_, txin)| txin.prevout.is_none())
                        .filter_map(|(i, txin)| {
                            let parent = txs.get(&txin.txid)?;
                            let out = parent.output.get(usize::from(txin.vout))?;
                            Some((Vin::from(i), out.clone()))
                        })
                        .collect();
                    (!fills.is_empty()).then_some((txid.clone(), fills))
                })
                .collect()
        };
        Self::write_back(state, filled)
    }

    /// Fill prevouts via an external resolver, typically backed by the
    /// brk indexer for confirmed parents.
    ///
    /// Phase 1 collects holes under `txs.read()`; phase 2 runs the
    /// resolver outside any lock; phase 3 writes back. Holes already
    /// resolvable from in-mempool parents have been filled by
    /// [`Resolver::resolve_in_mempool`] in the preceding `apply`, so
    /// anything reaching the resolver here is genuinely external.
    pub fn resolve_external<F>(state: &MempoolState, resolver: F) -> bool
    where
        F: Fn(&Txid, Vout) -> Option<TxOut>,
    {
        let holes: Vec<(Txid, Holes)> = {
            let txs = state.txs.read();
            if txs.unresolved().is_empty() {
                return false;
            }
            txs.unresolved()
                .iter()
                .filter_map(|txid| {
                    let tx = txs.get(txid)?;
                    let holes: Holes = tx
                        .input
                        .iter()
                        .enumerate()
                        .filter(|(_, txin)| txin.prevout.is_none())
                        .map(|(i, txin)| (Vin::from(i), txin.txid.clone(), txin.vout))
                        .collect();
                    (!holes.is_empty()).then_some((txid.clone(), holes))
                })
                .collect()
        };

        let filled: Vec<(Txid, Fills)> = holes
            .into_iter()
            .filter_map(|(txid, holes)| {
                let fills: Fills = holes
                    .into_iter()
                    .filter_map(|(vin, prev_txid, vout)| {
                        resolver(&prev_txid, vout).map(|o| (vin, o))
                    })
                    .collect();
                (!fills.is_empty()).then_some((txid, fills))
            })
            .collect();

        Self::write_back(state, filled)
    }

    /// Apply per-tx fills under `txs.write()` + `addrs.write()`.
    /// Each successful prevout write is folded into `AddrTracker` via
    /// `add_input`. Lock order matches the Applier's (txs before addrs).
    fn write_back(state: &MempoolState, fills: Vec<(Txid, Fills)>) -> bool {
        if fills.is_empty() {
            return false;
        }
        let mut txs = state.txs.write();
        let mut addrs = state.addrs.write();
        for (txid, tx_fills) in fills {
            for prevout in txs.apply_fills(&txid, tx_fills) {
                addrs.add_input(&txid, &prevout);
            }
        }
        true
    }
}
