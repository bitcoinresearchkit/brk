use brk_types::{Sats, VSize};

use super::*;
use crate::{
    state::TxEntry,
    test_support::{fake_entry_info, fake_tx, p2wpkh_script},
};

fn state_with(seeds: &[u8]) -> (RwLock<State>, Vec<Txid>) {
    let state = RwLock::new(State::default());
    let mut txids = Vec::with_capacity(seeds.len());
    for &seed in seeds {
        let tx = fake_tx(seed, &[], &[(p2wpkh_script(seed), 1_000)]);
        let txid = tx.txid;
        let entry = TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false);
        state.write().txs.insert(tx, entry);
        txids.push(txid);
    }
    (state, txids)
}

fn min_fee(sats: u64) -> FeeRate {
    FeeRate::from((Sats::from(sats), VSize::from(1_000u64)))
}

#[test]
fn first_tick_always_builds() {
    let rebuilder = Rebuilder::default();
    let state = RwLock::new(State::default());

    rebuilder.tick(&state, &[], min_fee(1), false);

    assert_eq!(rebuilder.rebuild_count(), 1);
}

#[test]
fn identical_inputs_reuse_snapshot() {
    let rebuilder = Rebuilder::default();
    let (state, txids) = state_with(&[1, 2]);
    rebuilder.tick(&state, &txids, min_fee(1), true);

    rebuilder.tick(&state, &txids, min_fee(1), false);

    assert_eq!(rebuilder.rebuild_count(), 1);
}

#[test]
fn reordered_template_rebuilds() {
    let rebuilder = Rebuilder::default();
    let (state, mut txids) = state_with(&[1, 2]);
    rebuilder.tick(&state, &txids, min_fee(1), true);
    txids.reverse();

    rebuilder.tick(&state, &txids, min_fee(1), false);

    assert_eq!(rebuilder.rebuild_count(), 2);
}

#[test]
fn changed_min_fee_rebuilds() {
    let rebuilder = Rebuilder::default();
    let (state, txids) = state_with(&[1]);
    rebuilder.tick(&state, &txids, min_fee(1), true);

    rebuilder.tick(&state, &txids, min_fee(2), false);

    assert_eq!(rebuilder.rebuild_count(), 2);
}

#[test]
fn changed_pool_rebuilds() {
    let rebuilder = Rebuilder::default();
    let (state, txids) = state_with(&[1]);
    rebuilder.tick(&state, &txids, min_fee(1), true);

    rebuilder.tick(&state, &txids, min_fee(1), true);

    assert_eq!(rebuilder.rebuild_count(), 2);
}
