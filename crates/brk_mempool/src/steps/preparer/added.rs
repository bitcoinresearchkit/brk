//! Classification and construction of newly-observed mempool txs.
//!
//! Two kinds of arrival:
//! - **Fresh**: the tx is unknown to us, so we decode the raw bytes,
//!   resolve prevouts against `known` or `parent_raws`, and build a
//!   full `Transaction` + `Entry`.
//! - **Revived**: the tx is in the graveyard. We rebuild the `Entry`
//!   (preserving `first_seen` / `rbf` / `size`) and let the Applier
//!   exhume the cached tx body. No raw decoding.

use std::mem;

use brk_rpc::RawTx;
use brk_types::{
    MempoolEntryInfo, Timestamp, Transaction, TxIn, TxOut, TxStatus, Txid, TxidPrefix, VSize, Vout,
};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::stores::{Entry, Tombstone, TxStore};

/// A newly observed tx. `Fresh` carries decoded raw data (just parsed
/// from `new_raws`); `Revived` carries only the rebuilt entry because
/// the tx body is still sitting in the graveyard and will be exhumed
/// by the Applier.
pub enum Addition {
    Fresh { tx: Transaction, entry: Entry },
    Revived { entry: Entry },
}

/// Decode a raw tx into a full `Fresh` addition. Resolves prevouts
/// against the live mempool first, then `parent_raws` (confirmed
/// parents fetched in step 3 of the Fetcher pipeline). Inputs whose
/// parent isn't in either source land with `prevout: None` and are
/// filled later by the Resolver or by `brk_query` at read time.
pub(super) fn fresh(
    info: &MempoolEntryInfo,
    mut raw: RawTx,
    parent_raws: &FxHashMap<Txid, RawTx>,
    mempool_txs: &TxStore,
) -> Addition {
    let total_size = raw.hex.len() / 2;
    let rbf = raw.tx.input.iter().any(|i| i.sequence.is_rbf());

    let input = mem::take(&mut raw.tx.input)
        .into_iter()
        .map(|txin| {
            let prev_txid: Txid = txin.previous_output.txid.into();
            let prev_vout = usize::from(Vout::from(txin.previous_output.vout));

            let prevout = if let Some(prev) = mempool_txs.get(&prev_txid) {
                prev.output
                    .get(prev_vout)
                    .map(|o| TxOut::from((o.script_pubkey.clone(), o.value)))
            } else if let Some(parent) = parent_raws.get(&prev_txid) {
                parent
                    .tx
                    .output
                    .get(prev_vout)
                    .map(|o| TxOut::from((o.script_pubkey.clone(), o.value.into())))
            } else {
                None
            };

            TxIn {
                // Mempool txs are never coinbase (Core rejects
                // them from the pool entirely). A missing prevout
                // only means we couldn't resolve the confirmed
                // parent (no `-txindex`); brk_query fills it at
                // read time from the indexer.
                is_coinbase: false,
                prevout,
                txid: prev_txid,
                vout: txin.previous_output.vout.into(),
                script_sig: txin.script_sig,
                script_sig_asm: (),
                witness: txin.witness.into(),
                sequence: txin.sequence.into(),
                inner_redeem_script_asm: (),
                inner_witness_script_asm: (),
            }
        })
        .collect();

    let mut tx = Transaction {
        index: None,
        txid: info.txid.clone(),
        version: raw.tx.version.into(),
        total_sigop_cost: 0,
        weight: info.weight.into(),
        lock_time: raw.tx.lock_time.into(),
        total_size,
        fee: info.fee,
        input,
        output: raw.tx.output.into_iter().map(TxOut::from).collect(),
        status: TxStatus::UNCONFIRMED,
    };
    tx.total_sigop_cost = tx.total_sigop_cost();

    let entry = build_entry(info, tx.total_size as u64, rbf, Timestamp::now());

    Addition::Fresh { tx, entry }
}

/// Resurrect an entry from a tombstone. The tx body stays buried
/// until the Applier exhumes it; we only rebuild the `Entry` so the
/// preserved `first_seen` / `rbf` / `size` carry over.
pub(super) fn revived(info: &MempoolEntryInfo, tomb: &Tombstone) -> Addition {
    let entry = build_entry(info, tomb.entry.size, tomb.entry.rbf, tomb.entry.first_seen);
    Addition::Revived { entry }
}

fn build_entry(info: &MempoolEntryInfo, size: u64, rbf: bool, first_seen: Timestamp) -> Entry {
    let depends: SmallVec<[TxidPrefix; 2]> = info.depends.iter().map(TxidPrefix::from).collect();
    Entry {
        txid: info.txid.clone(),
        fee: info.fee,
        vsize: VSize::from(info.vsize),
        size,
        depends,
        first_seen,
        rbf,
    }
}
