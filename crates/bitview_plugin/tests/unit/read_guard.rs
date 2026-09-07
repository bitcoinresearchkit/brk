use std::{sync::mpsc, thread, time::Duration};

use bitview_traversable::Traversable;
use brk_types::Version;

use super::*;
use crate::{PluginGate, PluginId, PluginStorage};

#[derive(bitview_traversable::Traversable)]
struct TestPlugin {
    #[traversable(skip)]
    gate: PluginGate,
}

impl TestPlugin {
    fn new() -> Self {
        Self {
            gate: PluginGate::new(),
        }
    }
}

impl Plugin for TestPlugin {
    fn storage(&self) -> PluginStorage {
        PluginStorage::new(PluginId::new("test"), Version::ONE)
    }

    fn gate(&self) -> &PluginGate {
        &self.gate
    }
}

#[test]
fn multi_read_deduplicates_and_releases_every_gate() {
    let first = TestPlugin::new();
    let second = TestPlugin::new();
    let read = PluginReadGuard::acquire_for(vec![&second, &first, &second, &first], Duration::ZERO)
        .unwrap();
    assert!(matches!(&read._guards, Guards::Multiple { _guards } if _guards.len() == 2));
    drop(read);
    first.gate.begin_update();
    second.gate.begin_update();
    assert!(PluginReadGuard::acquire_for(vec![&first, &second], Duration::ZERO).is_none());
    first.gate.finish_update();
    second.gate.finish_update();
}

#[test]
fn immediate_multi_read_deduplicates_and_releases_partial_guards() {
    let first = TestPlugin::new();
    let second = TestPlugin::new();
    let plugins: [&dyn Plugin; 4] = [&second, &first, &second, &first];
    let read = PluginReadGuard::try_acquire(&plugins).ok().unwrap();
    assert!(matches!(&read._guards, Guards::Multiple { _guards } if _guards.len() == 2));
    drop(read);

    // Close the last gate in acquisition order, forcing a partial acquisition.
    let sorted = PluginReadGuard::sorted_plugins(plugins.to_vec());
    sorted[1].gate().begin_update();
    let blocked = PluginReadGuard::try_acquire(&plugins).err().unwrap();
    assert!(std::ptr::addr_eq(blocked, sorted[1]));
    assert!(sorted[0].gate().0.gate.try_write().is_some());
    sorted[1].gate().finish_update();
    assert!(PluginReadGuard::try_acquire(&plugins).is_ok());
}

#[test]
fn multi_read_releases_partial_set_while_waiting() {
    let first = TestPlugin::new();
    let second = TestPlugin::new();
    second.gate.begin_update();

    thread::scope(|scope| {
        let reader = scope.spawn(|| {
            PluginReadGuard::acquire_for(
                vec![&first as &dyn Plugin, &second as &dyn Plugin],
                Duration::from_secs(1),
            )
        });

        thread::sleep(Duration::from_millis(10));
        let first_gate = first.gate.clone();
        let (closed_tx, closed_rx) = mpsc::channel();
        let writer = scope.spawn(move || {
            first_gate.begin_update();
            closed_tx.send(()).unwrap();
        });

        closed_rx.recv_timeout(Duration::from_secs(1)).unwrap();
        writer.join().unwrap();
        second.gate.finish_update();
        first.gate.finish_update();

        assert!(reader.join().unwrap().is_some());
    });
}

#[test]
fn multi_read_stops_waiting_at_its_deadline() {
    let first = TestPlugin::new();
    let second = TestPlugin::new();
    second.gate.begin_update();

    assert!(
        PluginReadGuard::acquire_for(
            vec![&first as &dyn Plugin, &second as &dyn Plugin],
            Duration::from_millis(10),
        )
        .is_none()
    );

    second.gate.finish_update();
}
