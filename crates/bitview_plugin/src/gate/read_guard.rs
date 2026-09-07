use std::time::{Duration, Instant};

use parking_lot::{ArcRwLockReadGuard, RawRwLock};

use super::PluginGate;
use crate::Plugin;

/// Keeps a Bitview plugin's mutable state stable for one logical read.
pub struct PluginReadGuard {
    _guards: Guards,
}

enum Guards {
    Single {
        _guard: ArcRwLockReadGuard<RawRwLock, ()>,
    },
    Multiple {
        _guards: Vec<ArcRwLockReadGuard<RawRwLock, ()>>,
    },
}

impl PluginGate {
    /// Attempts to stabilize this plugin for one logical read.
    ///
    /// This never blocks. Async callers can retry cooperatively while an
    /// update is running without tying up an executor thread.
    pub fn try_read(&self) -> Option<PluginReadGuard> {
        Some(PluginReadGuard {
            _guards: Guards::Single {
                _guard: self.0.gate.try_read_arc()?,
            },
        })
    }

    /// Waits up to `timeout` to stabilize this plugin for one logical read.
    pub fn read_for(&self, timeout: Duration) -> Option<PluginReadGuard> {
        Some(PluginReadGuard {
            _guards: Guards::Single {
                _guard: self.0.gate.try_read_arc_for(timeout)?,
            },
        })
    }
}

impl PluginReadGuard {
    /// Waits up to `timeout` to acquire multiple plugin gates without
    /// retaining a partial set while an update is running.
    pub fn acquire_for(mut plugins: Vec<&dyn Plugin>, timeout: Duration) -> Option<Self> {
        plugins.sort_unstable_by_key(|plugin| {
            let ptr = *plugin as *const dyn Plugin;
            ptr.cast::<()>() as usize
        });
        plugins.dedup_by(|a, b| std::ptr::addr_eq(*a, *b));
        let started = Instant::now();

        loop {
            let mut guards = Vec::with_capacity(plugins.len());
            let blocked = plugins.iter().find(|plugin| {
                let Some(guard) = plugin.gate().0.gate.try_read_arc() else {
                    return true;
                };
                guards.push(guard);
                false
            });
            let Some(blocked) = blocked else {
                return Some(Self {
                    _guards: Guards::Multiple { _guards: guards },
                });
            };

            drop(guards);
            let remaining = timeout.saturating_sub(started.elapsed());
            if remaining.is_zero() {
                return None;
            }
            drop(blocked.gate().read_for(remaining)?);
        }
    }
}

#[cfg(test)]
#[path = "../../tests/unit/read_guard.rs"]
mod tests;
