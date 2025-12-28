use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, Version};
use vecdb::{BinaryTransform, IterableBoxedVec, LazyVecFrom1, LazyVecFrom2, UnaryTransform};

const VERSION: Version = Version::ZERO;

/// Fully lazy version of `ComputedHeightValueVecs` where all fields are lazy transforms.
/// Used for computed values like supply_half where sources are stored sats and dollars vecs.
#[derive(Clone, Traversable)]
pub struct LazyHeightValueVecs {
    pub sats: LazyVecFrom1<Height, Sats, Height, Sats>,
    pub bitcoin: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub dollars: Option<LazyVecFrom2<Height, Dollars, Height, Close<Dollars>, Height, Sats>>,
}

impl LazyHeightValueVecs {
    /// Create lazy height value vecs from sats and price sources via transforms.
    ///
    /// - `SatsTransform`: Transform from Sats -> Sats (e.g., HalveSats)
    /// - `BitcoinTransform`: Transform from Sats -> Bitcoin (e.g., HalveSatsToBitcoin)
    /// - `DollarsTransform`: Binary transform from (Close<Dollars>, Sats) -> Dollars (e.g., HalfClosePriceTimesSats)
    pub fn from_sources<SatsTransform, BitcoinTransform, DollarsTransform>(
        name: &str,
        sats_source: IterableBoxedVec<Height, Sats>,
        price_source: Option<IterableBoxedVec<Height, Close<Dollars>>>,
        version: Version,
    ) -> Self
    where
        SatsTransform: UnaryTransform<Sats, Sats>,
        BitcoinTransform: UnaryTransform<Sats, Bitcoin>,
        DollarsTransform: BinaryTransform<Close<Dollars>, Sats, Dollars>,
    {
        let v = version + VERSION;

        let sats = LazyVecFrom1::transformed::<SatsTransform>(
            name,
            v + Version::ZERO,
            sats_source.clone(),
        );

        let bitcoin = LazyVecFrom1::transformed::<BitcoinTransform>(
            &format!("{name}_btc"),
            v + Version::ZERO,
            sats_source.clone(),
        );

        // dollars is binary transform: price × sats (with optional halving etc)
        let dollars = price_source.map(|price| {
            LazyVecFrom2::transformed::<DollarsTransform>(
                &format!("{name}_usd"),
                v + Version::ZERO,
                price,
                sats_source,
            )
        });

        Self {
            sats,
            bitcoin,
            dollars,
        }
    }
}
