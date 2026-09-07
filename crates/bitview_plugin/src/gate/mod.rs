use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use parking_lot::{ArcRwLockWriteGuard, Mutex, RawRwLock, RwLock};

mod read_guard;

pub use read_guard::PluginReadGuard;

/// Shared publication gate for one Bitview plugin.
///
/// Clones refer to the same gate. An update stays closed until
/// [`finish_update`](Self::finish_update) is called explicitly, so an error
/// cannot expose partially updated mutable state.
#[derive(Clone, Default)]
pub struct PluginGate(Arc<Inner>);

#[derive(Default)]
struct Inner {
    gate: Arc<RwLock<()>>,
    writer: Mutex<Option<ArcRwLockWriteGuard<RawRwLock, ()>>>,
    publication: AtomicU64,
    #[cfg(feature = "tokio")]
    changed: tokio::sync::watch::Sender<()>,
}

impl PluginGate {
    pub fn new() -> Self {
        Self::default()
    }

    /// Waits for existing readers, then closes the plugin to new readers.
    /// Calling this while the same update is already running is a no-op.
    pub fn begin_update(&self) {
        let mut writer = self.0.writer.lock();
        if writer.is_some() {
            return;
        }

        *writer = Some(self.0.gate.write_arc());
    }

    /// Publishes the completed update and admits new readers.
    ///
    /// # Panics
    ///
    /// Panics when no update is running.
    pub fn finish_update(&self) {
        let writer = self
            .0
            .writer
            .lock()
            .take()
            .expect("plugin update is not running");
        self.0.publication.fetch_add(1, Ordering::Release);
        drop(writer);
        #[cfg(feature = "tokio")]
        self.0.changed.send_replace(());
    }

    /// Subscribe before checking readiness so publication cannot be missed.
    #[cfg(feature = "tokio")]
    pub fn changes(&self) -> tokio::sync::watch::Receiver<()> {
        self.0.changed.subscribe()
    }

    /// Process-local revision of completed publications, shared by gate clones.
    /// Read while holding this gate when pairing the revision with plugin data.
    /// It advances even if a publication leaves the chain tip unchanged.
    pub fn publication(&self) -> u64 {
        self.0.publication.load(Ordering::Acquire)
    }
}

#[cfg(test)]
#[path = "../../tests/unit/gate.rs"]
mod tests;
