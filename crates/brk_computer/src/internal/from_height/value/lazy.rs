//! Lazy value wrapper for ValueFromHeight - all transforms are lazy.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::UnaryTransform;

use crate::internal::{LazyValue, LazyValueHeightDerived, ValueFromHeight};

/// Lazy value wrapper with height + all derived last transforms from ValueFromHeight.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyValueFromHeight {
    #[traversable(flatten)]
    pub height: LazyValue<Height>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<LazyValueHeightDerived>,
}

impl LazyValueFromHeight {
    pub(crate) fn from_block_source<
        SatsTransform,
        BitcoinTransform,
        CentsTransform,
        DollarsTransform,
    >(
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
        let height = LazyValue::from_block_source::<
            SatsTransform,
            BitcoinTransform,
            CentsTransform,
            DollarsTransform,
        >(name, source, version);

        let rest = LazyValueHeightDerived::from_block_source::<
            SatsTransform,
            BitcoinTransform,
            CentsTransform,
            DollarsTransform,
        >(name, source, version);

        Self {
            height,
            rest: Box::new(rest),
        }
    }
}
