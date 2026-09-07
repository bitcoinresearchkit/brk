use brk_error::Error;
use brk_types::{BlockTemplateDiffEntry, FeeRate, Sats, TxOut, TxidPrefix, Vin};

use super::*;
use crate::{
    state::TxEntry,
    test_support::{fake_entry_info, fake_tx, p2wpkh_script},
};

fn insert_tx(mempool: &Mempool, seed: u8, fee: u64, vsize: u64) -> Txid {
    let tx = fake_tx(seed, &[None], &[(p2wpkh_script(seed + 1), 1_234)]);
    let txid = tx.txid;
    let info = fake_entry_info(txid, fee, vsize);
    let entry = TxEntry::new(&info, vsize, false);
    let mut state = mempool.test_state_lock().write();
    state.txs.insert(tx, entry);
    txid
}

#[test]
fn block_template_hash_matches_next_block_hash() {
    let mempool = Mempool::for_test();
    let txid = insert_tx(&mempool, 0xA0, 1_234, 100);
    mempool.test_tick(&[txid], FeeRate::new(1.0));

    let template = mempool.block_template().unwrap();
    assert_eq!(template.hash, mempool.next_block_hash().unwrap());
    assert_eq!(template.transactions.len(), 1);
    assert_eq!(template.transactions[0].txid, txid);
}

#[test]
fn block_template_source_tracks_snapshot_and_body_changes() {
    let mempool = Mempool::for_test();
    let txid = insert_tx(&mempool, 0xA7, 1_234, 100);
    mempool.test_tick(&[txid], FeeRate::new(1.0));
    let initial = mempool.block_template_source();

    let prefix = TxidPrefix::from(&txid);
    let prevout = TxOut::from((p2wpkh_script(0xA8), Sats::from(2_000u64)));
    mempool
        .test_state_lock()
        .write()
        .txs
        .apply_fills(&prefix, vec![(Vin::from(0usize), prevout)]);
    let after_body_change = mempool.block_template_source();
    assert!(
        initial == after_body_change,
        "unpublished fills cannot change a published template"
    );

    mempool.test_tick(&[txid], FeeRate::new(2.0));
    let after_snapshot_change = mempool.block_template_source();
    assert!(after_body_change != after_snapshot_change);
}

#[test]
fn block_template_diff_round_trip_reconstructs_t1_from_t0() {
    // T0: pool has two txs, both in gbt -> block 0.
    let mempool = Mempool::for_test();
    let txid_a = insert_tx(&mempool, 0xA1, 1_111, 100);
    let txid_b = insert_tx(&mempool, 0xA2, 2_222, 100);
    mempool.test_tick(&[txid_a, txid_b], FeeRate::new(1.0));
    let t0 = mempool.block_template().unwrap();

    // T1: add a third tx, advance gbt. block_template_diff(t0.hash) must
    // be reconstructible into the new block 0 ordering by combining the
    // retained prior-indexed bodies from T0 with the New bodies inline.
    let txid_c = insert_tx(&mempool, 0xA3, 3_333, 100);
    mempool.test_tick(&[txid_a, txid_b, txid_c], FeeRate::new(1.0));
    let t1 = mempool.block_template().unwrap();

    let diff = mempool
        .block_template_diff(t0.hash)
        .unwrap()
        .expect("t0 is still in history");
    assert_eq!(diff.since, t0.hash);
    assert_eq!(diff.hash, t1.hash);

    let mut reconstructed = Vec::with_capacity(diff.order.len());
    for entry in &diff.order {
        match entry {
            BlockTemplateDiffEntry::Retained(idx) => {
                reconstructed.push(t0.transactions[*idx as usize].clone());
            }
            BlockTemplateDiffEntry::New(tx) => reconstructed.push(tx.clone()),
        }
    }
    let expected: Vec<_> = t1.transactions.iter().map(|tx| tx.txid).collect();
    let got: Vec<_> = reconstructed.iter().map(|tx| tx.txid).collect();
    assert_eq!(got, expected, "diff round-trips back into T1 ordering");
    assert!(diff.removed.is_empty());
}

#[test]
fn block_template_diff_removed_lists_evicted_txs() {
    let mempool = Mempool::for_test();
    let txid_a = insert_tx(&mempool, 0xA4, 1_111, 100);
    let txid_b = insert_tx(&mempool, 0xA5, 2_222, 100);
    mempool.test_tick(&[txid_a, txid_b], FeeRate::new(1.0));
    let t0 = mempool.block_template().unwrap();

    // T1: txid_a no longer in gbt.
    mempool.test_tick(&[txid_b], FeeRate::new(1.0));
    let diff = mempool.block_template_diff(t0.hash).unwrap().unwrap();
    assert_eq!(diff.removed, vec![txid_a]);
}

#[test]
fn block_template_diff_preserves_reordering_and_prior_removal_order() {
    let mempool = Mempool::for_test();
    let a = insert_tx(&mempool, 30, 100, 100);
    let b = insert_tx(&mempool, 31, 100, 100);
    let c = insert_tx(&mempool, 32, 100, 100);
    let d = insert_tx(&mempool, 33, 100, 100);
    let added = insert_tx(&mempool, 34, 100, 100);
    mempool.test_tick(&[a, b, c, d], FeeRate::new(1.0));
    let before = mempool.next_block_hash().unwrap();

    mempool.test_tick(&[d, added, b], FeeRate::new(1.0));
    let diff = mempool.block_template_diff(before).unwrap().unwrap();
    assert_eq!(diff.removed, [a, c]);
    assert_eq!(diff.order.len(), 3);
    assert!(matches!(diff.order[0], BlockTemplateDiffEntry::Retained(3)));
    assert!(matches!(&diff.order[1], BlockTemplateDiffEntry::New(tx) if tx.txid == added));
    assert!(matches!(diff.order[2], BlockTemplateDiffEntry::Retained(1)));

    mempool.test_tick(&[], FeeRate::new(1.0));
    let empty = mempool.block_template_diff(before).unwrap().unwrap();
    assert!(empty.order.is_empty());
    assert_eq!(empty.removed, [a, b, c, d]);
}

#[test]
fn block_template_diff_unknown_since_returns_none() {
    let mempool = Mempool::for_test();
    mempool.test_tick(&[], FeeRate::new(1.0));
    let bogus = NextBlockHash::new(0xDEAD_BEEF);
    assert!(mempool.resolve_block_template_diff(bogus).is_none());
    assert!(mempool.block_template_diff(bogus).unwrap().is_none());
}

#[test]
fn resolved_template_and_diff_keep_the_validated_publication_after_history_eviction() {
    let mempool = Mempool::for_test();
    let first = insert_tx(&mempool, 0xB0, 1_000, 100);
    let mut txids = vec![first];
    mempool.test_tick(&txids, FeeRate::new(1.0));
    let since = mempool.next_block_hash().unwrap();
    let source = mempool.block_template_source();
    let resolved = mempool
        .resolve_block_template_diff(since)
        .expect("initial template in history");

    for seed in 0xB1..=0xBB {
        txids.push(insert_tx(&mempool, seed, 1_000, 100));
        mempool.test_tick(&txids, FeeRate::new(1.0));
    }
    assert!(mempool.resolve_block_template_diff(since).is_none());

    let (diff, _) = mempool.block_template_diff_resolved(resolved).unwrap();
    assert_eq!(diff.since, since);
    assert_eq!(diff.hash, since);
    assert_eq!(diff.order.len(), 1);
    let template = source.build().unwrap();
    assert_eq!(template.hash, since);
    assert_eq!(template.transactions.len(), 1);
    assert_eq!(template.transactions[0].txid, first);
}

#[test]
fn block_template_empty_pool_has_no_transactions() {
    let mempool = Mempool::for_test();
    mempool.test_tick(&[], FeeRate::new(2.0));
    let template = mempool.block_template().unwrap();
    assert!(template.transactions.is_empty());
}

#[test]
fn body_fills_publish_a_new_identity_and_diff_reconstructs_every_field() {
    let mempool = Mempool::for_test();
    let changed = insert_tx(&mempool, 10, 100, 100);
    let stable = insert_tx(&mempool, 11, 100, 100);
    let ids = [changed, stable];
    mempool.test_tick(&ids, FeeRate::new(1.0));
    let before = mempool.block_template().unwrap();
    let published = mempool.snapshot();
    mempool.test_state_lock().write().txs.apply_fills(
        &TxidPrefix::from(changed),
        vec![(
            Vin::from(0usize),
            TxOut::from((p2wpkh_script(12), Sats::from(2_000u64))),
        )],
    );
    assert!(
        published.template_transactions()[0].input[0]
            .prevout
            .is_none()
    );
    assert_eq!(
        serde_json::to_vec(&mempool.block_template().unwrap()).unwrap(),
        serde_json::to_vec(&before).unwrap()
    );
    // Body changes alone must rebuild: no GBT, membership or fee-floor change.
    mempool
        .rebuilder()
        .tick(mempool.test_state_lock(), &ids, FeeRate::new(1.0), false);
    let after = mempool.block_template().unwrap();
    assert_ne!(before.hash, after.hash);
    assert!(after.transactions[0].input[0].prevout.is_some());
    assert!(Arc::ptr_eq(
        &published.template_transactions()[1],
        &mempool.snapshot().template_transactions()[1]
    ));
    let diff = mempool.block_template_diff(before.hash).unwrap().unwrap();
    assert!(diff.removed.is_empty());
    assert!(matches!(&diff.order[0], BlockTemplateDiffEntry::New(_)));
    assert!(matches!(
        &diff.order[1],
        BlockTemplateDiffEntry::Retained(1)
    ));
    let reconstructed: Vec<_> = diff
        .order
        .into_iter()
        .map(|entry| match entry {
            BlockTemplateDiffEntry::Retained(index) => before.transactions[index as usize].clone(),
            BlockTemplateDiffEntry::New(tx) => tx,
        })
        .collect();
    assert_eq!(
        serde_json::to_vec(&reconstructed).unwrap(),
        serde_json::to_vec(&after.transactions).unwrap()
    );
}

#[test]
fn published_bodies_survive_removal_and_incomplete_selection_is_not_served() {
    let mempool = Mempool::for_test();
    let txid = insert_tx(&mempool, 20, 100, 100);
    mempool.test_tick(&[txid], FeeRate::new(1.0));
    let before = mempool.block_template().unwrap();
    mempool
        .test_state_lock()
        .write()
        .txs
        .remove_by_prefix(&TxidPrefix::from(txid));
    assert_eq!(
        serde_json::to_vec(&mempool.block_template().unwrap()).unwrap(),
        serde_json::to_vec(&before).unwrap()
    );
    mempool.test_tick(&[txid], FeeRate::new(1.0));
    assert!(matches!(
        mempool.block_template(),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        mempool.block_template_diff(before.hash),
        Err(Error::StateUpdating)
    ));
}
