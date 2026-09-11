use super::*;
use crate::test_support::{fake_entry_info, fake_tx};

#[test]
fn membership_requires_the_exact_full_transaction_set() {
    let mut state = State::default();
    assert!(state.contains_all(&[]));
    let tx = fake_tx(1, &[None], &[]);
    let txid = tx.txid;
    state.txs.insert(
        tx,
        TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false),
    );
    assert!(!state.contains_all(&[]));
    assert!(!state.contains_all(&[txid, fake_tx(2, &[], &[]).txid]));
    assert!(!state.contains_all(&[fake_tx(2, &[], &[]).txid]));
    assert!(
        state.contains_all(&[txid]),
        "input resolution is separate from membership"
    );
}
