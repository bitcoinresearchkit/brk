use bitcoin::{Script, script::Instruction};
use brk_types::{OutputType, SigOps, TxInIndex};
use rayon::prelude::*;

use super::{BlockProcessor, InputSource, ProcessedOutput};

impl BlockProcessor<'_> {
    /// BIP-141 sigop cost per tx in the block. Mirrors
    /// `bitcoin::Transaction::total_sigop_cost` but dispatches on each
    /// input's prevout `OutputType` and each output's `OutputType`
    /// (already resolved by `process_inputs`/`process_outputs`) instead
    /// of round-tripping through bitcoin's closure API with
    /// synthetic-prevout `ScriptBuf` allocations. The legacy-sigop walk
    /// is short-circuited by `OutputType` for every script with a
    /// canonical shape (~99% of outputs and ~95% of inputs on mainnet);
    /// only `OpReturn`/`Unknown` outputs and non-segwit/non-P2SH inputs
    /// fall back to a real script walk.
    pub fn compute_sigops(
        &self,
        txins: &[(TxInIndex, InputSource)],
        txouts: &[ProcessedOutput<'_>],
    ) -> Vec<SigOps> {
        let txdata = &self.block.txdata;
        let base_tx_index = u32::from(self.lengths.tx_index);

        let mut tx_input_offsets = Vec::with_capacity(txdata.len());
        let mut tx_output_offsets = Vec::with_capacity(txdata.len());
        let mut input_offset = 0usize;
        let mut output_offset = 0usize;
        for tx in txdata {
            tx_input_offsets.push(input_offset);
            input_offset += tx.input.len();
            tx_output_offsets.push(output_offset);
            output_offset += tx.output.len();
        }

        txdata
            .par_iter()
            .enumerate()
            .map(|(i, tx)| {
                if tx.is_coinbase() {
                    return SigOps::ZERO;
                }
                let in_start = tx_input_offsets[i];
                let tx_inputs = &txins[in_start..in_start + tx.input.len()];
                let out_start = tx_output_offsets[i];
                let tx_outputs = &txouts[out_start..out_start + tx.output.len()];

                let mut legacy: usize = 0;
                let mut redeem: usize = 0;
                let mut witness: usize = 0;

                for (input, (_, source)) in tx.input.iter().zip(tx_inputs.iter()) {
                    let prev_kind = match source {
                        InputSource::PreviousBlock { output_type, .. } => *output_type,
                        InputSource::SameBlock { outpoint, .. } => {
                            let local = (u32::from(outpoint.tx_index()) - base_tx_index) as usize;
                            let vout = u32::from(outpoint.vout()) as usize;
                            txouts[tx_output_offsets[local] + vout].output_type
                        }
                    };

                    // Single match per input: legacy script_sig sigops AND the
                    // redeem/witness contribution. Consensus enforces a
                    // push-only or empty script_sig in the four cases below
                    // (BIP-16 for P2SH from block 173805 onwards; BIP-141 /
                    // BIP-341 for segwit/taproot from activation), so legacy
                    // sigops are guaranteed 0 there. Everything else falls
                    // through to a real `count_sigops_legacy` walk.
                    match prev_kind {
                        OutputType::P2SH => {
                            // Faithful to bitcoin's count_p2sh_sigops + the
                            // nested-segwit branch of count_witness_sigops in
                            // a single script walk: redeem sigops use
                            // last_pushdata (no push-only check), wrapped
                            // witness sigops require both push-only and
                            // last_pushdata.
                            let (last_push, is_push_only) =
                                last_push_and_push_only(&input.script_sig);
                            let Some(redeem_bytes) = last_push else {
                                continue;
                            };
                            let rs = Script::from_bytes(redeem_bytes);
                            redeem = redeem.saturating_add(rs.count_sigops());
                            if !is_push_only {
                                continue;
                            }
                            if rs.is_p2wpkh() {
                                witness = witness.saturating_add(1);
                            } else if rs.is_p2wsh()
                                && let Some(last) = input.witness.last()
                            {
                                witness =
                                    witness.saturating_add(Script::from_bytes(last).count_sigops());
                            }
                        }
                        OutputType::P2WPKH => {
                            witness = witness.saturating_add(1);
                        }
                        OutputType::P2WSH => {
                            if let Some(last) = input.witness.last() {
                                witness =
                                    witness.saturating_add(Script::from_bytes(last).count_sigops());
                            }
                        }
                        OutputType::P2TR => {}
                        _ => {
                            legacy = legacy.saturating_add(input.script_sig.count_sigops_legacy());
                        }
                    }
                }

                for processed in tx_outputs {
                    legacy = legacy.saturating_add(legacy_sigops_for_output(
                        processed.output_type,
                        &processed.txout.script_pubkey,
                    ));
                }

                SigOps::from(
                    legacy
                        .saturating_mul(4)
                        .saturating_add(redeem.saturating_mul(4))
                        .saturating_add(witness),
                )
            })
            .collect()
    }
}

/// Legacy sigop count of a script_pubkey, dispatched on `OutputType`.
/// Every variant except `OpReturn` and `Unknown` has a canonical shape
/// recognised by `OutputType::from`'s exact byte-pattern matchers, so
/// the legacy sigop count is fixed: P2PKH and P2PK both end in a
/// single OP_CHECKSIG (1), P2MS contains one OP_CHECKMULTISIG counted
/// as 20 in legacy mode, and P2SH/P2WPKH/P2WSH/P2TR/P2A/Empty contain
/// no CHECKSIG-class opcodes outside their pushdata. `OpReturn`
/// payloads can include 0xac/0xae bytes outside a push, and `Unknown`
/// can be anything, so both fall back to a real script walk.
#[inline]
fn legacy_sigops_for_output(output_type: OutputType, script_pubkey: &Script) -> usize {
    match output_type {
        OutputType::P2PKH | OutputType::P2PK33 | OutputType::P2PK65 => 1,
        OutputType::P2MS => 20,
        OutputType::P2SH
        | OutputType::P2WPKH
        | OutputType::P2WSH
        | OutputType::P2TR
        | OutputType::P2A
        | OutputType::Empty => 0,
        OutputType::OpReturn | OutputType::Unknown => script_pubkey.count_sigops_legacy(),
    }
}

/// Single-pass equivalent of bitcoin's private `last_pushdata()` plus the
/// public `is_push_only()`: returns the bytes of the script's last
/// `Instruction::PushBytes` (only when it is the *last* instruction)
/// alongside whether every instruction was a push (per Core,
/// `OP_RESERVED` and `OP_PUSHNUM_1..16` count as pushes too).
fn last_push_and_push_only(script: &Script) -> (Option<&[u8]>, bool) {
    let mut last: Option<&[u8]> = None;
    let mut push_only = true;
    for inst in script.instructions() {
        match inst {
            Ok(Instruction::PushBytes(b)) => {
                last = Some(b.as_bytes());
            }
            Ok(Instruction::Op(op)) => {
                last = None;
                if op.to_u8() > 0x60 {
                    push_only = false;
                }
            }
            Err(_) => {
                last = None;
                push_only = false;
                break;
            }
        }
    }
    (last, push_only)
}
