//! Fully lazy value type for Height indexing.

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{IterableCloneableVec, LazyVecFrom1, UnaryTransform};

use crate::internal::{SatsToBitcoin, ValueFromHeightLast};

const VERSION: Version = Version::ZERO;

/// Fully lazy value type at height level.
///
/// All fields are lazy transforms from existing sources - no storage.
#[derive(Clone, Traversable)]
pub struct LazyValueHeight {
    pub sats: LazyVecFrom1<Height, Sats, Height, Sats>,
    pub bitcoin: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub dollars: Option<LazyVecFrom1<Height, Dollars, Height, Dollars>>,
}

impl LazyValueHeight {
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

        let sats =
            LazyVecFrom1::transformed::<SatsTransform>(name, v, source.sats.height.boxed_clone());

        let bitcoin = LazyVecFrom1::transformed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            source.sats.height.boxed_clone(),
        );

        let dollars = source.dollars.as_ref().map(|d| {
            LazyVecFrom1::transformed::<DollarsTransform>(
                &format!("{name}_usd"),
                v,
                d.height.boxed_clone(),
            )
        });

        Self { sats, bitcoin, dollars }
    }
}
