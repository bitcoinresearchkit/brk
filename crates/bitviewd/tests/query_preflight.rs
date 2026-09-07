use std::{mem::discriminant, sync::mpsc, thread, time::Duration};

use bitview::{ComputePluginSet, ImportContext};
use bitview_default::DefaultPlugins;
use bitview_query::Query;
use brk_error::Error;
use brk_mempool::Mempool;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_types::{Addr, BlockHash, Day1, NextBlockHash, Txid};

#[test]
fn query_preflights_preserve_resolution_errors_and_defer_during_updates() {
    thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(assert_query_preflights_preserve_resolution_errors_and_defer_during_updates)
        .unwrap()
        .join()
        .unwrap();
}

fn assert_query_preflights_preserve_resolution_errors_and_defer_during_updates() {
    let directory = tempfile::tempdir().unwrap();
    let client = Client::new("http://127.0.0.1:1", Auth::None).unwrap();
    let reader = Reader::new_without_rlimit(directory.path().join("blocks"), &client);
    let plugins = DefaultPlugins::import(ImportContext::new(directory.path()), &reader).unwrap();

    let gate = plugins.publication().clone();
    let query = Query::build(&plugins, None);

    assert!(matches!(
        query.local_sync_status(),
        Err(Error::StateUpdating)
    ));
    gate.begin_update();
    thread::scope(|scope| {
        let (started_tx, started_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let query = &query;
        scope.spawn(move || {
            started_tx.send(()).unwrap();
            result_tx.send(query.local_sync_status()).unwrap();
        });
        started_rx.recv().unwrap();
        assert!(result_rx.recv_timeout(Duration::from_millis(10)).is_err());
        gate.finish_update();
        assert!(matches!(
            result_rx.recv_timeout(Duration::from_secs(1)).unwrap(),
            Err(Error::StateUpdating)
        ));
    });

    gate.begin_update();
    thread::scope(|scope| {
        let (started_tx, started_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let query = &query;
        scope.spawn(move || {
            started_tx.send(()).unwrap();
            result_tx
                .send(query.day_is_deeply_confirmed(Day1::default()))
                .unwrap();
        });

        started_rx.recv().unwrap();
        assert!(result_rx.recv_timeout(Duration::from_millis(10)).is_err());
        gate.finish_update();
        assert!(
            !result_rx
                .recv_timeout(Duration::from_secs(1))
                .unwrap()
                .unwrap()
        );
    });

    let unknown_block = BlockHash::default();
    assert!(matches!(
        query.resolve_block_snapshot(&unknown_block),
        Err(Error::NotFound(_))
    ));
    assert!(matches!(
        query.height_by_hash(&unknown_block),
        Err(Error::NotFound(_))
    ));

    let unknown_txid = Txid::COINBASE;
    assert!(matches!(
        query.resolve_confirmed_tx(&unknown_txid),
        Err(Error::UnknownTxid)
    ));
    assert!(matches!(
        query.resolve_tx(&unknown_txid),
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
    let query = Query::build(&plugins, Some(Mempool::new(&client)));
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
        let actual = query.addr_stats_preflight(&addr).unwrap_err();
        assert_eq!(discriminant(&actual), discriminant(&expected));
        assert_eq!(
            discriminant(&query.resolve_addr_chain_txs(&addr, None, 25).unwrap_err()),
            discriminant(&expected)
        );
        let actual = match query.addr_utxos_preflight(&addr, 1000) {
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
    assert!(matches!(
        query.addr_utxos_preflight(&unknown_addr, 1000),
        Ok(None)
    ));
    assert!(matches!(
        query.addr_utxos_preflight(&invalid_addr, 1000),
        Err(Error::InvalidAddr)
    ));
    gate.finish_update();

    gate.begin_update();
    let addr = Addr::from("17jGLFhcnPYqG17qN2ouxbScrcnroHqRP".to_owned());
    assert!(matches!(query.addr_stats_preflight(&addr), Ok(None)));
    assert!(matches!(
        query.addr_stats_preflight(&Addr::from("not-an-address".to_owned())),
        Err(brk_error::Error::InvalidAddr)
    ));
    assert!(matches!(
        query.addr(Addr::from("not-an-address".to_owned())),
        Err(brk_error::Error::InvalidAddr)
    ));

    gate.finish_update();
    assert!(matches!(
        query.addr_stats_preflight(&addr),
        Err(brk_error::Error::UnknownAddr)
    ));
}
