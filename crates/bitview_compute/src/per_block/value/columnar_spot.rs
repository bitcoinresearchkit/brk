use bitview_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{BinaryTransform, ColumnId, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec};

use crate::{
    CentsUnsignedToDollars, Identity, IndexSources, LazyColumnPerBlock, LazyIndexedVec,
    LazyPerBlock, SatsToBitcoin, SatsToCents,
};

#[derive(Clone, Traversable)]
pub struct LazyColumnSpotValuePerBlock<C>
where
    C: ColumnId,
{
    /// Reported in BTC; one BTC equals 100,000,000 satoshis.
    pub btc: LazyPerBlock<Bitcoin, Sats>,
    /// Reported in satoshis.
    pub sats: LazyColumnPerBlock<Sats, C>,
    /// Reported in US dollars.
    pub usd: LazyPerBlock<Dollars, Cents>,
    /// Reported in US cents; 100 cents equal one US dollar.
    pub cents: LazyPerBlock<Cents>,
}

impl<C> LazyColumnSpotValuePerBlock<C>
where
    C: ColumnId,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, Sats>, C>,
        column: C,
        indexes: &IndexSources,
        spot_price: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Self {
        let sats =
            LazyColumnPerBlock::new(&format!("{name}_sats"), version, source, column, indexes);
        let btc = LazyPerBlock::from_resolutions::<SatsToBitcoin>(name, version, &sats.resolutions);
        let cents_source = LazyIndexedVec::new(
            &format!("{name}_cents_source"),
            version,
            sats.resolutions.height_source(),
            spot_price,
            |_, sats, spot| SatsToCents::apply(sats, spot),
        );
        let cents = LazyPerBlock::from_height_source::<Identity<Cents>>(
            &format!("{name}_cents"),
            version,
            &cents_source,
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
