//! Computation context holding shared state during block processing.

use brk_types::{Dollars, Height, Timestamp};
use vecdb::VecIndex;

use crate::price;

/// Context shared across block processing.
pub struct ComputeContext<'a> {
    /// Starting height for this computation run
    pub starting_height: Height,

    /// Last height to process
    pub last_height: Height,

    /// Whether price data is available
    pub compute_dollars: bool,

    /// Price data (optional)
    pub price: Option<&'a price::Vecs>,

    /// Pre-computed height -> timestamp mapping
    pub height_to_timestamp: Vec<Timestamp>,

    /// Pre-computed height -> price mapping (if available)
    pub height_to_price: Option<Vec<Dollars>>,
}

impl<'a> ComputeContext<'a> {
    /// Get price at height (None if no price data or height out of range).
    pub fn price_at(&self, height: Height) -> Option<Dollars> {
        self.height_to_price.as_ref()?.get(height.to_usize()).copied()
    }

    /// Get timestamp at height.
    pub fn timestamp_at(&self, height: Height) -> Timestamp {
        self.height_to_timestamp[height.to_usize()]
    }
}
