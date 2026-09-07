use brk_types::{FeeRate, Sats, TxOut, Txid, VSize};

use super::*;
use crate::{
    AddedKind,
    cycle::CycleDiff,
    steps::preparer::{TxAddition, TxsPulled},
    test_support::{fake_entry_info, fake_tx, p2wpkh_script},
};

fn fresh_addition(seed: u8, fee: u64, vsize: u64) -> (TxAddition, Txid) {
    let prev = Some(TxOut::from((p2wpkh_script(seed), Sats::from(2_500u64))));
    let tx = fake_tx(seed, &[prev], &[(p2wpkh_script(seed + 1), 1_234)]);
    let txid = tx.txid;
    let info = fake_entry_info(txid, fee, vsize);
    let entry = TxEntry::new(&info, vsize, false);
    (TxAddition::Fresh { tx, entry }, txid)
}

fn fresh_pulled(addition: TxAddition) -> TxsPulled {
    TxsPulled {
        live_len: 1,
        added: vec![addition],
        removed: vec![],
    }
}

#[test]
fn publish_one_inserts_into_all_stores() {
    let lock = RwLock::new(State::default());
    lock.write().published_tip = Some(Default::default());
    let snapshot = Snapshot::default();
    let mut diff = CycleDiff::default();
    let (addition, txid) = fresh_addition(0xC0, 200, 100);

    apply(&lock, &snapshot, fresh_pulled(addition), &mut diff);

    let state = lock.read();
    assert!(
        state.published_tip.is_none(),
        "mutation closes the old publication"
    );
    assert!(state.txs.contains(&txid));
    assert_eq!(diff.added.len(), 1);
    assert_eq!(diff.added[0].txid, txid);
}

#[test]
fn revived_path_exhumes_body_from_graveyard() {
    let lock = RwLock::new(State::default());
    let snapshot = Snapshot::default();
    let (addition, txid) = fresh_addition(0xC1, 300, 100);
    let TxAddition::Fresh { tx, entry } = addition else {
        unreachable!();
    };
    // Pre-load the graveyard with this tx, then submit a Revived
    // addition that re-publishes it without a raw body.
    let rate = FeeRate::from((entry.fee, entry.vsize));
    lock.write()
        .graveyard
        .bury(tx, entry.clone(), rate, TxRemoval::Vanished);

    let mut diff = CycleDiff::default();
    apply(
        &lock,
        &snapshot,
        fresh_pulled(TxAddition::Revived { entry }),
        &mut diff,
    );

    let state = lock.read();
    assert!(state.txs.contains(&txid), "revived tx republished");
    assert!(state.graveyard.get(&txid).is_none(), "tomb consumed");
    assert_eq!(diff.added.len(), 1);
    assert!(matches!(diff.added[0].kind, AddedKind::Revived));
}

#[test]
fn revived_with_empty_graveyard_is_dropped() {
    let lock = RwLock::new(State::default());
    let snapshot = Snapshot::default();
    let info = fake_entry_info(Txid::COINBASE, 100, 100);
    let entry = TxEntry::new(&info, 100, false);

    let mut diff = CycleDiff::default();
    apply(
        &lock,
        &snapshot,
        fresh_pulled(TxAddition::Revived { entry }),
        &mut diff,
    );

    let state = lock.read();
    assert!(!state.txs.contains(&Txid::COINBASE));
    assert!(diff.added.is_empty(), "no body, no event");
}

#[test]
fn bury_preserves_chunk_rate_from_snapshot() {
    let lock = RwLock::new(State::default());
    let (addition, txid) = fresh_addition(0xC2, 100, 100);

    // Publish first to plant the tx, with a fee-rate that differs
    // from the snapshot's stub rate so we can tell them apart.
    apply(
        &lock,
        &Snapshot::default(),
        fresh_pulled(addition),
        &mut CycleDiff::default(),
    );
    let isolated_rate = FeeRate::from((Sats::from(100u64), VSize::from(100u64)));

    let cpfp_rate = FeeRate::from((Sats::from(500u64), VSize::from(100u64)));
    let prefix = TxidPrefix::from(&txid);
    let snapshot = Snapshot::for_test_with_chunk_rates(&[(prefix, cpfp_rate, txid)]);

    let mut diff = CycleDiff::default();
    apply(
        &lock,
        &snapshot,
        TxsPulled {
            live_len: 0,
            added: vec![],
            removed: vec![(prefix, TxRemoval::Vanished)],
        },
        &mut diff,
    );

    assert_eq!(diff.removed.len(), 1);
    assert_eq!(diff.removed[0].chunk_rate, cpfp_rate);
    assert_ne!(diff.removed[0].chunk_rate, isolated_rate);
    let state = lock.read();
    assert_eq!(state.graveyard.get(&txid).unwrap().chunk_rate, cpfp_rate);
}

#[test]
fn bury_falls_back_to_isolated_rate_when_snapshot_misses() {
    let lock = RwLock::new(State::default());
    let (addition, txid) = fresh_addition(0xC3, 700, 100);
    apply(
        &lock,
        &Snapshot::default(),
        fresh_pulled(addition),
        &mut CycleDiff::default(),
    );

    let isolated_rate = FeeRate::from((Sats::from(700u64), VSize::from(100u64)));
    let prefix = TxidPrefix::from(&txid);

    let mut diff = CycleDiff::default();
    apply(
        &lock,
        &Snapshot::default(),
        TxsPulled {
            live_len: 0,
            added: vec![],
            removed: vec![(prefix, TxRemoval::Vanished)],
        },
        &mut diff,
    );

    assert_eq!(diff.removed[0].chunk_rate, isolated_rate);
}
