//! Lazy value wrapper for ValueFromHeightLast - all transforms are lazy.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::UnaryTransform;

use crate::internal::{LazyValueHeight, LazyValueHeightDerivedLast, ValueFromHeightLast};

const VERSION: Version = Version::ZERO;

/// Lazy value wrapper with height + all derived last transforms from ValueFromHeightLast.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyValueFromHeightLast {
    #[traversable(flatten)]
    pub height: LazyValueHeight,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<LazyValueHeightDerivedLast>,
}

impl LazyValueFromHeightLast {
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

        let height =
            LazyValueHeight::from_block_source::<SatsTransform, BitcoinTransform, DollarsTransform>(name, source, v);

        let rest =
            LazyValueHeightDerivedLast::from_block_source::<SatsTransform, BitcoinTransform, DollarsTransform>(
                name, source, v,
            );

        Self { height, rest: Box::new(rest) }
    }
}
