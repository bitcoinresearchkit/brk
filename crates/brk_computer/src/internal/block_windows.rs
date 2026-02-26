//! Base generic struct with 2 type parameters — one per rolling window duration.
//!
//! Foundation for tx-derived rolling window types (1h, 24h — actual time-based).

use brk_traversable::Traversable;

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct BlockWindows<A, B = A> {
    #[traversable(rename = "1h")]
    pub _1h: A,
    #[traversable(rename = "24h")]
    pub _24h: B,
}

