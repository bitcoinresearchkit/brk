use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{BinaryTransform, IterableBoxedVec, LazyVecFrom1, LazyVecFrom2, UnaryTransform};

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
    pub fn from_sources<SatsTransform, BitcoinTransform, DollarsTransform>(
        name: &str,
        sats_source: IterableBoxedVec<Height, Sats>,
        price_source: Option<IterableBoxedVec<Height, Close<Dollars>>>,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        DollarsTransform: BinaryTransform<Close<Dollars>, Sats, Dollars>,
    {
        let v = version + VERSION;

        let sats = LazyVecFrom1::transformed::<SatsTransform>(name, v, sats_source.clone());

        let bitcoin = LazyVecFrom1::transformed::<BitcoinTransform>(
            &format!("{name}_btc"),
            v,
            sats_source.clone(),
        );

        let dollars = price_source.map(|price| {
            LazyVecFrom2::transformed::<DollarsTransform>(
                &format!("{name}_usd"),
                v,
                price,
                sats_source,
            )
        });

        Self {
            sats,
            rest: LazyDerivedValuesHeight { bitcoin, dollars },
        }
    }
}
