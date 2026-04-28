//! Two arrival kinds:
//!
//! - **Fresh** - tx unknown to us. Decode the raw bytes, resolve
//!   prevouts against `known` or `parent_raws`, build a full
//!   `Transaction` + `Entry`.
//! - **Revived** - tx in the graveyard. Rebuild the `Entry` only
//!   (preserving `first_seen`, `rbf`, `size`). The Applier exhumes
//!   the cached tx body. No raw decoding.

use std::mem;

use brk_rpc::RawTx;
use brk_types::{MempoolEntryInfo, Timestamp, Transaction, TxIn, TxOut, TxStatus, Txid, Vout};
use rustc_hash::FxHashMap;

use crate::{TxTombstone, stores::TxStore};

use super::TxEntry;

pub enum TxAddition {
    Fresh { tx: Transaction, entry: TxEntry },
    Revived { entry: TxEntry },
}

impl TxAddition {
    /// Resolves prevouts against the live mempool first, then `parent_raws`.
    /// Unresolved inputs land with `prevout: None` for later filling by
    /// the Resolver or by `brk_query` at read time.
    pub(super) fn fresh(
        info: &MempoolEntryInfo,
        raw: RawTx,
        parent_raws: &FxHashMap<Txid, RawTx>,
        mempool_txs: &TxStore,
    ) -> Self {
        let total_size = raw.hex.len() / 2;
        let rbf = raw.tx.input.iter().any(|i| i.sequence.is_rbf());
        let tx = Self::build_tx(info, raw, total_size, mempool_txs, parent_raws);
        let entry = TxEntry::new(info, total_size as u64, rbf, Timestamp::now());
        Self::Fresh { tx, entry }
    }

    fn build_tx(
        info: &MempoolEntryInfo,
        mut raw: RawTx,
        total_size: usize,
        mempool_txs: &TxStore,
        parent_raws: &FxHashMap<Txid, RawTx>,
    ) -> Transaction {
        let input = mem::take(&mut raw.tx.input)
            .into_iter()
            .map(|txin| Self::build_txin(txin, mempool_txs, parent_raws))
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
        tx
    }

    pub(super) fn revived(info: &MempoolEntryInfo, tomb: &TxTombstone) -> Self {
        let entry = TxEntry::new(info, tomb.entry.size, tomb.entry.rbf, tomb.entry.first_seen);
        Self::Revived { entry }
    }

    fn build_txin(
        txin: bitcoin::TxIn,
        mempool_txs: &TxStore,
        parent_raws: &FxHashMap<Txid, RawTx>,
    ) -> TxIn {
        let prev_txid: Txid = txin.previous_output.txid.into();
        let prev_vout = usize::from(Vout::from(txin.previous_output.vout));
        let prevout = Self::resolve_prevout(&prev_txid, prev_vout, mempool_txs, parent_raws);

        TxIn {
            // Mempool txs are never coinbase (Core rejects them
            // from the pool entirely).
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
    }

    fn resolve_prevout(
        prev_txid: &Txid,
        prev_vout: usize,
        mempool_txs: &TxStore,
        parent_raws: &FxHashMap<Txid, RawTx>,
    ) -> Option<TxOut> {
        if let Some(prev) = mempool_txs.get(prev_txid) {
            return prev
                .output
                .get(prev_vout)
                .map(|o| TxOut::from((o.script_pubkey.clone(), o.value)));
        }
        parent_raws.get(prev_txid).and_then(|parent| {
            parent
                .tx
                .output
                .get(prev_vout)
                .map(|o| TxOut::from((o.script_pubkey.clone(), o.value.into())))
        })
    }
}
