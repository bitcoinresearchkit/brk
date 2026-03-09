use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Sats, Version};
use vecdb::UnaryTransform;

use crate::internal::{AmountPerBlock, DerivedResolutions};

#[derive(Clone, Traversable)]
pub struct LazyAmountDerivedResolutions {
    pub sats: DerivedResolutions<Sats, Sats>,
    pub btc: DerivedResolutions<Bitcoin, Sats>,
    pub cents: DerivedResolutions<Cents, Cents>,
    pub usd: DerivedResolutions<Dollars, Dollars>,
}

impl LazyAmountDerivedResolutions {
    pub(crate) fn from_block_source<
        SatsTransform,
        BitcoinTransform,
        CentsTransform,
        DollarsTransform,
    >(
        name: &str,
        source: &AmountPerBlock,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        CentsTransform: UnaryTransform<Cents, Cents>,
        DollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let sats = DerivedResolutions::from_derived_computed::<SatsTransform>(
            &format!("{name}_sats"),
            version,
            &source.sats.resolutions,
        );

        let btc = DerivedResolutions::from_derived_computed::<BitcoinTransform>(
            name,
            version,
            &source.sats.resolutions,
        );

        let cents = DerivedResolutions::from_derived_computed::<CentsTransform>(
            &format!("{name}_cents"),
            version,
            &source.cents.resolutions,
        );

        let usd = DerivedResolutions::from_lazy::<DollarsTransform, Cents>(
            &format!("{name}_usd"),
            version,
            &source.usd.resolutions,
        );

        Self {
            sats,
            btc,
            cents,
            usd,
        }
    }
}
