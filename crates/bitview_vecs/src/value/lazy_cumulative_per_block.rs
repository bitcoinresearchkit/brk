use bitview_transforms::{CentsUnsignedToDollars, SatsToBitcoin};
use bitview_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{Ident, ReadableCloneableVec};

use crate::{IndexSources, LazyPerBlock};

#[derive(Clone, Traversable)]
pub struct LazyCumulativeValuePerBlock {
    /// Reported in BTC; one BTC equals 100,000,000 satoshis.
    pub btc: LazyPerBlock<Bitcoin, Sats>,
    /// Reported in satoshis.
    pub sats: LazyPerBlock<Sats>,
    /// Reported in US dollars.
    pub usd: LazyPerBlock<Dollars, Cents>,
    /// Reported in US cents; 100 cents equal one US dollar.
    pub cents: LazyPerBlock<Cents>,
}

impl LazyCumulativeValuePerBlock {
    pub fn from_sources(
        name: &str,
        version: Version,
        cumulative_sats: &(impl ReadableCloneableVec<Height, Sats> + ?Sized),
        cumulative_cents: &(impl ReadableCloneableVec<Height, Cents> + ?Sized),
        indexes: &IndexSources,
    ) -> Self {
        let sats = LazyPerBlock::from_height_source::<Ident>(
            &format!("{name}_sats"),
            version,
            cumulative_sats,
            indexes,
        );
        let btc = LazyPerBlock::from_lazy::<SatsToBitcoin, Sats>(name, version, &sats);
        let cents = LazyPerBlock::from_height_source::<Ident>(
            &format!("{name}_cents"),
            version,
            cumulative_cents,
            indexes,
        );
        let usd = LazyPerBlock::from_lazy::<CentsUnsignedToDollars, Cents>(
            &format!("{name}_usd"),
            version,
            &cents,
        );
        Self {
            btc,
            sats,
            usd,
            cents,
        }
    }
}
