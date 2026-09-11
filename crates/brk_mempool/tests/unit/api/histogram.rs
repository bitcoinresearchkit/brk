use super::*;
use crate::Mempool;
use crate::{
    state::TxEntry,
    test_support::{fake_entry_info, fake_tx, p2wpkh_script},
};

#[test]
fn histogram_reads_require_a_completed_matching_publication() {
    let mut mempool = Mempool::for_test();
    let tip = BlockHash::default();
    let next = "11".repeat(32).parse().unwrap();
    let reader = mempool.read_only_clone();
    let check = |tip: &BlockHash, ready| {
        assert_eq!(reader.load().live_raw_histogram(tip).is_ok(), ready);
        assert_eq!(reader.load().live_eligible_histogram(tip).is_ok(), ready);
    };
    check(&tip, false);
    mempool.test_publish(tip);
    check(&tip, true);
    check(&next, false);
    check(&tip, true);
    mempool.test_publish(next);
    check(&tip, false);
    check(&next, true);
}

#[test]
fn published_histograms_capture_owned_output_counts() {
    let mut mempool = Mempool::for_test();
    let tip = BlockHash::default();
    let tx = fake_tx(
        1,
        &[],
        &[(p2wpkh_script(1), 12_345), (p2wpkh_script(2), 10_000)],
    );
    let txid = tx.txid;
    let entry = TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false);
    {
        let state = mempool.test_state_mut();
        state.txs.insert(tx, entry);
    }
    mempool.test_publish(tip);
    let raw = mempool.published().live_raw_histogram(&tip).unwrap();
    let eligible = mempool.published().live_eligible_histogram(&tip).unwrap();
    assert_eq!(raw.iter().sum::<u32>(), 2);
    assert_eq!(eligible.iter().sum::<u32>(), 1);
    {
        let state = mempool.test_state_mut();
        state.txs.remove_by_prefix(&txid.into()).unwrap();
    }
    mempool.test_publish(tip);
    assert_eq!(
        mempool
            .published()
            .live_raw_histogram(&tip)
            .unwrap()
            .iter()
            .sum::<u32>(),
        0
    );
    assert_eq!(raw.iter().sum::<u32>(), 2);
    assert_eq!(eligible.iter().sum::<u32>(), 1);
}
