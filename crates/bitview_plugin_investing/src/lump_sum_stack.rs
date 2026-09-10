use bitview_plugin_mappings::Vecs as MappingVecs;
use bitview_plugin_price::Vecs as PriceVecs;
use bitview_transforms::SatsToCents;
use bitview_traversable::Traversable;
use bitview_vecs::{LazyPerBlock, LazySpotValuePerBlock, LazyWindowVec};
use brk_error::Result;
use brk_types::{Cents, Dollars, Height, Sats, Version};
use vecdb::{BinaryTransform, Ident, ReadableCloneableVec};

use crate::DCA_AMOUNT;

#[derive(Clone, derive_more::Deref, derive_more::DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LumpSumStack(pub LazySpotValuePerBlock);

impl LumpSumStack {
    pub fn from_window(
        name: &str,
        days: u32,
        version: Version,
        mappings: &MappingVecs,
        window_starts: &impl ReadableCloneableVec<Height, Height>,
        prices: &PriceVecs,
    ) -> Result<Self> {
        let total_invested = DCA_AMOUNT * days as usize;

        let sats_source = LazyWindowVec::<Height, Cents, Sats>::new(
            &format!("{name}_sats_source"),
            version,
            &prices.spot.cents.height,
            window_starts,
            false,
            move |_, past, _| Self::sats_at_price(total_invested, past),
        );
        let sats = LazyPerBlock::from_height_source::<Ident>(
            &format!("{name}_sats"),
            version,
            &sats_source,
            mappings,
        );

        let cents_source = LazyWindowVec::<Height, Cents, Cents>::new(
            &format!("{name}_cents_source"),
            version,
            &prices.spot.cents.height,
            window_starts,
            false,
            move |current, past, _| {
                SatsToCents::apply(Self::sats_at_price(total_invested, past), current)
            },
        );
        let cents = LazyPerBlock::from_height_source::<Ident>(
            &format!("{name}_cents"),
            version,
            &cents_source,
            mappings,
        );
        Ok(Self(LazySpotValuePerBlock::from_sats_and_cents(
            name, version, sats, cents,
        )))
    }

    #[inline(always)]
    fn sats_at_price(total_invested: Dollars, price: Cents) -> Sats {
        Sats::from_dollars_at_price(total_invested, Dollars::from(price))
    }
}
