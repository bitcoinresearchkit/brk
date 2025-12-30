use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use vecdb::{IterableCloneableVec, UnaryTransform};

use crate::grouped::{ComputedValueVecsFromDateIndex, LazyVecsFromDateIndex};

const VERSION: Version = Version::ZERO;

/// Fully lazy version of `ComputedValueVecsFromDateIndex` where all fields are lazy transforms.
/// Used for computed values like supply_half where sources are stored sats and dollars vecs.
#[derive(Clone, Traversable)]
pub struct LazyValueVecsFromDateIndex {
    pub sats: LazyVecsFromDateIndex<Sats, Sats>,
    pub bitcoin: LazyVecsFromDateIndex<Bitcoin, Sats>,
    pub dollars: Option<LazyVecsFromDateIndex<Dollars, Dollars>>,
}

impl LazyValueVecsFromDateIndex {
    /// Create lazy dateindex value vecs from source vecs via transforms.
    ///
    /// - `SatsTransform`: Transform from Sats -> Sats (e.g., HalveSats)
    /// - `BitcoinTransform`: Transform from Sats -> Bitcoin (e.g., HalveSatsToBitcoin)
    /// - `DollarsTransform`: Transform from Dollars -> Dollars (e.g., HalveDollars)
    pub fn from_source<SatsTransform, BitcoinTransform, DollarsTransform>(
        name: &str,
        source: &ComputedValueVecsFromDateIndex,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        DollarsTransform: UnaryTransform<Dollars, Dollars>,
    {
        let v = version + VERSION;

        let sats = LazyVecsFromDateIndex::from_computed::<SatsTransform>(
            name,
            v + Version::ZERO,
            source.sats.dateindex.as_ref().map(|v| v.boxed_clone()),
            &source.sats,
        );

        let bitcoin = LazyVecsFromDateIndex::from_computed::<BitcoinTransform>(
            &format!("{name}_btc"),
            v + Version::ZERO,
            source.sats.dateindex.as_ref().map(|v| v.boxed_clone()),
            &source.sats,
        );

        let dollars = source.dollars.as_ref().map(|dollars_source| {
            LazyVecsFromDateIndex::from_computed::<DollarsTransform>(
                &format!("{name}_usd"),
                v + Version::ZERO,
                dollars_source.dateindex.as_ref().map(|v| v.boxed_clone()),
                dollars_source,
            )
        });

        Self {
            sats,
            bitcoin,
            dollars,
        }
    }
}
