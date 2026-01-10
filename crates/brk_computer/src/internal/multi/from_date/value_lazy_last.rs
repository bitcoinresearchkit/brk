//! Lazy value type for Last pattern from DateIndex.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{LazyFromDateLast, ValueFromHeightLast, ValueFromDateLast};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
pub struct LazyValueFromDateLast {
    pub sats: LazyFromDateLast<Sats, Sats>,
    pub bitcoin: LazyFromDateLast<Bitcoin, Sats>,
    pub dollars: Option<LazyFromDateLast<Dollars, Dollars>>,
}

impl LazyValueFromDateLast {
    pub fn from_source<SatsTransform, BitcoinTransform, DollarsTransform>(
        name: &str,
        source: &ValueFromDateLast,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        DollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let v = version + VERSION;

        let sats = LazyFromDateLast::from_derived::<SatsTransform>(
            name,
            v,
            source.sats_dateindex.boxed_clone(),
            &source.sats,
        );

        let bitcoin = LazyFromDateLast::from_derived::<BitcoinTransform>(
            &format!("{name}_btc"),
            v,
            source.sats_dateindex.boxed_clone(),
            &source.sats,
        );

        let dollars = source.dollars.as_ref().map(|dollars_source| {
            LazyFromDateLast::from_computed::<DollarsTransform>(
                &format!("{name}_usd"),
                v,
                dollars_source.dateindex.boxed_clone(),
                dollars_source,
            )
        });

        Self { sats, bitcoin, dollars }
    }

    pub fn from_block_source<SatsTransform, BitcoinTransform, DollarsTransform>(
        name: &str,
        source: &ValueFromHeightLast,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        DollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let v = version + VERSION;

        let sats = LazyFromDateLast::from_derived::<SatsTransform>(
            name,
            v,
            source.sats.rest.dateindex.boxed_clone(),
            &source.sats.rest.dates,
        );

        let bitcoin = LazyFromDateLast::from_derived::<BitcoinTransform>(
            &format!("{name}_btc"),
            v,
            source.sats.rest.dateindex.boxed_clone(),
            &source.sats.rest.dates,
        );

        let dollars = source.dollars.as_ref().map(|dollars_source| {
            LazyFromDateLast::from_derived::<DollarsTransform>(
                &format!("{name}_usd"),
                v,
                dollars_source.rest.dateindex.boxed_clone(),
                &dollars_source.rest.dates,
            )
        });

        Self { sats, bitcoin, dollars }
    }
}
