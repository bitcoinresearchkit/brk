//! Traits for consistent state flushing and importing.
//!
//! These traits ensure all stateful components follow the same patterns
//! for checkpoint/resume operations, preventing bugs where new fields
//! are forgotten during flush operations.

use brk_error::Result;
use brk_types::Height;
use vecdb::Exit;

/// Trait for components that can be flushed to disk.
///
/// This is for simple flush operations that don't require height tracking.
pub trait Flushable {
    /// Safely flush data to disk.
    fn safe_flush(&mut self, exit: &Exit) -> Result<()>;
}

/// Trait for stateful components that track data indexed by height.
///
/// This ensures consistent patterns for:
/// - Flushing state at checkpoints
/// - Importing state when resuming from a checkpoint
/// - Resetting state when starting from scratch
pub trait HeightFlushable {
    /// Flush state to disk at the given height checkpoint.
    fn flush_at_height(&mut self, height: Height, exit: &Exit) -> Result<()>;

    /// Import state from the most recent checkpoint at or before the given height.
    /// Returns the actual height that was imported.
    fn import_at_or_before(&mut self, height: Height) -> Result<Height>;

    /// Reset state for starting from scratch.
    fn reset(&mut self) -> Result<()>;
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

/// Blanket implementation for Option<T> where T: HeightFlushable
impl<T: HeightFlushable> HeightFlushable for Option<T> {
    fn flush_at_height(&mut self, height: Height, exit: &Exit) -> Result<()> {
        if let Some(inner) = self.as_mut() {
            inner.flush_at_height(height, exit)?;
        }
        Ok(())
    }

    fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        if let Some(inner) = self.as_mut() {
            inner.import_at_or_before(height)
        } else {
            Ok(height)
        }
    }

    fn reset(&mut self) -> Result<()> {
        if let Some(inner) = self.as_mut() {
            inner.reset()?;
        }
        Ok(())
    }
}
