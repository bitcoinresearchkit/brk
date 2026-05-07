//! Two arrival kinds:
//!
//! - **Fresh** - tx unknown to us. Decode the raw bytes, resolve
//!   prevouts against the live mempool (same-cycle parents), build a
//!   full `Transaction` + `Entry`. Confirmed parents land as
//!   `prevout: None` and are filled post-apply by the resolver passed
//!   to `Mempool::update_with`.
//! - **Revived** - tx in the graveyard. Rebuild the `Entry` only
//!   (preserving `rbf`, `size`). The Applier exhumes the cached tx
//!   body. No raw decoding.

use std::mem;

use brk_rpc::RawTx;
use brk_types::{MempoolEntryInfo, SigOps, Transaction, TxIn, TxOut, TxStatus, Txid, Vout};

use crate::{TxTombstone, stores::TxStore};

use super::TxEntry;

pub enum TxAddition {
    Fresh { tx: Transaction, entry: TxEntry },
    Revived { entry: TxEntry },
}

impl TxAddition {
    /// Resolves prevouts against the live mempool only. Confirmed
    /// parents land with `prevout: None` and are filled by the
    /// resolver supplied to `Mempool::update_with` in the same cycle.
    pub(super) fn fresh(info: &MempoolEntryInfo, raw: RawTx, mempool_txs: &TxStore) -> Self {
        let total_size = raw.hex.len() / 2;
        let rbf = raw.tx.input.iter().any(|i| i.sequence.is_rbf());
        let tx = Self::build_tx(info, raw, total_size, mempool_txs);
        let entry = TxEntry::new(info, total_size as u64, rbf);
        Self::Fresh { tx, entry }
    }

    fn build_tx(
        info: &MempoolEntryInfo,
        mut raw: RawTx,
        total_size: usize,
        mempool_txs: &TxStore,
    ) -> Transaction {
        let input = mem::take(&mut raw.tx.input)
            .into_iter()
            .map(|txin| Self::build_txin(txin, mempool_txs))
            .collect();
        let mut tx = Transaction {
            index: None,
            txid: info.txid,
            version: raw.tx.version.into(),
            total_sigop_cost: SigOps::ZERO,
            weight: info.weight,
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
        let entry = TxEntry::new(info, tomb.entry.size, tomb.entry.rbf);
        Self::Revived { entry }
    }

    fn build_txin(txin: bitcoin::TxIn, mempool_txs: &TxStore) -> TxIn {
        let prev_txid: Txid = txin.previous_output.txid.into();
        let prev_vout = usize::from(Vout::from(txin.previous_output.vout));
        let prevout = Self::resolve_prevout(&prev_txid, prev_vout, mempool_txs);

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

    fn resolve_prevout(prev_txid: &Txid, prev_vout: usize, mempool_txs: &TxStore) -> Option<TxOut> {
        let prev = mempool_txs.get(prev_txid)?;
        prev.output
            .get(prev_vout)
            .map(|o| TxOut::from((o.script_pubkey.clone(), o.value)))
    }
}
