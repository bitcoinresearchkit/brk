use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{LazyVecFrom1, ReadableCloneableVec, UnaryTransform, VecIndex};

use crate::internal::ValueFromHeight;

/// Fully lazy value type at height level.
///
/// All fields are lazy transforms from existing sources - no storage.
#[derive(Clone, Traversable)]
pub struct LazyValue<I: VecIndex> {
    pub sats: LazyVecFrom1<I, Sats, I, Sats>,
    pub btc: LazyVecFrom1<I, Bitcoin, I, Sats>,
    pub cents: LazyVecFrom1<I, Cents, I, Cents>,
    pub usd: LazyVecFrom1<I, Dollars, I, Dollars>,
}

impl LazyValue<Height> {
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
        let sats = LazyVecFrom1::transformed::<SatsTransform>(
            name,
            version,
            source.sats.height.read_only_boxed_clone(),
        );

        let btc = LazyVecFrom1::transformed::<BitcoinTransform>(
            &format!("{name}_btc"),
            version,
            source.sats.height.read_only_boxed_clone(),
        );

        let cents = LazyVecFrom1::transformed::<CentsTransform>(
            &format!("{name}_cents"),
            version,
            source.cents.height.read_only_boxed_clone(),
        );

        let usd = LazyVecFrom1::transformed::<DollarsTransform>(
            &format!("{name}_usd"),
            version,
            source.usd.height.read_only_boxed_clone(),
        );

        Self {
            sats,
            btc,
            cents,
            usd,
        }
    }
}
