#[cfg(feature = "storage")]
use bitview_traversable::Traversable;

/// Open, high, low and close fields; open and close may use different sampling shapes.
#[derive(Clone)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct Ohlc<T, C = T, O = T> {
    /// Opening price for each supported period. A populated period uses its
    /// first block-level price; an empty period carries the previous close.
    pub open: O,
    /// Highest price for each supported period. A populated period uses its
    /// maximum block-level price; an empty period carries the previous close.
    pub high: T,
    /// Lowest price for each supported period. A populated period uses its
    /// minimum block-level price; an empty period carries the previous close.
    pub low: T,
    /// Closing price for each supported period. A populated period uses its
    /// final block-level price; an empty period is null.
    pub close: C,
}
