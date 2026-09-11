//! Ordered range boundaries with cached, shared, and cursor-based reverse lookups.

mod base;
mod cached_cursor;
mod cursor;
mod share;

pub use base::RangeMap;
pub use cached_cursor::CachedRangeMapCursor;
pub use cursor::RangeMapCursor;
pub use share::SharedRangeMap;
