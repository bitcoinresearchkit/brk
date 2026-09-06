use brk_types::{Sats, TxOut, TxidPrefix, Vin};

use super::*;
use crate::test_support::{fake_entry_info, fake_tx, p2wpkh_script};

#[test]
fn publication_requires_complete_membership_and_prevouts() {
    let mut state = State::default();
    let tip = BlockHash::default();
    assert!(state.ensure_published_at(&tip).is_err());
    state.publish_at(tip, &[]);
    assert!(
        state.ensure_published_at(&tip).is_ok(),
        "observed empty is available"
    );

    let tx = fake_tx(1, &[None], &[]);
    let txid = tx.txid;
    let prefix = TxidPrefix::from(txid);
    let entry = TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false);
    state.txs.insert(tx, entry);
    state.publish_at(tip, &[txid]);
    assert!(
        state.ensure_published_at(&tip).is_err(),
        "unresolved spends hide address activity"
    );

    state.txs.apply_fills(
        &prefix,
        vec![(
            Vin::from(0usize),
            TxOut::from((p2wpkh_script(2), Sats::from(1_000u64))),
        )],
    );
    state.publish_at(tip, &[]);
    assert!(
        state.ensure_published_at(&tip).is_err(),
        "extra local body is not the observed set"
    );
    state.publish_at(tip, &[txid, fake_tx(2, &[], &[]).txid]);
    assert!(
        state.ensure_published_at(&tip).is_err(),
        "capped or failed downloads are incomplete"
    );
    state.publish_at(tip, &[txid]);
    assert!(state.ensure_published_at(&tip).is_ok());
}
