use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{BinaryTransform, ReadableBoxedVec, LazyVecFrom1, LazyVecFrom2, UnaryTransform};

use crate::internal::LazyDerivedValuesHeight;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyFromHeightValue {
    #[traversable(rename = "sats")]
    pub sats: LazyVecFrom1<Height, Sats, Height, Sats>,
    #[deref]
    #[deref_mut]
    pub rest: LazyDerivedValuesHeight,
}

impl LazyFromHeightValue {
    pub(crate) fn from_sources<SatsTransform, BitcoinTransform, DollarsTransform>(
        name: &str,
        sats_source: ReadableBoxedVec<Height, Sats>,
        price_source: ReadableBoxedVec<Height, Dollars>,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        DollarsTransform: BinaryTransform<Dollars, Sats, Dollars>,
    {
        let v = version + VERSION;

        let sats = LazyVecFrom1::transformed::<SatsTransform>(name, v, sats_source.clone());

        let btc = LazyVecFrom1::transformed::<BitcoinTransform>(
            &format!("{name}_btc"),
            v,
            sats_source.clone(),
        );

        let usd = LazyVecFrom2::transformed::<DollarsTransform>(
            &format!("{name}_usd"),
            v,
            price_source,
            sats_source,
        );

        Self {
            sats,
            rest: LazyDerivedValuesHeight { btc, usd },
        }
    }
}
