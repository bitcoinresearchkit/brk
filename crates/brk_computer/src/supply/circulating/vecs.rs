use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats};
use vecdb::LazyVecFrom1;

use crate::internal::LazyValueDateLast;

/// Circulating supply - lazy references to distribution's actual supply (KISS)
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height_to_sats: LazyVecFrom1<Height, Sats, Height, Sats>,
    pub height_to_btc: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub height_to_usd: Option<LazyVecFrom1<Height, Dollars, Height, Dollars>>,
    pub indexes: LazyValueDateLast,
}
