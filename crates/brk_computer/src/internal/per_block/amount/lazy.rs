//! Lazy value wrapper for AmountPerBlock - all transforms are lazy.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::UnaryTransform;

use crate::internal::{
    AmountPerBlock, Identity, LazyAmount, LazyAmountDerivedResolutions, SatsToBitcoin,
};

/// Lazy value wrapper with height + all derived last transforms from AmountPerBlock.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyAmountPerBlock {
    #[traversable(flatten)]
    pub height: LazyAmount<Height>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub resolutions: Box<LazyAmountDerivedResolutions>,
}

impl LazyAmountPerBlock {
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
        let height = LazyAmount::from_block_source::<
            SatsTransform,
            BitcoinTransform,
            CentsTransform,
            DollarsTransform,
        >(name, source, version);

        let resolutions = LazyAmountDerivedResolutions::from_block_source::<
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

    pub(crate) fn identity(name: &str, source: &AmountPerBlock, version: Version) -> Self {
        Self::from_block_source::<Identity<Sats>, SatsToBitcoin, Identity<Cents>, Identity<Dollars>>(
            name, source, version,
        )
    }
}
