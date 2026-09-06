use bitcoin::{ScriptBuf, Transaction, TxIn, TxOut, absolute::LockTime, transaction::Version};

use super::*;

fn tx(seed: u32) -> Transaction {
    let mut input = TxIn::default();
    input.previous_output.vout = seed;
    Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![input],
        output: vec![TxOut {
            value: Amount::from_sat(1_000),
            script_pubkey: ScriptBuf::new(),
        }],
    }
}

fn row(tx: &Transaction, depends: Vec<i64>) -> BlockTemplateTransaction {
    BlockTemplateTransaction {
        data: bitcoin::consensus::encode::serialize_hex(tx),
        txid: tx.compute_txid().to_string(),
        hash: tx.compute_wtxid().to_string(),
        depends,
        fee: 100,
        sigops: 0,
        weight: tx.weight().to_wu(),
    }
}

#[test]
fn valid_template_preserves_bodies_and_resolves_parents() {
    let parent = tx(0);
    let mut child = tx(1);
    child.input[0].previous_output.txid = parent.compute_txid();
    child.input[0].previous_output.vout = 0;
    let actual = ClientInner::build_gbt(vec![row(&parent, vec![]), row(&child, vec![1])]).unwrap();
    assert_eq!(actual[0].tx, parent);
    assert_eq!(actual[1].tx, child);
    assert_eq!(actual[1].depends, vec![Txid::from(parent.compute_txid())]);
}

#[test]
fn rejects_metadata_mismatch_duplicate_ids_and_unbounded_weight() {
    let tx = tx(0);
    let mut cases = Vec::new();
    let mut altered = row(&tx, vec![]);
    altered.txid = "0".repeat(64);
    cases.push(altered);
    let mut altered = row(&tx, vec![]);
    altered.hash = "0".repeat(64);
    cases.push(altered);
    for weight in [0, tx.weight().to_wu() + 1, u64::MAX] {
        let mut altered = row(&tx, vec![]);
        altered.weight = weight;
        cases.push(altered);
    }
    let mut altered = row(&tx, vec![]);
    altered.data.push_str("00");
    cases.push(altered);
    for altered in cases {
        assert!(ClientInner::build_gbt(vec![altered]).is_err());
    }
    assert!(ClientInner::build_gbt(vec![row(&tx, vec![]), row(&tx, vec![])]).is_err());
    let mut large = row(&tx, vec![]);
    large.weight = BitcoinWeight::MAX_BLOCK.to_wu();
    let error = ClientInner::build_gbt(vec![large.clone(), large])
        .unwrap_err()
        .to_string();
    assert!(error.contains("aggregate weight"), "{error}");
}

#[test]
fn rejects_invalid_missing_and_future_dependencies() {
    let parent = tx(0);
    let mut child = tx(1);
    child.input[0].previous_output.txid = parent.compute_txid();
    for depends in [vec![-1], vec![0], vec![2], vec![i64::MAX], vec![]] {
        assert!(ClientInner::build_gbt(vec![row(&parent, vec![]), row(&child, depends)]).is_err());
    }
    assert!(ClientInner::build_gbt(vec![row(&child, vec![]), row(&parent, vec![])]).is_err());
    assert!(ClientInner::build_gbt(vec![row(&parent, vec![]), row(&tx(2), vec![1])]).is_err());
}

#[test]
fn repeated_parent_inputs_normalize_cores_repeated_dependency_indexes() {
    let mut parent = tx(0);
    parent.output.push(parent.output[0].clone());
    let mut child = tx(1);
    child.input[0].previous_output.txid = parent.compute_txid();
    child.input[0].previous_output.vout = 0;
    let mut second = child.input[0].clone();
    second.previous_output.vout = 1;
    child.input.push(second);
    let rows = ClientInner::build_gbt(vec![row(&parent, vec![]), row(&child, vec![1, 1])]).unwrap();
    assert_eq!(rows[1].depends, vec![Txid::from(parent.compute_txid())]);
}

#[test]
fn rejects_impossible_fee_and_output_totals() {
    let mut first = row(&tx(0), vec![]);
    first.fee = -1;
    assert!(ClientInner::build_gbt(vec![first.clone()]).is_err());
    first.fee = Amount::MAX_MONEY.to_sat() as i64;
    let mut second = row(&tx(1), vec![]);
    second.fee = first.fee;
    assert!(ClientInner::build_gbt(vec![first, second]).is_err());
    let mut tx = tx(0);
    tx.output[0].value = Amount::from_sat(u64::MAX);
    assert!(ClientInner::build_gbt(vec![row(&tx, vec![])]).is_err());
}
