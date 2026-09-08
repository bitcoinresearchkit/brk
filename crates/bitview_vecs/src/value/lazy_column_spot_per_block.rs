use bitview_cohort::{ADDR_TYPE_IDS, AddrTypeId, WithAddrTypes};
use bitview_transforms::{CentsUnsignedToDollars, SatsToBitcoin, SatsToCents};
use bitview_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{
    BinaryTransform, CacheBudget, ColumnId, Ident, PcoVec, PinnedCachedVec, ReadOnlyColumnarVec,
    ReadableCloneableVec, ReadableColumnarVec,
};

use crate::{
    IndexSources, LazyColumnPerBlock, LazyIndexedVec, LazyPerBlock, LazySpotValuePerBlock,
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
        cache: &'static CacheBudget,
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, Sats>, C>,
        column: C,
        indexes: &IndexSources,
        spot_price: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Self {
        let sats = LazyColumnPerBlock::new(
            cache,
            &format!("{name}_sats"),
            version,
            source,
            column,
            indexes,
        );
        let btc = LazyPerBlock::from_resolutions::<SatsToBitcoin>(name, version, &sats.resolutions);
        let cents_source = LazyIndexedVec::new(
            &format!("{name}_cents_source"),
            version,
            sats.resolutions.height_source(),
            spot_price,
            |_, sats, spot| SatsToCents::apply(sats, spot),
        );
        let cents = LazyPerBlock::from_height_source::<Ident>(
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

impl LazyColumnSpotValuePerBlock<AddrTypeId> {
    pub fn with_addr_types(
        cache: &'static CacheBudget,
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, Sats>, AddrTypeId>,
        indexes: &IndexSources,
        spot_price: &impl ReadableCloneableVec<Height, Cents>,
    ) -> WithAddrTypes<Self, LazySpotValuePerBlock> {
        let sats = PinnedCachedVec::wrap(source.sum_columns(
            &format!("{name}_sats"),
            version,
            ADDR_TYPE_IDS,
        ));
        let all =
            LazySpotValuePerBlock::from_sats_source(name, version, &sats, indexes, spot_price);
        let by_addr_type = AddrTypeId::series(|column, type_name| {
            LazyColumnSpotValuePerBlock::new(
                cache,
                &format!("{type_name}_{name}"),
                version,
                source,
                column,
                indexes,
                spot_price,
            )
        });

        WithAddrTypes { all, by_addr_type }
    }
}
