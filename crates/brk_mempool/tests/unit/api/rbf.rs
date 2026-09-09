use bitcoin::{Txid as BitcoinTxid, hashes::Hash};
use brk_types::{FeeRate, TxOut, TxidPrefix};

use super::*;
use crate::{
    Mempool, TxRemoval,
    state::TxEntry,
    test_support::{fake_entry_info, fake_tx, fake_txid, p2wpkh_script},
};

/// Place a live tx (the replacer) and bury one or more predecessors
/// pointing at it. `bury_chain` carries `(seed, predecessor_of_next)`
/// pairs in oldest-first order. Each links forward to the next entry
/// or to `live_seed` when last.
fn build_rbf_world(live_seed: u8, predecessors: &[u8]) -> (Mempool, Txid, Vec<Txid>) {
    let mempool = Mempool::for_test();
    let live_tx = fake_tx(
        live_seed,
        &[Some(TxOut::from((p2wpkh_script(99), Sats::from(6_234u64))))],
        &[(p2wpkh_script(live_seed + 1), 1_234)],
    );
    let live_txid = live_tx.txid;
    let live_entry = TxEntry::new(&fake_entry_info(live_txid, 5_000, 100), 100, true);

    let mut pred_txids = Vec::with_capacity(predecessors.len());
    let mut state = mempool.test_state_lock().write();
    for (i, seed) in predecessors.iter().enumerate() {
        let tx = fake_tx(*seed, &[None], &[(p2wpkh_script(seed + 1), 1_234)]);
        let txid = tx.txid;
        // Each predecessor signals BIP-125 (rbf=true) so full_rbf stays clear.
        let entry = TxEntry::new(&fake_entry_info(txid, 1_000, 100), 100, true);
        let by = predecessors
            .get(i + 1)
            .map(|next_seed| fake_txid(*next_seed))
            .unwrap_or(live_txid);
        let rate = FeeRate::from((entry.fee, entry.vsize));
        state
            .graveyard
            .bury(tx, entry, rate, TxRemoval::Replaced { by });
        pred_txids.push(txid);
    }
    state.txs.insert(live_tx, live_entry);
    drop(state);
    mempool.test_tick(&[live_txid], FeeRate::new(1.0));
    mempool
        .test_state_lock()
        .write()
        .publish_at(BlockHash::default(), &[live_txid]);
    assert!(
        mempool
            .read()
            .ensure_published_at(&BlockHash::default())
            .is_ok()
    );
    (mempool, live_txid, pred_txids)
}

#[test]
fn rbf_requires_matching_publication_and_graph_revision() {
    let tip = BlockHash::default();
    let empty = Mempool::for_test();
    assert!(matches!(
        empty.rbf_for_tx(&Txid::COINBASE, &tip),
        Err(Error::StateUpdating)
    ));
    for limit in [0, 25] {
        assert!(matches!(
            empty.recent_rbf_trees(false, limit, &tip),
            Err(Error::StateUpdating)
        ));
    }

    let (mempool, live, predecessors) = build_rbf_world(200, &[1]);
    let root = mempool.rbf_for_tx(&live, &tip).unwrap().root.unwrap();
    assert_eq!(root.rate, mempool.snapshot().chunk_rate_for(&live).unwrap());
    let wrong_tip = "11".repeat(32).parse().unwrap();
    assert!(matches!(
        mempool.rbf_for_tx(&live, &wrong_tip),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        mempool.recent_rbf_trees(false, 25, &wrong_tip),
        Err(Error::StateUpdating)
    ));

    // A completed live revision must not borrow rates from the older graph.
    let mut state = mempool.test_state_lock().write();
    state.txs.remove_by_prefix(&TxidPrefix::from(live)).unwrap();
    state.publish_at(tip, &[]);
    drop(state);
    assert!(matches!(
        mempool.rbf_for_tx(&predecessors[0], &tip),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        mempool.recent_rbf_trees(false, 25, &tip),
        Err(Error::StateUpdating)
    ));
    mempool.test_tick(&[], FeeRate::new(1.0));
    mempool.test_state_lock().write().publish_at(tip, &[]);
    assert!(
        mempool
            .rbf_for_tx(&predecessors[0], &tip)
            .unwrap()
            .is_empty()
    );
    assert!(
        mempool
            .recent_rbf_trees(false, 25, &tip)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn rbf_for_tx_single_replacement_returns_root_and_replaces() {
    // pred -> live. rbf_for_tx(pred) walks forward to live and lists
    // pred under its `replaces` tree.
    let (mempool, live, preds) = build_rbf_world(0xC0, &[0xC1]);
    let pred = preds[0];

    let rbf = mempool.rbf_for_tx(&pred, &BlockHash::default()).unwrap();
    let root = rbf.root.expect("terminal replacer reachable");
    assert_eq!(root.txid, live);
    assert!(root.in_mempool);
    assert!(!root.replaces[0].in_mempool);
    let replaced_txids: Vec<Txid> = root.replaces.iter().map(|n| n.txid).collect();
    assert_eq!(replaced_txids, vec![pred]);
    // Convenience list: direct predecessors of the requested tx.
    assert!(
        rbf.replaces.is_empty(),
        "pred has no predecessors of its own"
    );
}

#[test]
fn rbf_for_tx_chain_walks_to_terminal_root() {
    // A -> B -> C(live). rbf_for_tx(A) walks A -> B -> C, root is C.
    // root.replaces is B, B.replaces is A.
    let (mempool, live, preds) = build_rbf_world(0xC2, &[0xC3, 0xC4]);
    let a = preds[0];
    let b = preds[1];

    let rbf = mempool.rbf_for_tx(&a, &BlockHash::default()).unwrap();
    let root = rbf.root.expect("terminal replacer reachable");
    assert_eq!(root.txid, live);
    assert_eq!(root.replaces.len(), 1);
    assert_eq!(root.replaces[0].txid, b);
    assert_eq!(root.replaces[0].replaces.len(), 1);
    assert_eq!(root.replaces[0].replaces[0].txid, a);
}

#[test]
fn rbf_for_tx_unknown_tx_returns_none_root() {
    let mempool = Mempool::for_test();
    mempool.test_tick(&[], FeeRate::new(1.0));
    mempool
        .test_state_lock()
        .write()
        .publish_at(BlockHash::default(), &[]);
    let bogus = Txid::COINBASE;
    let rbf = mempool.rbf_for_tx(&bogus, &BlockHash::default()).unwrap();
    assert!(rbf.root.is_none());
    assert!(rbf.replaces.is_empty());
}

#[test]
fn rbf_for_tx_rejects_live_prefix_collision() {
    let (mempool, live, _) = build_rbf_world(0xCA, &[]);
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(live.as_slice());
    bytes[8] ^= 1;
    let collision = Txid::from(BitcoinTxid::from_byte_array(bytes));
    assert_eq!(TxidPrefix::from(&live), TxidPrefix::from(&collision));

    assert!(
        mempool
            .rbf_for_tx(&collision, &BlockHash::default())
            .unwrap()
            .is_empty()
    );
}

#[test]
fn recent_rbf_trees_dedup_by_root_and_respect_limit() {
    // Chain 0xC6 -> 0xC7 -> live plus a sibling 0xC8 also replaced by
    // live. All paths roll up to the same root, so the recent listing
    // dedups them down to a single tree.
    let (mempool, live, _preds) = build_rbf_world(0xC5, &[0xC6, 0xC7]);
    {
        let mut state = mempool.test_state_lock().write();
        let extra = fake_tx(0xC8, &[None], &[(p2wpkh_script(0xC9), 1_234)]);
        let extra_txid = extra.txid;
        let entry = TxEntry::new(&fake_entry_info(extra_txid, 999, 100), 100, true);
        let rate = FeeRate::from((entry.fee, entry.vsize));
        state
            .graveyard
            .bury(extra, entry, rate, TxRemoval::Replaced { by: live });
    }
    let trees = mempool
        .recent_rbf_trees(false, 10, &BlockHash::default())
        .unwrap();
    assert_eq!(trees.len(), 1, "all paths roll up to one root");
    assert_eq!(trees[0].txid, live);

    let capped = mempool
        .recent_rbf_trees(false, 0, &BlockHash::default())
        .unwrap();
    assert!(capped.is_empty(), "limit honored");
}

#[test]
fn deep_and_cyclic_histories_fail_without_partial_trees() {
    let predecessors: Vec<u8> = (1..=MAX_RBF_DEPTH as u8).collect();
    let (deep, live, _) = build_rbf_world(200, &predecessors);
    assert!(matches!(
        deep.rbf_for_tx(&live, &BlockHash::default()),
        Err(Error::Internal(_))
    ));
    assert!(matches!(
        deep.recent_rbf_trees(false, 25, &BlockHash::default()),
        Err(Error::Internal(_))
    ));

    let (cycle, live, predecessors) = build_rbf_world(200, &[1]);
    let mut state = cycle.test_state_lock().write();
    let record = state.txs.remove_by_prefix(&TxidPrefix::from(live)).unwrap();
    let rate = record.entry.fee_rate();
    state.graveyard.bury(
        record.tx,
        record.entry,
        rate,
        TxRemoval::Replaced {
            by: predecessors[0],
        },
    );
    drop(state);
    cycle.test_tick(&[], FeeRate::new(1.0));
    cycle
        .test_state_lock()
        .write()
        .publish_at(BlockHash::default(), &[]);
    assert!(matches!(
        cycle.rbf_for_tx(&live, &BlockHash::default()),
        Err(Error::Internal(_))
    ));
    assert!(matches!(
        cycle.recent_rbf_trees(false, 25, &BlockHash::default()),
        Err(Error::Internal(_))
    ));
}

#[test]
fn tree_width_and_stale_scan_entries_consume_work_budget() {
    let (mempool, live, _) = build_rbf_world(200, &[]);
    let mut state = mempool.test_state_lock().write();
    for seed in 1..=8 {
        let tx = fake_tx(seed, &[], &[]);
        let entry = TxEntry::new(&fake_entry_info(tx.txid, 100, 100), 100, true);
        let rate = entry.fee_rate();
        state
            .graveyard
            .bury(tx, entry, rate, TxRemoval::Replaced { by: live });
    }
    assert!(Mempool::build_rbf_node(&live, &state.txs, &state.graveyard, &mut 8, 0).is_err());
    assert!(
        Mempool::build_rbf_node(&live, &state.txs, &state.graveyard, &mut 9, 0)
            .unwrap()
            .is_some()
    );
    // No matching replacement trees: even stale order entries must be charged.
    for _ in 0..=MAX_RBF_WORK {
        let tx = fake_tx(50, &[], &[]);
        let txid = tx.txid;
        let entry = TxEntry::new(&fake_entry_info(txid, 100, 100), 100, true);
        let rate = entry.fee_rate();
        state.graveyard.bury(tx, entry, rate, TxRemoval::Vanished);
        state.graveyard.exhume(&txid);
    }
    drop(state);
    assert!(matches!(
        mempool.recent_rbf_trees(false, 25, &BlockHash::default()),
        Err(Error::Internal(_))
    ));
}
