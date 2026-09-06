use brk_types::{AddrBytes, Sats, Timestamp, TxOut};

use super::*;
use crate::{
    cycle::AddrTransitions,
    state::TxEntry,
    test_support::{fake_entry_info, fake_tx, p2wpkh_script},
};

#[test]
fn completed_selection_shares_bodies_and_rejects_wrong_chain() {
    let mempool = Mempool::for_test();
    let script = p2wpkh_script(1);
    let addr = AddrBytes::try_from(&script).unwrap();
    let tip = BlockHash::default();
    assert!(mempool.addr_txs(&addr, 50, &tip).is_err());
    assert!(mempool.addr_stats(&addr, &tip).is_err());

    let mut transitions = AddrTransitions::default();
    let mut state = mempool.test_state_lock().write();
    for seed in 1..=2 {
        let prevout = TxOut::from((script.clone(), Sats::from(2_000u64)));
        let tx = fake_tx(seed, &[Some(prevout)], &[]);
        let entry = TxEntry::new(&fake_entry_info(tx.txid, 100, 100), 100, false);
        state.addrs.add_tx(&mut transitions, &tx);
        state.txs.insert(tx, entry);
    }
    let live = state.txs.txids().copied().collect::<Vec<_>>();
    state.publish_at(tip, &live);
    drop(state);

    let transactions = mempool.addr_txs(&addr, 1, &tip).unwrap();
    assert_eq!(transactions.len(), 1);
    let state = mempool.read();
    let stored = &state.txs.record(&transactions[0].txid).unwrap().tx;
    assert!(Arc::ptr_eq(stored, &transactions[0]));
    drop(state);
    let wrong_tip = "11".repeat(32).parse().unwrap();
    assert!(mempool.addr_txs(&addr, 1, &wrong_tip).is_err());
    assert!(mempool.addr_stats(&addr, &wrong_tip).is_err());
}

#[test]
fn bounded_pages_match_full_order_including_timestamp_ties() {
    let mempool = Mempool::for_test();
    let script = p2wpkh_script(1);
    let addr = AddrBytes::try_from(&script).unwrap();
    let mut transitions = AddrTransitions::default();
    let mut state = mempool.test_state_lock().write();
    let mut expected = Vec::new();
    for seed in 1..=128 {
        let prevout = TxOut::from((script.clone(), Sats::from(2_000u64)));
        let tx = fake_tx(seed, &[Some(prevout)], &[]);
        let mut entry = TxEntry::new(&fake_entry_info(tx.txid, 100, 100), 100, false);
        entry.first_seen = Timestamp::from(100u32 + u32::from(seed % 7));
        expected.push((entry.first_seen, TxidPrefix::from(tx.txid), tx.txid));
        state.addrs.add_tx(&mut transitions, &tx);
        state.txs.insert(tx, entry);
    }
    let tip = BlockHash::default();
    let live = state.txs.txids().copied().collect::<Vec<_>>();
    state.publish_at(tip, &live);
    drop(state);
    expected.sort_unstable_by_key(|(time, prefix, _)| Reverse((*time, *prefix)));
    for limit in [0, 1, 5, 25, 127, 128, usize::MAX] {
        let page = mempool.addr_txs(&addr, limit, &tip).unwrap();
        assert_eq!(
            page.iter().map(|tx| tx.txid).collect::<Vec<_>>(),
            expected
                .iter()
                .take(limit)
                .map(|(_, _, txid)| *txid)
                .collect::<Vec<_>>()
        );
    }
}
