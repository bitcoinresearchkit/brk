use std::panic::{catch_unwind, panic_any};

use rustc_hash::FxHashMap;

use super::*;

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
    assert!(mempool.tick_with(|_| FxHashMap::default()).is_err());
    assert_eq!(mempool.0.state.read().published_tip, Some(tip));
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
