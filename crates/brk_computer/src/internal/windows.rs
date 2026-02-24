//! Base generic struct with 4 type parameters — one per rolling window duration.
//!
//! Foundation for all rolling window types (24h, 7d, 30d, 1y).

use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct Windows<A, B = A, C = A, D = A> {
    #[traversable(rename = "24h")]
    pub _24h: A,
    #[traversable(rename = "7d")]
    pub _7d: B,
    #[traversable(rename = "30d")]
    pub _30d: C,
    #[traversable(rename = "1y")]
    pub _1y: D,
}
