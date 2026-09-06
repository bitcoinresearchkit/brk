use bitcoin::{
    Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness, absolute::LockTime,
    consensus, hashes::Hash, transaction::Version,
};

use super::*;

#[test]
fn content_identity_changes_with_witness_when_txid_does_not() {
    let mut transaction = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: bitcoin::Txid::from_byte_array([1; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::from_slice(&[b"first"]),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(1),
            script_pubkey: ScriptBuf::new(),
        }],
    };
    let first_txid = transaction.compute_txid();
    let first_hash = content_hash(&consensus::serialize(&transaction));

    transaction.input[0].witness = Witness::from_slice(&[b"second"]);
    assert_eq!(transaction.compute_txid(), first_txid);
    assert_ne!(
        content_hash(&consensus::serialize(&transaction)),
        first_hash
    );
}
