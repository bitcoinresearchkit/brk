//! Fully lazy value type for Height indexing.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{ReadableCloneableVec, LazyVecFrom1, UnaryTransform};

use crate::internal::ValueFromHeightLast;

const VERSION: Version = Version::ZERO;

/// Fully lazy value type at height level.
///
/// All fields are lazy transforms from existing sources - no storage.
#[derive(Clone, Traversable)]
pub struct LazyValueFromHeight {
    pub sats: LazyVecFrom1<Height, Sats, Height, Sats>,
    pub btc: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub usd: LazyVecFrom1<Height, Dollars, Height, Dollars>,
}

impl LazyValueFromHeight {
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

        let sats =
            LazyVecFrom1::transformed::<SatsTransform>(name, v, source.sats.height.read_only_boxed_clone());

        let btc = LazyVecFrom1::transformed::<BitcoinTransform>(
            &format!("{name}_btc"),
            v,
            source.sats.height.read_only_boxed_clone(),
        );

        let usd = LazyVecFrom1::transformed::<DollarsTransform>(
            &format!("{name}_usd"),
            v,
            source.usd.height.read_only_boxed_clone(),
        );

        Self { sats, btc, usd }
    }
}
