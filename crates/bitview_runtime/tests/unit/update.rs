use std::time::Duration;

use bitview_plugin::{Plugin, PluginId, PluginStorage, Publication};
use bitview_traversable::Traversable;
use brk_error::Error;
use brk_exit::Exit;
use brk_types::Version;

use super::*;

#[derive(Traversable)]
struct TestPlugin {}

impl Plugin for TestPlugin {
    fn storage(&self) -> PluginStorage {
        PluginStorage::new(PluginId::new("test"), Version::ONE)
    }
}

#[derive(crate::PluginSet)]
struct TestPlugins {
    first: TestPlugin,
    second: TestPlugin,
    #[plugin_set(skip)]
    publication: Publication,
    #[plugin_set(skip)]
    computed_while_closed: bool,
    #[plugin_set(skip)]
    committed_while_closed: bool,
    #[plugin_set(skip)]
    fail_compute: bool,
    #[plugin_set(skip)]
    fail_commit: bool,
}

impl TestPlugins {
    fn new() -> Self {
        Self {
            first: TestPlugin {},
            second: TestPlugin {},
            publication: Publication::default(),
            computed_while_closed: false,
            committed_while_closed: false,
            fail_compute: false,
            fail_commit: false,
        }
    }
}

impl ComputePluginSet for TestPlugins {
    fn publication(&self) -> &Publication {
        &self.publication
    }

    fn compute(&mut self, _context: UpdateContext<'_>) -> Result<()> {
        self.computed_while_closed = self.publication().try_read().is_none();
        if self.fail_compute {
            Err(Error::Internal("test compute failure"))
        } else {
            Ok(())
        }
    }

    fn commit(&mut self) -> Result<()> {
        self.committed_while_closed = self.publication().try_read().is_none();
        if self.fail_commit {
            Err(Error::Internal("test commit failure"))
        } else {
            Ok(())
        }
    }
}

#[test]
fn publishes_once_only_after_complete_compute_and_commit() -> Result<()> {
    let mut plugins = TestPlugins::new();
    let reader = plugins.publication().clone();
    let exit = Exit::new();
    update(&mut plugins, UpdateContext::new(&exit))?;
    assert!(plugins.computed_while_closed);
    assert!(plugins.committed_while_closed);
    assert!(reader.try_read().is_some());
    assert_eq!(reader.revision(), 1);
    update(&mut plugins, UpdateContext::new(&exit))?;
    assert_eq!(reader.revision(), 2);
    Ok(())
}

#[test]
fn failures_keep_the_whole_pipeline_closed() {
    for fail_compute in [false, true] {
        let mut plugins = TestPlugins::new();
        plugins.fail_compute = fail_compute;
        plugins.fail_commit = !fail_compute;
        let reader = plugins.publication().clone();
        let exit = Exit::new();
        assert!(update(&mut plugins, UpdateContext::new(&exit)).is_err());
        assert!(plugins.computed_while_closed);
        assert_eq!(plugins.committed_while_closed, !fail_compute);
        assert!(reader.try_read().is_none());
        assert!(reader.read_for(Duration::from_millis(10)).is_none());
        assert_eq!(reader.revision(), 0);
    }
}
