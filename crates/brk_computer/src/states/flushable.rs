//! Traits for consistent state flushing and importing.
//!
//! These traits ensure all stateful components follow the same patterns
//! for checkpoint/resume operations, preventing bugs where new fields
//! are forgotten during flush operations.

use brk_error::Result;
use vecdb::Exit;

/// Trait for components that can be flushed to disk.
///
/// This is for simple flush operations that don't require height tracking.
pub trait Flushable {
    /// Safely flush data to disk with fsync for durability.
    fn safe_flush(&mut self, exit: &Exit) -> Result<()>;
}

/// Blanket implementation for Option<T> where T: Flushable
impl<T: Flushable> Flushable for Option<T> {
    fn safe_flush(&mut self, exit: &Exit) -> Result<()> {
        if let Some(inner) = self.as_mut() {
            inner.safe_flush(exit)?;
        }
        Ok(())
    }
}
