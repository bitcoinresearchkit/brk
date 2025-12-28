//! Global allocator and memory utilities for brk.
//!
//! This crate sets mimalloc as the global allocator and provides
//! utilities for monitoring and managing memory.
//! ```

use std::{fmt, mem::MaybeUninit};

use log::info;
use mimalloc::MiMalloc as Allocator;

#[global_allocator]
static GLOBAL: Allocator = Allocator;

/// Mimalloc allocator utilities
pub struct Mimalloc;

impl Mimalloc {
    /// Get current mimalloc memory statistics.
    /// Very fast (~100-500ns) - uses getrusage/mach syscalls, no file I/O.
    #[inline]
    pub fn stats() -> Stats {
        let mut elapsed_msecs = MaybeUninit::uninit();
        let mut user_msecs = MaybeUninit::uninit();
        let mut system_msecs = MaybeUninit::uninit();
        let mut current_rss = MaybeUninit::uninit();
        let mut peak_rss = MaybeUninit::uninit();
        let mut current_commit = MaybeUninit::uninit();
        let mut peak_commit = MaybeUninit::uninit();
        let mut page_faults = MaybeUninit::uninit();

        unsafe {
            libmimalloc_sys::mi_process_info(
                elapsed_msecs.as_mut_ptr(),
                user_msecs.as_mut_ptr(),
                system_msecs.as_mut_ptr(),
                current_rss.as_mut_ptr(),
                peak_rss.as_mut_ptr(),
                current_commit.as_mut_ptr(),
                peak_commit.as_mut_ptr(),
                page_faults.as_mut_ptr(),
            );

            Stats {
                current_rss: current_rss.assume_init(),
                peak_rss: peak_rss.assume_init(),
                current_commit: current_commit.assume_init(),
                peak_commit: peak_commit.assume_init(),
            }
        }
    }

    /// Eagerly free memory back to OS.
    /// This is expensive - only call at natural pause points.
    #[inline]
    pub fn collect() {
        unsafe { libmimalloc_sys::mi_collect(true) }
    }

    /// Collect if wasted memory exceeds threshold (in MB).
    /// Returns true if collection was triggered.
    pub fn collect_if_wasted_above(threshold_mb: usize) -> bool {
        let stats = Self::stats();

        info!("Mimalloc stats: {:?}", stats);

        if stats.wasted_mb() > threshold_mb {
            info!(
                "Mimalloc wasted {} MB (commit: {} MB, rss: {} MB), collecting...",
                stats.wasted_mb(),
                stats.commit_mb(),
                stats.rss_mb(),
            );
            Self::collect();
            true
        } else {
            false
        }
    }

    /// Force collection and return stats before/after.
    pub fn force_collect() -> (Stats, Stats) {
        let before = Self::stats();
        Self::collect();
        let after = Self::stats();

        info!(
            "Mimalloc collected: {} MB -> {} MB (freed {} MB)",
            before.commit_mb(),
            after.commit_mb(),
            before.commit_mb().saturating_sub(after.commit_mb()),
        );

        (before, after)
    }
}

/// Memory stats from mimalloc
#[derive(Debug, Clone, Copy)]
pub struct Stats {
    /// Resident set size (physical memory used)
    pub current_rss: usize,
    pub peak_rss: usize,
    /// Committed memory (virtual memory reserved)
    pub current_commit: usize,
    pub peak_commit: usize,
}

impl Stats {
    /// Returns wasted memory in bytes (commit - rss).
    /// High values suggest fragmentation.
    #[inline]
    pub fn wasted(&self) -> usize {
        self.current_commit.saturating_sub(self.current_rss)
    }

    /// Returns wasted memory in MB.
    #[inline]
    pub fn wasted_mb(&self) -> usize {
        self.wasted() / 1024 / 1024
    }

    /// Returns current RSS in MB.
    #[inline]
    pub fn rss_mb(&self) -> usize {
        self.current_rss / 1024 / 1024
    }

    /// Returns current commit in MB.
    #[inline]
    pub fn commit_mb(&self) -> usize {
        self.current_commit / 1024 / 1024
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rss: {} MB, commit: {} MB, wasted: {} MB",
            self.rss_mb(),
            self.commit_mb(),
            self.wasted_mb()
        )
    }
}
