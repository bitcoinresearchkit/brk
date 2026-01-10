//! Lazy value wrapper for ValueFromHeightLast - all transforms are lazy.

use brk_traversable::Traversable;
use brk_types::{Dollars, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::UnaryTransform;

use crate::internal::{LazyValueFromDateLast, LazyValueHeight, SatsToBitcoin, ValueFromHeightLast};

const VERSION: Version = Version::ZERO;

/// Lazy value wrapper with height + date last transforms from ValueFromHeightLast.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyValueFromHeightLast {
    #[traversable(flatten)]
    pub height: LazyValueHeight,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: LazyValueFromDateLast,
}

impl LazyValueFromHeightLast {
    pub fn from_block_source<SatsTransform, DollarsTransform>(
        name: &str,
        source: &ValueFromHeightLast,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        DollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let v = version + VERSION;

        let height =
            LazyValueHeight::from_block_source::<SatsTransform, DollarsTransform>(name, source, v);

        let dates = LazyValueFromDateLast::from_block_source::<SatsTransform, SatsToBitcoin, DollarsTransform>(
            name, source, v,
        );

        Self { height, dates }
    }
}
