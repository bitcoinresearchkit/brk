pub mod bytes;
pub mod conversions;

/// Marker for tracking when data was last modified.
///
/// Used for change tracking, rollback support, and ETag generation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[must_use = "Stamp values should be used for tracking"]
pub struct Stamp(u64);

impl Stamp {
    pub fn new(stamp: u64) -> Self {
        Self(stamp)
    }
}
