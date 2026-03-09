//! Lazy value wrapper for AmountFromHeight - all transforms are lazy.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::UnaryTransform;

use crate::internal::{AmountFromHeight, LazyAmount, LazyAmountHeightDerived};

/// Lazy value wrapper with height + all derived last transforms from AmountFromHeight.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyAmountFromHeight {
    #[traversable(flatten)]
    pub height: LazyAmount<Height>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<LazyAmountHeightDerived>,
}

impl LazyAmountFromHeight {
    pub(crate) fn from_block_source<
        SatsTransform,
        BitcoinTransform,
        CentsTransform,
        DollarsTransform,
    >(
        name: &str,
        source: &AmountFromHeight,
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

        let rest = LazyAmountHeightDerived::from_block_source::<
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
