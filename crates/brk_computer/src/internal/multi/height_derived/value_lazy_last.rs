//! Lazy value type for Last pattern across all height-derived indexes.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use vecdb::UnaryTransform;

use crate::internal::{LazyHeightDerivedLast, ValueFromHeightLast};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
pub struct LazyValueHeightDerivedLast {
    pub sats: LazyHeightDerivedLast<Sats, Sats>,
    pub btc: LazyHeightDerivedLast<Bitcoin, Sats>,
    pub usd: LazyHeightDerivedLast<Dollars, Dollars>,
}

impl LazyValueHeightDerivedLast {
    pub(crate) fn from_block_source<SatsTransform, BitcoinTransform, DollarsTransform>(
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

        let sats = LazyHeightDerivedLast::from_derived_computed::<SatsTransform>(
            name,
            v,
            &source.sats.rest,
        );

        let btc = LazyHeightDerivedLast::from_derived_computed::<BitcoinTransform>(
            &format!("{name}_btc"),
            v,
            &source.sats.rest,
        );

        let usd = LazyHeightDerivedLast::from_derived_computed::<DollarsTransform>(
            &format!("{name}_usd"),
            v,
            &source.usd.rest,
        );

        Self { sats, btc, usd }
    }
}
