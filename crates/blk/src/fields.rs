use std::cell::OnceCell;

use bitcoin::{
    Address, Block, Network, ScriptBuf, Transaction, TxIn, TxOut, consensus::encode::serialize_hex,
    hex::DisplayHex,
};
use brk_error::{Error, Result};
use brk_types::ReadBlock;
use serde_json::{Value, json};

use crate::path::{Path, Step};

pub struct Ctx<'a> {
    block: &'a ReadBlock,
    size_weight: OnceCell<(usize, usize)>,
}

impl<'a> Ctx<'a> {
    pub fn new(block: &'a ReadBlock) -> Self {
        Self {
            block,
            size_weight: OnceCell::new(),
        }
    }

    pub fn resolve(&self, path: &Path) -> Result<Value> {
        let (step, rest) = pop(&path.steps)?;
        let b = self.block;
        let raw: &Block = b;
        let scalar = |v| scalar_leaf(v, step, rest);
        match step.name.as_str() {
            "height" => scalar(json!(*b.height())),
            "hash" => scalar(json!(b.hash().to_string())),
            "time" => scalar(json!(b.header.time)),
            "version" => scalar(json!(b.header.version.to_consensus())),
            "version_hex" => scalar(json!(format!(
                "{:08x}",
                b.header.version.to_consensus() as u32
            ))),
            "bits" => scalar(json!(b.header.bits.to_consensus())),
            "nonce" => scalar(json!(b.header.nonce)),
            "prev" => scalar(json!(b.header.prev_blockhash.to_string())),
            "merkle" => scalar(json!(b.header.merkle_root.to_string())),
            "difficulty" => scalar(json!(b.header.difficulty_float())),
            "txs" => scalar(json!(b.txdata.len())),
            "n_inputs" => scalar(json!(
                b.txdata.iter().map(|tx| tx.input.len()).sum::<usize>()
            )),
            "n_outputs" => scalar(json!(
                b.txdata.iter().map(|tx| tx.output.len()).sum::<usize>()
            )),
            "witness_txs" => scalar(json!(
                b.txdata.iter().filter(|tx| tx_has_witness(tx)).count()
            )),
            "size" => scalar(json!(self.size_and_weight().0)),
            "weight" => scalar(json!(self.size_and_weight().1)),
            "strippedsize" => {
                let (size, weight) = self.size_and_weight();
                scalar(json!((weight - size) / 3))
            }
            "subsidy" => scalar(json!(subsidy_sats(*b.height()))),
            "header_hex" => scalar(json!(serialize_hex(&b.header))),
            "hex" => scalar(json!(serialize_hex(raw))),
            "coinbase" => scalar(json!(b.coinbase_tag().as_str())),
            "tx" => pick(&b.txdata, step, rest, |i, tx| resolve_tx(tx, i == 0, rest)),
            other => Err(unknown("block", other)),
        }
    }

    pub fn resolve_str(&self, path: &Path) -> Result<String> {
        Ok(match self.resolve(path)? {
            Value::String(s) => s,
            other => other.to_string(),
        })
    }

    pub fn full(&self) -> Value {
        let b = self.block;
        let (size, weight) = self.size_and_weight();
        let tx: Vec<Value> = b
            .txdata
            .iter()
            .enumerate()
            .map(|(i, tx)| tx_to_value(tx, i == 0))
            .collect();
        json!({
            "height": *b.height(),
            "hash": b.hash().to_string(),
            "version": b.header.version.to_consensus(),
            "version_hex": format!("{:08x}", b.header.version.to_consensus() as u32),
            "merkle": b.header.merkle_root.to_string(),
            "time": b.header.time,
            "nonce": b.header.nonce,
            "bits": b.header.bits.to_consensus(),
            "difficulty": b.header.difficulty_float(),
            "prev": b.header.prev_blockhash.to_string(),
            "txs": b.txdata.len(),
            "n_inputs": b.txdata.iter().map(|t| t.input.len()).sum::<usize>(),
            "n_outputs": b.txdata.iter().map(|t| t.output.len()).sum::<usize>(),
            "witness_txs": b.txdata.iter().filter(|t| tx_has_witness(t)).count(),
            "size": size,
            "strippedsize": (weight - size) / 3,
            "weight": weight,
            "subsidy": subsidy_sats(*b.height()),
            "coinbase": b.coinbase_tag().as_str(),
            "header_hex": serialize_hex(&b.header),
            "tx": tx,
        })
    }

    fn size_and_weight(&self) -> (usize, usize) {
        *self
            .size_weight
            .get_or_init(|| self.block.total_size_and_weight())
    }
}

fn resolve_tx(tx: &Transaction, is_coinbase: bool, steps: &[Step]) -> Result<Value> {
    if steps.is_empty() {
        return Ok(tx_to_value(tx, is_coinbase));
    }
    let (step, rest) = pop(steps)?;
    let scalar = |v| scalar_leaf(v, step, rest);
    match step.name.as_str() {
        "txid" => scalar(json!(tx.compute_txid().to_string())),
        "wtxid" => scalar(json!(tx.compute_wtxid().to_string())),
        "version" => scalar(json!(tx.version.0)),
        "locktime" => scalar(json!(tx.lock_time.to_consensus_u32())),
        "size" => scalar(json!(tx.total_size())),
        "base_size" => scalar(json!(tx.base_size())),
        "vsize" => scalar(json!(tx.vsize())),
        "weight" => scalar(json!(tx.weight().to_wu())),
        "inputs" => scalar(json!(tx.input.len())),
        "outputs" => scalar(json!(tx.output.len())),
        "is_coinbase" => scalar(json!(is_coinbase)),
        "has_witness" => scalar(json!(tx_has_witness(tx))),
        "is_rbf" => scalar(json!(tx_is_rbf(tx))),
        "total_out" => scalar(json!(tx_total_out(tx))),
        "hex" => scalar(json!(serialize_hex(tx))),
        "vin" => pick(&tx.input, step, rest, |j, vin| {
            resolve_vin(vin, is_coinbase && j == 0, rest)
        }),
        "vout" => pick(&tx.output, step, rest, |_, vout| resolve_vout(vout, rest)),
        other => Err(unknown("tx", other)),
    }
}

fn resolve_vin(vin: &TxIn, is_coinbase: bool, steps: &[Step]) -> Result<Value> {
    if steps.is_empty() {
        return Ok(vin_to_value(vin, is_coinbase));
    }
    let (step, rest) = pop(steps)?;
    let scalar = |v| scalar_leaf(v, step, rest);
    match step.name.as_str() {
        "prev_txid" => scalar(json!(vin.previous_output.txid.to_string())),
        "prev_vout" => scalar(json!(vin.previous_output.vout)),
        "sequence" => scalar(json!(vin.sequence.0)),
        "script_sig" => scalar(json!(vin.script_sig.to_hex_string())),
        "script_sig_asm" => scalar(json!(vin.script_sig.to_asm_string())),
        "witness" => scalar(witness_to_value(vin)),
        "has_witness" => scalar(json!(!vin.witness.is_empty())),
        "is_rbf" => scalar(json!(vin.sequence.is_rbf())),
        "coinbase" => scalar(json!(is_coinbase)),
        other => Err(unknown("vin", other)),
    }
}

fn resolve_vout(vout: &TxOut, steps: &[Step]) -> Result<Value> {
    if steps.is_empty() {
        return Ok(vout_to_value(vout));
    }
    let (step, rest) = pop(steps)?;
    let scalar = |v| scalar_leaf(v, step, rest);
    match step.name.as_str() {
        "value" => scalar(json!(vout.value.to_sat())),
        "script_pubkey" => scalar(json!(vout.script_pubkey.to_hex_string())),
        "script_pubkey_asm" => scalar(json!(vout.script_pubkey.to_asm_string())),
        "type" => scalar(json!(script_type(&vout.script_pubkey))),
        "address" => scalar(address_value(&vout.script_pubkey)),
        other => Err(unknown("vout", other)),
    }
}

fn pick<T>(
    items: &[T],
    step: &Step,
    _rest: &[Step],
    mut resolve: impl FnMut(usize, &T) -> Result<Value>,
) -> Result<Value> {
    match step.index {
        Some(i) => {
            let item = items
                .get(i)
                .ok_or_else(|| out_of_range(&step.name, i, items.len()))?;
            resolve(i, item)
        }
        None => Ok(Value::Array(
            items
                .iter()
                .enumerate()
                .map(|(i, item)| resolve(i, item))
                .collect::<Result<_>>()?,
        )),
    }
}

fn pop(steps: &[Step]) -> Result<(&Step, &[Step])> {
    steps
        .split_first()
        .ok_or_else(|| Error::Parse("empty path segment".into()))
}

fn scalar_leaf(v: Value, step: &Step, rest: &[Step]) -> Result<Value> {
    if step.index.is_some() {
        return Err(Error::Parse(format!("'{}' is not an array", step.name)));
    }
    if !rest.is_empty() {
        return Err(Error::Parse(format!(
            "'{}' is a scalar; nothing to drill into",
            step.name
        )));
    }
    Ok(v)
}

fn out_of_range(name: &str, i: usize, len: usize) -> Error {
    Error::Parse(format!("{name}.{i} out of range (len {len})"))
}

fn unknown(level: &str, name: &str) -> Error {
    Error::Parse(format!(
        "unknown {level} field '{name}' (run `blk --help` for the list)"
    ))
}

fn tx_to_value(tx: &Transaction, is_coinbase: bool) -> Value {
    let vin: Vec<Value> = tx
        .input
        .iter()
        .enumerate()
        .map(|(j, v)| vin_to_value(v, is_coinbase && j == 0))
        .collect();
    let vout: Vec<Value> = tx.output.iter().map(vout_to_value).collect();
    json!({
        "txid": tx.compute_txid().to_string(),
        "wtxid": tx.compute_wtxid().to_string(),
        "version": tx.version.0,
        "locktime": tx.lock_time.to_consensus_u32(),
        "size": tx.total_size(),
        "base_size": tx.base_size(),
        "vsize": tx.vsize(),
        "weight": tx.weight().to_wu(),
        "inputs": tx.input.len(),
        "outputs": tx.output.len(),
        "is_coinbase": is_coinbase,
        "has_witness": tx_has_witness(tx),
        "is_rbf": tx_is_rbf(tx),
        "total_out": tx_total_out(tx),
        "hex": serialize_hex(tx),
        "vin": vin,
        "vout": vout,
    })
}

fn vin_to_value(vin: &TxIn, is_coinbase: bool) -> Value {
    json!({
        "prev_txid": vin.previous_output.txid.to_string(),
        "prev_vout": vin.previous_output.vout,
        "sequence": vin.sequence.0,
        "script_sig": vin.script_sig.to_hex_string(),
        "script_sig_asm": vin.script_sig.to_asm_string(),
        "witness": witness_to_value(vin),
        "has_witness": !vin.witness.is_empty(),
        "is_rbf": vin.sequence.is_rbf(),
        "coinbase": is_coinbase,
    })
}

fn vout_to_value(vout: &TxOut) -> Value {
    json!({
        "value": vout.value.to_sat(),
        "script_pubkey": vout.script_pubkey.to_hex_string(),
        "script_pubkey_asm": vout.script_pubkey.to_asm_string(),
        "type": script_type(&vout.script_pubkey),
        "address": address_value(&vout.script_pubkey),
    })
}

fn tx_has_witness(tx: &Transaction) -> bool {
    tx.input.iter().any(|i| !i.witness.is_empty())
}

fn tx_is_rbf(tx: &Transaction) -> bool {
    tx.input.iter().any(|i| i.sequence.is_rbf())
}

fn tx_total_out(tx: &Transaction) -> u64 {
    tx.output.iter().map(|o| o.value.to_sat()).sum()
}

fn subsidy_sats(height: u32) -> u64 {
    let halvings = height / 210_000;
    if halvings >= 64 {
        0
    } else {
        (50 * 100_000_000u64) >> halvings
    }
}

fn witness_to_value(vin: &TxIn) -> Value {
    Value::Array(
        vin.witness
            .iter()
            .map(|w| Value::String(w.to_lower_hex_string()))
            .collect(),
    )
}

fn script_type(s: &ScriptBuf) -> &'static str {
    if s.is_p2pkh() {
        "p2pkh"
    } else if s.is_p2sh() {
        "p2sh"
    } else if s.is_p2wpkh() {
        "p2wpkh"
    } else if s.is_p2wsh() {
        "p2wsh"
    } else if s.is_p2tr() {
        "p2tr"
    } else if s.is_op_return() {
        "op_return"
    } else if s.is_p2pk() {
        "p2pk"
    } else {
        "unknown"
    }
}

fn address_value(s: &ScriptBuf) -> Value {
    Address::from_script(s, Network::Bitcoin)
        .map(|a| Value::String(a.to_string()))
        .unwrap_or(Value::Null)
}
