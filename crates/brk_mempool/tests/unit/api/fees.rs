use brk_error::Error;
use brk_types::TxidPrefix;

use super::*;
use crate::{
    state::TxEntry,
    test_support::{self, fake_entry_info, fake_tx},
};

#[test]
fn projections_require_publication_and_complete_template_selection() {
    let mempool = Mempool::for_test();
    assert!(matches!(mempool.fees(), Err(Error::StateUpdating)));
    assert!(matches!(mempool.block_stats(), Err(Error::StateUpdating)));
    assert!(matches!(
        mempool.block_template(),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        mempool.next_block_hash(),
        Err(Error::StateUpdating)
    ));
    mempool.test_tick(&[], FeeRate::new(2.0));
    assert_eq!(mempool.fees().unwrap().minimum_fee, FeeRate::new(2.0));
    let blocks = mempool.block_stats().unwrap();
    assert!(!blocks.is_empty());
    assert!(blocks.iter().all(|block| block.tx_count == 0));
    assert!(mempool.block_template().unwrap().transactions.is_empty());
    mempool.test_tick(&[test_support::fake_txid(42)], FeeRate::new(2.0));
    assert!(matches!(mempool.fees(), Err(Error::StateUpdating)));
    assert!(matches!(mempool.block_stats(), Err(Error::StateUpdating)));
}

#[test]
fn live_rate_requires_a_matching_completed_projection() {
    let mempool = Mempool::for_test();
    let tx = fake_tx(1, &[], &[]);
    let txid = tx.txid;
    let entry = TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false);
    let fallback = entry.fee_rate();
    mempool.test_state_lock().write().txs.insert(tx, entry);
    let tip = BlockHash::default();
    assert!(mempool.effective_fee_rate(&txid, &tip).is_err());
    mempool.test_tick(&[txid], FeeRate::new(1.0));
    mempool.test_state_lock().write().publish_at(tip, &[txid]);
    assert_eq!(
        mempool.effective_fee_rate(&txid, &tip).unwrap(),
        Some(fallback)
    );
    assert!(
        mempool
            .effective_fee_rate(&txid, &"11".repeat(32).parse().unwrap())
            .is_err()
    );
    mempool
        .test_state_lock()
        .write()
        .txs
        .remove_by_prefix(&TxidPrefix::from(txid));
    assert!(mempool.snapshot().chunk_rate_for(&txid).is_some());
    mempool.test_state_lock().write().publish_at(tip, &[]);
    assert!(mempool.effective_fee_rate(&txid, &tip).is_err());
    mempool.test_tick(&[], FeeRate::new(1.0));
    mempool.test_state_lock().write().publish_at(tip, &[]);
    assert_eq!(mempool.effective_fee_rate(&txid, &tip).unwrap(), None);
}
