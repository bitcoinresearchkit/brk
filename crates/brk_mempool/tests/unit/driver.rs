use std::panic::{catch_unwind, panic_any};

use rustc_hash::FxHashMap;

use super::*;

#[test]
fn statistics_retain_complete_membership_independently_of_live_state() {
    use crate::{
        state::TxEntry,
        test_support::{fake_entry_info, fake_tx},
    };
    let mempool = Mempool::for_test();
    let tip = Default::default();
    assert!(matches!(mempool.info(), Err(Error::StateUpdating)));
    mempool.publish_observation(tip, &[]);
    assert_eq!(mempool.info().unwrap().count, 0);

    let tx = fake_tx(1, &[None], &[]);
    let txid = tx.txid;
    {
        let mut state = mempool.0.state.write();
        state.published_tip = None;
        state.info.add(&tx, 100_u64.into());
        state.txs.insert(
            tx,
            TxEntry::new(&fake_entry_info(txid, 100, 100), 100, false),
        );
        // Holding the live write lock must not block statistics readers.
        let reader = mempool.clone();
        let (send, receive) = std::sync::mpsc::channel();
        let reading = std::thread::spawn(move || send.send(reader.info().unwrap().count).unwrap());
        assert_eq!(
            receive
                .recv_timeout(std::time::Duration::from_secs(1))
                .unwrap(),
            0
        );
        reading.join().unwrap();
    }
    mempool.publish_observation(tip, &[]);
    assert_eq!(
        mempool.info().unwrap().count,
        0,
        "incomplete membership retains the previous snapshot"
    );
    mempool.publish_observation(tip, &[txid]);
    assert!(
        mempool.0.state.read().published_tip.is_none(),
        "unresolved inputs still hide address data"
    );
    let first = mempool.info().unwrap();
    assert_eq!(first.count, 1);
    assert_eq!(u64::from(first.total_fee), 100);
    {
        let mut state = mempool.0.state.write();
        let record = state.txs.remove_by_prefix(&txid.into()).unwrap();
        state.info.remove(&record.tx, record.entry.fee);
    }
    assert_eq!(mempool.info().unwrap().count, 1);
    mempool.publish_observation(tip, &[]);
    assert_eq!(mempool.info().unwrap().count, 0);
    assert_eq!(first.count, 1, "captured statistics are owned");
}

#[test]
fn concurrent_cycles_are_rejected_before_rpc_or_mutation() {
    let mempool = Mempool::for_test();
    let _cycle = mempool.0.cycle.lock();
    assert!(matches!(
        mempool.tick_with(|_| FxHashMap::default()),
        Err(brk_error::Error::StateUpdating)
    ));
}

#[test]
fn fetch_failure_preserves_the_previous_publication() {
    use brk_rpc::{Auth, Client};
    use std::{net::TcpListener, sync::Arc, time::Duration};

    // An unused local port fails immediately with retries disabled.
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let mut mempool = Mempool::for_test();
    Arc::get_mut(&mut mempool.0).unwrap().client =
        Client::new_with(&format!("http://{address}"), Auth::None, 0, Duration::ZERO).unwrap();
    let tip = Default::default();
    mempool.0.state.write().published_tip = Some(tip);
    mempool.publish_observation(tip, &[]);
    assert!(mempool.tick_with(|_| FxHashMap::default()).is_err());
    assert_eq!(mempool.0.state.read().published_tip, Some(tip));
    assert_eq!(mempool.info().unwrap().count, 0);
}

#[test]
#[should_panic(expected = "Mempool::start_with already running on this instance")]
fn double_start_panics_with_documented_message() {
    let mempool = Mempool::for_test();
    // Simulate a prior `start_with` having grabbed the latch. We
    // can't actually call it first because the real call enters an
    // infinite loop. Flipping the atomic is what the runtime check
    // observes anyway.
    mempool.0.started.store(true, Ordering::Release);
    mempool.start_with(|_: &[(Txid, Vout)]| FxHashMap::default());
}

#[test]
fn panic_msg_extracts_static_str_payload() {
    let payload = catch_unwind(|| panic!("boom static")).unwrap_err();
    assert_eq!(Mempool::panic_msg(payload.as_ref()), "boom static");
}

#[test]
fn panic_msg_extracts_string_payload() {
    let payload = catch_unwind(|| panic!("boom owned {}", 42)).unwrap_err();
    assert_eq!(Mempool::panic_msg(payload.as_ref()), "boom owned 42");
}

#[test]
fn panic_msg_falls_back_for_non_string_payload() {
    // Payload that isn't &str or String: the helper labels it
    // explicitly instead of dropping it on the floor.
    let payload = catch_unwind(|| panic_any(42u32)).unwrap_err();
    assert_eq!(
        Mempool::panic_msg(payload.as_ref()),
        "<non-string panic payload>"
    );
}
