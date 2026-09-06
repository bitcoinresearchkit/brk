use super::*;

#[test]
fn dropping_the_http_owner_signals_remaining_work() {
    let owner = Cancellation::default();
    let signal = owner.signal();
    drop(signal.clone());
    assert!(!signal.load(Ordering::Relaxed));
    drop(owner);
    assert!(signal.load(Ordering::Relaxed));
}
