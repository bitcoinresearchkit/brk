//! Lazy binary value wrapper combining height (with price) + difficultyepoch + date last transforms.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{BinaryTransform, IterableCloneableVec, UnaryTransform};

use super::LazyFromHeightValue;
use crate::internal::{LazyTransformedValueDifficultyEpoch, LazyValueFromDateLast};
use crate::{internal::ValueFromHeightLast, price};

const VERSION: Version = Version::ZERO;

/// Lazy binary value wrapper with height (using price binary transform) + difficultyepoch + date last transforms.
///
/// Use this when the height-level dollars need a binary transform (e.g., price × sats)
/// rather than a unary transform from existing dollars.
///
/// No merge at this level - denominations (sats, bitcoin, dollars) stay as separate branches.
/// Each inner field has merge which combines indexes within each denomination.
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct LazyBinaryValueFromHeightLast {
    #[traversable(flatten)]
    pub height: LazyFromHeightValue,
    #[traversable(flatten)]
    pub difficultyepoch: LazyTransformedValueDifficultyEpoch,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: LazyValueFromDateLast,
}

impl LazyBinaryValueFromHeightLast {
    pub fn from_block_source<SatsTransform, BitcoinTransform, HeightDollarsTransform, DateDollarsTransform>(
        name: &str,
        source: &ValueFromHeightLast,
        price: Option<&price::Vecs>,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        HeightDollarsTransform: BinaryTransform<Close<Dollars>, Sats, Dollars>,
        DateDollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let v = version + VERSION;

        let price_source = price.map(|p| p.usd.split.close.height.boxed_clone());

        let height = LazyFromHeightValue::from_sources::<SatsTransform, BitcoinTransform, HeightDollarsTransform>(
            name,
            source.sats.height.boxed_clone(),
            price_source,
            v,
        );

        let difficultyepoch = LazyTransformedValueDifficultyEpoch::from_block_source::<
            SatsTransform,
            BitcoinTransform,
            HeightDollarsTransform,
        >(name, source, price, v);

        let dates = LazyValueFromDateLast::from_block_source::<SatsTransform, BitcoinTransform, DateDollarsTransform>(
            name, source, v,
        );

        Self { height, difficultyepoch, dates }
    }
}
