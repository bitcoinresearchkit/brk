use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, Version};
use vecdb::{IterableBoxedVec, LazyVecFrom1, LazyVecFrom2};

use crate::internal::{ClosePriceTimesSats, SatsToBitcoin};

#[derive(Clone, Traversable)]
pub struct LazyDerivedValuesHeight {
    pub bitcoin: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub dollars: Option<LazyVecFrom2<Height, Dollars, Height, Close<Dollars>, Height, Sats>>,
}

const VERSION: Version = Version::ZERO;

impl LazyDerivedValuesHeight {
    pub fn from_source(
        name: &str,
        sats_source: IterableBoxedVec<Height, Sats>,
        version: Version,
        price_source: Option<IterableBoxedVec<Height, Close<Dollars>>>,
    ) -> Self {
        let bitcoin = LazyVecFrom1::transformed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            version + VERSION,
            sats_source.clone(),
        );

        let dollars = price_source.map(|price| {
            LazyVecFrom2::transformed::<ClosePriceTimesSats>(
                &format!("{name}_usd"),
                version + VERSION,
                price,
                sats_source,
            )
        });

        Self { bitcoin, dollars }
    }
}
