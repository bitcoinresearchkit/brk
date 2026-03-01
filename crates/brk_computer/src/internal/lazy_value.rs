//! Fully lazy value type.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{ReadableCloneableVec, LazyVecFrom1, UnaryTransform, VecIndex};

use crate::internal::ValueFromHeight;

const VERSION: Version = Version::ZERO;

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
    pub(crate) fn from_block_source<SatsTransform, BitcoinTransform, CentsTransform, DollarsTransform>(
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
        let v = version + VERSION;

        let sats =
            LazyVecFrom1::transformed::<SatsTransform>(name, v, source.sats.height.read_only_boxed_clone());

        let btc = LazyVecFrom1::transformed::<BitcoinTransform>(
            &format!("{name}_btc"),
            v,
            source.sats.height.read_only_boxed_clone(),
        );

        let cents = LazyVecFrom1::transformed::<CentsTransform>(
            &format!("{name}_cents"),
            v,
            source.cents.height.read_only_boxed_clone(),
        );

        let usd = LazyVecFrom1::transformed::<DollarsTransform>(
            &format!("{name}_usd"),
            v,
            source.usd.height.read_only_boxed_clone(),
        );

        Self { sats, btc, cents, usd }
    }
}
