//! Lazy binary value wrapper combining height (with price) + all derived last transforms.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{BinaryTransform, ReadableCloneableVec, UnaryTransform};

use super::LazyFromHeightValue;
use crate::internal::{LazyValueHeightDerivedLast, ValueFromHeightLast};
use crate::prices;

const VERSION: Version = Version::ZERO;

/// Lazy binary value wrapper with height (using price binary transform) + all derived last transforms.
///
/// Use this when the height-level dollars need a binary transform (e.g., price * sats)
/// rather than a unary transform from existing dollars.
///
/// All coarser-than-height periods (minute1 through difficultyepoch) use unary transforms
/// on the pre-computed values from the source.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryValueFromHeightLast {
    #[traversable(flatten)]
    pub height: LazyFromHeightValue,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<LazyValueHeightDerivedLast>,
}

impl LazyBinaryValueFromHeightLast {
    pub(crate) fn from_block_source<
        SatsTransform,
        BitcoinTransform,
        HeightDollarsTransform,
        DateDollarsTransform,
    >(
        name: &str,
        source: &ValueFromHeightLast,
        prices: &prices::Vecs,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        HeightDollarsTransform: BinaryTransform<Dollars, Sats, Dollars>,
        DateDollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let v = version + VERSION;

        let price_source = prices.usd.price.read_only_boxed_clone();

        let height = LazyFromHeightValue::from_sources::<
            SatsTransform,
            BitcoinTransform,
            HeightDollarsTransform,
        >(name, source.sats.height.read_only_boxed_clone(), price_source, v);

        let rest =
            LazyValueHeightDerivedLast::from_block_source::<SatsTransform, BitcoinTransform, DateDollarsTransform>(
                name, source, v,
            );

        Self { height, rest: Box::new(rest) }
    }
}
