use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use bitview_plugin::{Plugin, PluginGate, PluginId, PluginReadGuard, PluginStorage};
use bitview_traversable::Traversable;
use brk_error::Error;
use brk_exit::Exit;
use brk_types::Version;

use super::*;

#[derive(bitview_traversable::Traversable)]
struct TestPlugin {
    #[traversable(skip)]
    gate: PluginGate,
}

impl Plugin for TestPlugin {
    fn storage(&self) -> PluginStorage {
        PluginStorage::new(PluginId::new("test"), Version::ONE)
    }

    fn gate(&self) -> &PluginGate {
        &self.gate
    }
}

#[derive(crate::PluginSet)]
struct TestPlugins {
    plugin: TestPlugin,
    #[plugin_set(skip)]
    computed_while_closed: AtomicBool,
    #[plugin_set(skip)]
    committed_while_closed: AtomicBool,
    #[plugin_set(skip)]
    fail_commit: bool,
}

impl ComputePluginSet for TestPlugins {
    fn compute(&mut self, _context: UpdateContext<'_>) -> Result<()> {
        self.computed_while_closed
            .store(self.plugin.gate().try_read().is_none(), Ordering::Relaxed);
        Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        self.committed_while_closed
            .store(self.plugin.gate().try_read().is_none(), Ordering::Relaxed);
        if self.fail_commit {
            Err(Error::Internal("test commit failure"))
        } else {
            Ok(())
        }
    }
}

#[test]
fn publishes_only_after_compute_and_commit_complete() -> Result<()> {
    let mut plugins = TestPlugins {
        plugin: TestPlugin {
            gate: PluginGate::new(),
        },
        computed_while_closed: AtomicBool::new(false),
        committed_while_closed: AtomicBool::new(false),
        fail_commit: false,
    };

    let exit = Exit::new();
    update(&mut plugins, UpdateContext::new(&exit))?;

    assert!(plugins.computed_while_closed.load(Ordering::Relaxed));
    assert!(plugins.committed_while_closed.load(Ordering::Relaxed));
    assert!(plugins.plugin.gate().try_read().is_some());
    Ok(())
}

#[test]
fn commit_failure_keeps_plugin_closed() {
    let mut plugins = TestPlugins {
        plugin: TestPlugin {
            gate: PluginGate::new(),
        },
        computed_while_closed: AtomicBool::new(false),
        committed_while_closed: AtomicBool::new(false),
        fail_commit: true,
    };

    let exit = Exit::new();
    assert!(update(&mut plugins, UpdateContext::new(&exit)).is_err());
    assert!(plugins.computed_while_closed.load(Ordering::Relaxed));
    assert!(plugins.committed_while_closed.load(Ordering::Relaxed));
    assert!(plugins.plugin.gate().try_read().is_none());
    assert!(
        PluginReadGuard::acquire_for(
            vec![&plugins.plugin as &dyn Plugin],
            Duration::from_millis(10),
        )
        .is_none()
    );
}
