//! Lazy value wrapper for ValuePerBlock - all transforms are lazy.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::UnaryTransform;

use crate::internal::{
    Identity, LazyValue, LazyValueDerivedResolutions, SatsToBitcoin, ValuePerBlock,
};

/// Lazy value wrapper with height + all derived last transforms from ValuePerBlock.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyValuePerBlock {
    #[traversable(flatten)]
    pub height: LazyValue<Height>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub resolutions: Box<LazyValueDerivedResolutions>,
}

impl LazyValuePerBlock {
    pub(crate) fn from_block_source<
        SatsTransform,
        BitcoinTransform,
        CentsTransform,
        DollarsTransform,
    >(
        name: &str,
        source: &ValuePerBlock,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        CentsTransform: UnaryTransform<Cents, Cents>,
        DollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let height = LazyValue::from_block_source::<
            SatsTransform,
            BitcoinTransform,
            CentsTransform,
            DollarsTransform,
        >(name, source, version);

        let resolutions = LazyValueDerivedResolutions::from_block_source::<
            SatsTransform,
            BitcoinTransform,
            CentsTransform,
            DollarsTransform,
        >(name, source, version);

        Self {
            height,
            resolutions: Box::new(resolutions),
        }
    }

    pub(crate) fn identity(name: &str, source: &ValuePerBlock, version: Version) -> Self {
        Self::from_block_source::<Identity<Sats>, SatsToBitcoin, Identity<Cents>, Identity<Dollars>>(
            name, source, version,
        )
    }
}
