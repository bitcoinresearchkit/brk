use std::sync::Arc;

use brk_types::{BlockHash, FeeRate, TxOutspend, Vout};

use super::{check_output_count, transaction_times_hash};
use crate::{
    Mempool, TxRemoval,
    state::TxEntry,
    test_support::{fake_entry_info, fake_tx, fake_txid, p2wpkh_script},
};

#[test]
fn bulk_outspends_resolve_fan_in_and_preserve_existing_spends() {
    let mempool = Mempool::for_test();
    let count = 1000;
    let parent = fake_tx(1, &[], &vec![(p2wpkh_script(1), 1_000); count]);
    let mut spender = fake_tx(2, &[None], &[(p2wpkh_script(2), 900)]);
    spender.input.resize(count, spender.input[0].clone());
    for (vin, input) in spender.input.iter_mut().enumerate() {
        input.txid = parent.txid;
        input.vout = Vout::from(count - vin - 1);
        input.prevout = Some(parent.output[count - vin - 1].clone());
    }
    let tip = BlockHash::default();
    {
        let mut state = mempool.test_state_lock().write();
        let entry = TxEntry::new(&fake_entry_info(spender.txid, 100, 100), 100, false);
        state
            .outpoint_spends
            .insert_spends(&spender, entry.txid_prefix());
        state.txs.insert(spender.clone(), entry);
        state.txs.insert(
            parent.clone(),
            TxEntry::new(&fake_entry_info(parent.txid, 100, 100), 100, false),
        );
        state.publish_at(tip, &[parent.txid, spender.txid]);
    }
    let outspends = mempool
        .outspends_if_present(&parent.txid, &tip)
        .unwrap()
        .unwrap();
    assert_eq!(outspends.len(), count);
    for (vout, outspend) in outspends.iter().enumerate() {
        assert_eq!(outspend.txid, Some(spender.txid));
        assert_eq!(outspend.vin, Some((count - vout - 1).into()));
    }
    let mut overlay = vec![TxOutspend::UNSPENT; count];
    overlay[0] = outspends[0].clone();
    overlay[0].txid = Some(fake_txid(3));
    mempool
        .merge_outspends(&parent.txid, &mut overlay, &tip)
        .unwrap();
    assert_eq!(overlay[0].txid, Some(fake_txid(3)));
    for outspend in &overlay[1..] {
        assert_eq!(outspend.txid, Some(spender.txid));
    }
}

#[test]
fn outspend_resolves_parent_and_spender_from_one_snapshot() {
    let mempool = Mempool::for_test();
    let parent = fake_tx(
        1,
        &[],
        &[(p2wpkh_script(1), 1_000), (p2wpkh_script(2), 2_000)],
    );
    let mut spender = fake_tx(2, &[None], &[(p2wpkh_script(2), 900)]);
    spender.input[0].txid = parent.txid;
    spender.input[0].vout = Vout::ZERO;
    spender.input[0].prevout = Some(parent.output[0].clone());

    let parent_entry = TxEntry::new(&fake_entry_info(parent.txid, 100, 100), 100, false);
    let spender_entry = TxEntry::new(&fake_entry_info(spender.txid, 100, 100), 100, false);
    let mut state = mempool.test_state_lock().write();
    state
        .outpoint_spends
        .insert_spends(&spender, spender_entry.txid_prefix());
    state.txs.insert(parent.clone(), parent_entry);
    state.txs.insert(spender.clone(), spender_entry);
    let tip = BlockHash::default();
    state.publish_at(tip, &[parent.txid, spender.txid]);
    drop(state);

    let outspend = mempool
        .outspend_if_present(&parent.txid, Vout::ZERO, &tip)
        .unwrap()
        .unwrap();
    assert!(outspend.spent);
    assert_eq!(outspend.txid, Some(spender.txid));
    assert_eq!(outspend.vin, Some(0usize.into()));
    assert!(outspend.status.is_some_and(|status| !status.confirmed));

    let outspends = mempool
        .outspends_if_present(&parent.txid, &tip)
        .unwrap()
        .unwrap();
    assert_eq!(outspends.len(), 2);
    assert!(outspends[0].spent);
    assert!(!outspends[1].spent);

    assert!(
        !mempool
            .outspend_if_present(&parent.txid, Vout::from(2usize), &tip)
            .unwrap()
            .unwrap()
            .spent
    );
    assert!(
        mempool
            .outspend_if_present(&fake_txid(3), Vout::ZERO, &tip)
            .unwrap()
            .is_none()
    );
}

#[test]
fn transaction_reads_reject_unready_or_wrong_chain_and_share_vanished_bodies() {
    let mempool = Mempool::for_test();
    let tip = BlockHash::default();
    let wrong_tip = "11".repeat(32).parse().unwrap();
    let tx = fake_tx(1, &[], &[(p2wpkh_script(1), 1_234)]);
    let txid = tx.txid;
    let assert_unready = |tip| {
        assert!(mempool.transaction(&txid, tip).is_err());
        assert!(mempool.contains_txid(&txid, tip).is_err());
        assert!(mempool.outspend_if_present(&txid, Vout::ZERO, tip).is_err());
        assert!(mempool.outspends_if_present(&txid, tip).is_err());
        assert!(mempool.outspend(&txid, Vout::ZERO, tip).is_err());
        assert!(mempool.lookup_spender(&txid, Vout::ZERO, tip).is_err());
        assert!(
            mempool
                .merge_outspends(&txid, &mut [TxOutspend::UNSPENT], tip)
                .is_err()
        );
    };
    assert_unready(&tip);
    {
        let mut state = mempool.test_state_lock().write();
        state.txs.insert(
            tx,
            TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false),
        );
        state.publish_at(tip, &[txid]);
    }
    assert_unready(&wrong_tip);
    let body = mempool.transaction(&txid, &tip).unwrap().unwrap();
    assert!(mempool.contains_txid(&txid, &tip).unwrap());
    {
        let mut state = mempool.test_state_lock().write();
        state.published_tip = None;
        let record = state.txs.remove_by_prefix(&txid.into()).unwrap();
        state.graveyard.bury(
            record.tx,
            record.entry,
            FeeRate::new(1.0),
            TxRemoval::Vanished,
        );
        state.publish_at(tip, &[]);
    }
    assert!(!mempool.contains_txid(&txid, &tip).unwrap());
    let vanished = mempool.transaction(&txid, &tip).unwrap().unwrap();
    assert!(Arc::ptr_eq(&body, &vanished));
    assert!(mempool.outspends_if_present(&txid, &tip).unwrap().is_none());
    assert_eq!(body.output.len(), 1);
}

#[test]
fn output_offsets_are_checked_before_conversion_or_allocation() {
    assert!(check_output_count(usize::from(Vout::MAX) + 1).is_ok());
    assert!(check_output_count(usize::from(Vout::MAX) + 2).is_err());
    assert!(check_output_count(usize::MAX).is_err());
}

#[test]
fn aggregates_require_completed_observations_including_empty_results() {
    let mempool = Mempool::for_test();
    let txid = fake_txid(1);
    let check = |ready| {
        assert_eq!(mempool.info().is_ok(), ready);
        assert_eq!(mempool.txids().is_ok(), ready);
        assert_eq!(mempool.txids_hash().is_ok(), ready);
        assert_eq!(mempool.txids_with_hash().is_ok(), ready);
        assert_eq!(mempool.recent_txs().is_ok(), ready);
        assert_eq!(mempool.transaction_times(&[txid]).is_ok(), ready);
        assert_eq!(mempool.transaction_times_with_hash(&[txid]).is_ok(), ready);
    };
    check(false);
    mempool
        .test_state_lock()
        .write()
        .publish_at(BlockHash::default(), &[]);
    check(true);
    assert!(mempool.txids().unwrap().is_empty());
    assert_eq!(mempool.transaction_times(&[txid]).unwrap(), vec![0]);
    mempool.test_state_lock().write().published_tip = None;
    check(false);
    let tx = fake_tx(1, &[], &[(p2wpkh_script(1), 1_234)]);
    {
        let mut state = mempool.test_state_lock().write();
        state.txs.insert(
            tx,
            TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false),
        );
        state.publish_at("11".repeat(32).parse().unwrap(), &[txid]);
    }
    check(true);
    let (txids, hash) = mempool.txids_with_hash().unwrap();
    assert_eq!(txids, vec![txid]);
    assert_eq!(hash, mempool.txids_hash().unwrap());
    assert_eq!(txids, mempool.txids().unwrap());
}

#[test]
fn transaction_times_hash_includes_order_and_length() {
    assert_ne!(
        transaction_times_hash(&[1, 2]),
        transaction_times_hash(&[2, 1])
    );
    assert_ne!(
        transaction_times_hash(&[1]),
        transaction_times_hash(&[1, 0])
    );
    assert_eq!(
        transaction_times_hash(&[1, 2]),
        transaction_times_hash(&[1, 2])
    );
}
