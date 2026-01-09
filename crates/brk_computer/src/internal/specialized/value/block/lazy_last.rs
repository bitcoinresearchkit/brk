//! Lazy value wrapper for ValueBlockLast - all transforms are lazy.

use brk_traversable::Traversable;
use brk_types::{Dollars, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::UnaryTransform;

use super::LazyValueBlockHeight;
use crate::internal::{LazyValueDateLast, SatsToBitcoin, ValueBlockLast};

const VERSION: Version = Version::ZERO;

/// Lazy value wrapper with height + date last transforms from ValueBlockLast.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyLastBlockValue {
    #[traversable(flatten)]
    pub height: LazyValueBlockHeight,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: LazyValueDateLast,
}

impl LazyLastBlockValue {
    pub fn from_block_source<SatsTransform, DollarsTransform>(
        name: &str,
        source: &ValueBlockLast,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        DollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let v = version + VERSION;

        let height =
            LazyValueBlockHeight::from_block_source::<SatsTransform, DollarsTransform>(name, source, v);

        let dates = LazyValueDateLast::from_block_source::<SatsTransform, SatsToBitcoin, DollarsTransform>(
            name, source, v,
        );

        Self { height, dates }
    }
}
