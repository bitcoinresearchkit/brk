use crate::test_cache::init_cache;
use std::{sync::mpsc, thread, time::Duration};

use bitview::{ComputePluginSet, ImportContext};
use bitview_default::DefaultPlugins;
use bitview_query::Query;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_types::Index;
use tempfile::tempdir;

#[test]
fn series_reads_wait_for_source_and_bound_publications() {
    thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(check_publication_gates)
        .unwrap()
        .join()
        .unwrap();
}

fn check_publication_gates() {
    init_cache();
    let directory = tempdir().unwrap();
    let client = Client::new("http://127.0.0.1:1", Auth::None).unwrap();
    let reader = Reader::new_without_rlimit(directory.path().join("blocks"), &client);
    let plugins = DefaultPlugins::import(ImportContext::new(directory.path()), &reader).unwrap();
    let query = Query::build(&plugins, None);

    for (name, index) in [
        ("txin_index", Index::TxOutIndex),
        ("txin_index", Index::TxInIndex),
        (
            "utxos_4m_to_5m_old_transfer_volume_average_1y_cents",
            Index::Day1,
        ),
        ("addr_state", Index::P2AAddrIndex),
    ] {
        let name = name.into();
        let expected = query.len(&name, index).unwrap();
        let gate = plugins.publication().clone();
        gate.begin_update();
        let query = query.clone();
        let (started_tx, started_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let reader = thread::spawn(move || {
            started_tx.send(()).unwrap();
            result_tx.send(query.len(&name, index)).unwrap();
        });
        started_rx.recv().unwrap();
        let waiting = result_rx.recv_timeout(Duration::from_millis(30));
        // Reopen even on assertion failure so the reader cannot be stranded.
        gate.finish_update();
        assert!(matches!(waiting, Err(mpsc::RecvTimeoutError::Timeout)));
        assert_eq!(
            result_rx
                .recv_timeout(Duration::from_secs(1))
                .unwrap()
                .unwrap(),
            expected
        );
        reader.join().unwrap();
    }
}

#[allow(dead_code)]
#[path = "common/cache.rs"]
mod test_cache;
