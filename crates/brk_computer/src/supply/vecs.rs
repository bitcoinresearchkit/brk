use brk_traversable::Traversable;
use vecdb::Database;

use super::{burned, circulating, inflation, market_cap, velocity};

/// Supply metrics module
///
/// This module owns all supply-related metrics:
/// - circulating: Lazy references to distribution's actual circulating supply
/// - burned: Cumulative opreturn and unspendable supply
/// - inflation: Inflation rate derived from supply
/// - velocity: BTC and USD velocity metrics
/// - market_cap: Lazy references to supply in USD (circulating * price)
#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub circulating: circulating::Vecs,
    pub burned: burned::Vecs,
    pub inflation: inflation::Vecs,
    pub velocity: velocity::Vecs,
    pub market_cap: Option<market_cap::Vecs>,
}
