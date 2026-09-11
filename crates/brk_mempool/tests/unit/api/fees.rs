use crate::Mempool;
use brk_error::Error;
use brk_types::{BlockHash, FeeRate, TxidPrefix};

use crate::{
    state::TxEntry,
    test_support::{self, fake_entry_info, fake_tx},
};

#[test]
fn projections_require_publication_and_complete_template_selection() {
    let mut mempool = Mempool::for_test();
    assert!(matches!(
        mempool.published().fees(),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        mempool.published().block_stats(),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        mempool.published().block_template_source().build(),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        mempool.published().next_block_hash(),
        Err(Error::StateUpdating)
    ));
    mempool.test_tick(&[], FeeRate::new(2.0));
    assert_eq!(
        mempool.published().fees().unwrap().minimum_fee,
        FeeRate::new(2.0)
    );
    let blocks = mempool.published().block_stats().unwrap();
    assert!(!blocks.is_empty());
    assert!(blocks.iter().all(|block| block.tx_count == 0));
    assert!(
        mempool
            .published()
            .block_template_source()
            .build()
            .unwrap()
            .transactions
            .is_empty()
    );
    let previous = mempool.published();
    mempool.test_tick(&[test_support::fake_txid(42)], FeeRate::new(2.0));
    assert_eq!(
        mempool.published().fees().unwrap().minimum_fee,
        previous.fees().unwrap().minimum_fee
    );
    assert_eq!(
        mempool.published().next_block_hash().unwrap(),
        previous.next_block_hash().unwrap()
    );
}

#[test]
fn live_rate_requires_a_matching_completed_projection() {
    let mut mempool = Mempool::for_test();
    let tx = fake_tx(1, &[], &[]);
    let txid = tx.txid;
    let entry = TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false);
    let fallback = entry.fee_rate();
    mempool.test_state_mut().txs.insert(tx, entry);
    let tip = BlockHash::default();
    assert!(mempool.published().cpfp_info(&txid, &tip).is_err());
    mempool.test_tick(&[txid], FeeRate::new(1.0));
    assert_eq!(
        mempool
            .published()
            .cpfp_info(&txid, &tip)
            .unwrap()
            .map(|info| info.effective_fee_per_vsize),
        Some(fallback)
    );
    assert!(
        mempool
            .published()
            .cpfp_info(&txid, &"11".repeat(32).parse().unwrap())
            .is_err()
    );
    mempool
        .test_state_mut()
        .txs
        .remove_by_prefix(&TxidPrefix::from(txid));
    assert!(
        mempool
            .published()
            .snapshot()
            .chunk_rate_for(&txid)
            .is_some()
    );
    assert!(
        mempool
            .published()
            .cpfp_info(&txid, &tip)
            .unwrap()
            .is_some()
    );
    mempool.test_tick(&[], FeeRate::new(1.0));
    assert!(
        mempool
            .published()
            .cpfp_info(&txid, &tip)
            .unwrap()
            .is_none()
    );
}
