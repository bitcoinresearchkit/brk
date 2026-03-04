//! Lazy value type for Last pattern across all height-derived indexes.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Sats, Version};
use vecdb::UnaryTransform;

use crate::internal::{LazyHeightDerived, ValueFromHeight};

#[derive(Clone, Traversable)]
pub struct LazyValueHeightDerived {
    pub sats: LazyHeightDerived<Sats, Sats>,
    pub btc: LazyHeightDerived<Bitcoin, Sats>,
    pub cents: LazyHeightDerived<Cents, Cents>,
    pub usd: LazyHeightDerived<Dollars, Dollars>,
}

impl LazyValueHeightDerived {
    pub(crate) fn from_block_source<SatsTransform, BitcoinTransform, CentsTransform, DollarsTransform>(
        name: &str,
        source: &ValueFromHeight,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        CentsTransform: UnaryTransform<Cents, Cents>,
        DollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let sats = LazyHeightDerived::from_derived_computed::<SatsTransform>(
            name,
            version,
            &source.sats.rest,
        );

        let btc = LazyHeightDerived::from_derived_computed::<BitcoinTransform>(
            &format!("{name}_btc"),
            version,
            &source.sats.rest,
        );

        let cents = LazyHeightDerived::from_derived_computed::<CentsTransform>(
            &format!("{name}_cents"),
            version,
            &source.cents.rest,
        );

        let usd = LazyHeightDerived::from_lazy::<DollarsTransform, Cents>(
            &format!("{name}_usd"),
            version,
            &source.usd.rest,
        );

        Self { sats, btc, cents, usd }
    }
}
