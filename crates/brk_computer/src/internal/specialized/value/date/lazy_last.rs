//! Lazy value type for Last pattern from DateIndex.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::internal::{LazyDateLast, ValueDateLast};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
pub struct LazyValueDateLast {
    pub sats: LazyDateLast<Sats, Sats>,
    pub bitcoin: LazyDateLast<Bitcoin, Sats>,
    pub dollars: Option<LazyDateLast<Dollars, Dollars>>,
}

impl LazyValueDateLast {
    pub fn from_source<SatsTransform, BitcoinTransform, DollarsTransform>(
        name: &str,
        source: &ValueDateLast,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        DollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let v = version + VERSION;

        let sats = LazyDateLast::from_derived::<SatsTransform>(
            name,
            v,
            source.sats_dateindex.boxed_clone(),
            &source.sats,
        );

        let bitcoin = LazyDateLast::from_derived::<BitcoinTransform>(
            &format!("{name}_btc"),
            v,
            source.sats_dateindex.boxed_clone(),
            &source.sats,
        );

        let dollars = source.dollars.as_ref().map(|dollars_source| {
            LazyDateLast::from_computed::<DollarsTransform>(
                &format!("{name}_usd"),
                v,
                dollars_source.dateindex.boxed_clone(),
                dollars_source,
            )
        });

        Self { sats, bitcoin, dollars }
    }
}
