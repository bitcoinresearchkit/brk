//! Global allocator and memory utilities for brk.
//!
//! This crate sets mimalloc as the global allocator and provides
//! utilities for monitoring and managing memory.

use libmimalloc_sys::mi_collect;
use mimalloc::MiMalloc as Allocator;

#[global_allocator]
static GLOBAL: Allocator = Allocator;

/// Mimalloc allocator utilities
pub struct Mimalloc;

impl Mimalloc {
    /// Eagerly free memory back to OS.
    /// Only call at natural pause points.
    #[inline]
    pub fn collect() {
        unsafe { mi_collect(true) }
    }
}
