use crate::test_cache::init_cache;
use std::{
    mem::discriminant,
    thread,
    time::{Duration, Instant},
};

use bitview::{ComputePluginSet, ImportContext};
use bitview_default::DefaultPlugins;
use bitview_query::Query;
use brk_error::Error;
use brk_mempool::Mempool;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_types::{Addr, BlockHash, NextBlockHash, Txid};
use tempfile::tempdir;

#[test]
fn query_preflights_preserve_resolution_errors_and_safe_prefix_during_updates() {
    thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(assert_query_preflights_preserve_resolution_errors_and_safe_prefix_during_updates)
        .unwrap()
        .join()
        .unwrap();
}

fn assert_query_preflights_preserve_resolution_errors_and_safe_prefix_during_updates() {
    init_cache();
    let directory = tempdir().unwrap();
    let client = Client::new("http://127.0.0.1:1", Auth::None).unwrap();
    let reader = Reader::new_without_rlimit(directory.path().join("blocks"), &client);
    let plugins = DefaultPlugins::import(ImportContext::new(directory.path()), &reader).unwrap();

    let gate = plugins.publication().clone();
    let query = Query::build(&plugins, None);

    assert!(matches!(
        query.local_sync_status(),
        Err(Error::StateUpdating)
    ));
    // Immutable preflights use the retained safe prefix without waiting for
    // append-only publication. This empty fixture still has no published tip.
    gate.begin_update();
    assert!(matches!(
        query.local_sync_status(),
        Err(Error::StateUpdating)
    ));
    gate.finish_update();

    let unknown_block = BlockHash::default();
    assert!(matches!(
        query.resolve_block_snapshot(&unknown_block),
        Err(Error::NotFound(_))
    ));

    let unknown_txid = Txid::COINBASE;
    assert!(matches!(
        query.resolve_confirmed_tx(&unknown_txid),
        Err(Error::UnknownTxid)
    ));
    assert!(matches!(
        query.transaction_status(&unknown_txid),
        Err(Error::UnknownTxid)
    ));
    assert!(matches!(
        query.resolve_raw_transaction(&unknown_txid),
        Err(Error::UnknownTxid)
    ));
    assert!(matches!(
        query.resolve_transaction(&unknown_txid),
        Err(Error::UnknownTxid)
    ));
    assert!(matches!(
        query.resolve_cpfp(&unknown_txid),
        Err(Error::UnknownTxid)
    ));

    // An absent mempool leaves chain-only lookup errors intact. An attached but
    // never observed mempool cannot establish absence (or an empty RBF tree).
    let query = Query::build(&plugins, Some(Mempool::new(&client).read_only_clone()));
    for result in [
        query.transaction_status(&unknown_txid).map(|_| ()),
        query.resolve_raw_transaction(&unknown_txid).map(|_| ()),
        query.resolve_transaction(&unknown_txid).map(|_| ()),
        query.resolve_cpfp(&unknown_txid).map(|_| ()),
        query.resolve_rbf(&unknown_txid).map(|_| ()),
    ] {
        assert!(matches!(result, Err(Error::StateUpdating)), "{result:?}");
    }

    assert!(matches!(
        query.resolve_block_template_diff(NextBlockHash::new(0xDEAD_BEEF)),
        Err(Error::NotFound(_))
    ));

    for raw in ["17jGLFhcnPYqG17qN2ouxbScrcnroHqRP", "not-an-address"] {
        let addr = Addr::from(raw.to_owned());
        let expected = query.addr(addr.clone()).unwrap_err();
        assert_eq!(
            discriminant(&query.resolve_addr_chain_txs(&addr, None, 25).unwrap_err()),
            discriminant(&expected)
        );
        let actual = match query.resolve_addr_utxos(&addr, 1000) {
            Ok(_) => panic!("unknown or invalid address should be rejected"),
            Err(error) => error,
        };
        assert_eq!(discriminant(&actual), discriminant(&expected));
    }

    let unknown_addr = Addr::from("17jGLFhcnPYqG17qN2ouxbScrcnroHqRP".to_owned());
    assert!(
        matches!(
            query.addr_mempool_txs(&unknown_addr, 50),
            Err(Error::StateUpdating)
        ),
        "an unobserved mempool is not an observed empty address"
    );

    let invalid_addr = Addr::from("not-an-address".to_owned());
    assert!(matches!(
        query.addr_mempool_txs(&invalid_addr, 50),
        Err(Error::InvalidAddr)
    ));
    assert!(matches!(
        query.resolve_addr_txs(&unknown_addr, 50, 25, 50),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        query.resolve_addr_txs(&invalid_addr, 50, 25, 50),
        Err(Error::InvalidAddr)
    ));

    gate.begin_update();
    let timed = query.with_deadline(Instant::now() + Duration::from_millis(20));
    assert!(matches!(
        timed.resolve_addr_utxos(&unknown_addr, 1000),
        Err(Error::ReadTimeout)
    ));
    assert!(matches!(
        query.resolve_addr_utxos(&invalid_addr, 1000),
        Err(Error::InvalidAddr)
    ));
    gate.finish_update();

    gate.begin_update();
    let addr = Addr::from("17jGLFhcnPYqG17qN2ouxbScrcnroHqRP".to_owned());
    let timed = query.with_deadline(Instant::now() + Duration::from_millis(20));
    assert!(matches!(timed.addr(addr.clone()), Err(Error::ReadTimeout)));
    assert!(matches!(
        query.addr(Addr::from("not-an-address".to_owned())),
        Err(Error::InvalidAddr)
    ));

    gate.finish_update();
    assert!(matches!(query.addr(addr), Err(Error::UnknownAddr)));
}

#[allow(dead_code)]
#[path = "common/cache.rs"]
mod test_cache;
