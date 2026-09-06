use super::*;

#[test]
fn histogram_reads_require_a_completed_matching_publication() {
    let mempool = Mempool::for_test();
    let tip = BlockHash::default();
    let next = "11".repeat(32).parse().unwrap();
    let check = |tip: &BlockHash, ready| {
        assert_eq!(mempool.live_raw_histogram(tip).is_ok(), ready);
        assert_eq!(mempool.live_eligible_histogram(tip).is_ok(), ready);
    };
    check(&tip, false);
    mempool.test_state_lock().write().publish_at(tip, &[]);
    check(&tip, true);
    check(&next, false);
    mempool.test_state_lock().write().published_tip = None;
    check(&tip, false);
    mempool.test_state_lock().write().publish_at(next, &[]);
    check(&tip, false);
    check(&next, true);
}

#[test]
fn published_histograms_capture_owned_output_counts() {
    use crate::{
        state::TxEntry,
        test_support::{fake_entry_info, fake_tx, p2wpkh_script},
    };
    let mempool = Mempool::for_test();
    let tip = BlockHash::default();
    let tx = fake_tx(
        1,
        &[],
        &[(p2wpkh_script(1), 12_345), (p2wpkh_script(2), 10_000)],
    );
    let txid = tx.txid;
    let entry = TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false);
    {
        let mut state = mempool.test_state_lock().write();
        state.txs.insert(tx, entry);
        state.publish_at(tip, &[txid]);
    }
    let raw = mempool.live_raw_histogram(&tip).unwrap();
    let eligible = mempool.live_eligible_histogram(&tip).unwrap();
    assert_eq!(raw.iter().sum::<u32>(), 2);
    assert_eq!(eligible.iter().sum::<u32>(), 1);
    {
        let mut state = mempool.test_state_lock().write();
        state.published_tip = None;
        state.txs.remove_by_prefix(&txid.into()).unwrap();
        state.publish_at(tip, &[]);
    }
    assert_eq!(
        mempool
            .live_raw_histogram(&tip)
            .unwrap()
            .iter()
            .sum::<u32>(),
        0
    );
    assert_eq!(raw.iter().sum::<u32>(), 2);
    assert_eq!(eligible.iter().sum::<u32>(), 1);
}
