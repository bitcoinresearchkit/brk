//! Fully lazy value types for DifficultyEpoch indexing.
//!
//! Two variants exist for different source patterns:
//! - `LazyValueDifficultyEpochFromHeight`: For sources without dollars (computes from price × sats)
//! - `LazyTransformedValueDifficultyEpoch`: For transformed views (e.g., halved supply)

use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, DifficultyEpoch, Dollars, Height, Sats, Version};
use vecdb::{
    BinaryTransform, IterableBoxedVec, IterableCloneableVec, LazyVecFrom1, LazyVecFrom2,
    UnaryTransform,
};

use crate::internal::{ClosePriceTimesSats, LazyLast, SatsToBitcoin, ValueFromHeightLast};
use crate::price;

const VERSION: Version = Version::ZERO;

/// Lazy value type at difficultyepoch level - computed from height sats + price.
///
/// Use this when the source only has height-indexed sats (e.g., ValueFromHeightAndDateLast).
/// Dollars are computed via price × sats binary transform.
#[derive(Clone, Traversable)]
pub struct LazyValueDifficultyEpoch {
    pub sats: LazyLast<DifficultyEpoch, Sats, Height, DifficultyEpoch>,
    pub bitcoin: LazyVecFrom1<DifficultyEpoch, Bitcoin, DifficultyEpoch, Sats>,
    pub dollars: Option<
        LazyVecFrom2<
            DifficultyEpoch,
            Dollars,
            DifficultyEpoch,
            Close<Dollars>,
            DifficultyEpoch,
            Sats,
        >,
    >,
}

impl LazyValueDifficultyEpoch {
    /// Create from height sats source and difficultyepoch identity.
    /// Bitcoin is derived from sats. Dollars are computed from price × sats.
    pub fn from_height_source(
        name: &str,
        height_sats: IterableBoxedVec<Height, Sats>,
        difficultyepoch_identity: IterableBoxedVec<DifficultyEpoch, DifficultyEpoch>,
        price: Option<&price::Vecs>,
        version: Version,
    ) -> Self {
        let v = version + VERSION;

        let sats = LazyLast::from_source(name, v, height_sats, difficultyepoch_identity);

        let bitcoin = LazyVecFrom1::transformed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.boxed_clone(),
        );

        let dollars = price.map(|p| {
            LazyVecFrom2::transformed::<ClosePriceTimesSats>(
                &format!("{name}_usd"),
                v,
                p.usd.split.close.difficultyepoch.boxed_clone(),
                sats.boxed_clone(),
            )
        });

        Self {
            sats,
            bitcoin,
            dollars,
        }
    }
}

/// Lazy value type at difficultyepoch level - transformed from existing difficultyepoch sources.
///
/// Use this when creating transformed views (e.g., halved supply) from sources that
/// already have difficultyepoch aggregations. Applies transforms to the existing aggregations.
#[derive(Clone, Traversable)]
pub struct LazyTransformedValueDifficultyEpoch {
    pub sats: LazyVecFrom1<DifficultyEpoch, Sats, DifficultyEpoch, Sats>,
    pub bitcoin: LazyVecFrom1<DifficultyEpoch, Bitcoin, DifficultyEpoch, Sats>,
    pub dollars: Option<
        LazyVecFrom2<
            DifficultyEpoch,
            Dollars,
            DifficultyEpoch,
            Close<Dollars>,
            DifficultyEpoch,
            Sats,
        >,
    >,
}

impl LazyTransformedValueDifficultyEpoch {
    /// Create transformed difficultyepoch values from a ValueFromHeightLast source.
    /// SatsTransform is applied to the source's difficultyepoch sats.
    /// BitcoinTransform converts source sats to bitcoin (should combine sats transform + conversion).
    /// Dollars are computed from price × transformed sats.
    pub fn from_block_source<SatsTransform, BitcoinTransform, DollarsTransform>(
        name: &str,
        source: &ValueFromHeightLast,
        price: Option<&price::Vecs>,
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
            v,
            source.sats.rest.difficultyepoch.boxed_clone(),
        );

        let bitcoin = LazyVecFrom1::transformed::<BitcoinTransform>(
            &format!("{name}_btc"),
            v,
            source.sats.rest.difficultyepoch.boxed_clone(),
        );

        let dollars = price.map(|p| {
            LazyVecFrom2::transformed::<DollarsTransform>(
                &format!("{name}_usd"),
                v,
                p.usd.split.close.difficultyepoch.boxed_clone(),
                source.sats.rest.difficultyepoch.boxed_clone(),
            )
        });

        Self {
            sats,
            bitcoin,
            dollars,
        }
    }
}
