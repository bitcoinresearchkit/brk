use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec};

use crate::internal::{
    ComputedVecValue, ValueHeightDerivedSumCum, LazyBinaryFromHeightSumCum, LazyValueFromHeightSumCum,
    ValueFromHeightSumCum,
};

/// Lazy value vecs computed from two ValueFromHeightSumCum sources via binary transforms.
/// Used for computing coinbase = subsidy + fee.
#[derive(Clone, Traversable)]
pub struct ValueBinaryFromHeight {
    pub sats: LazyBinaryFromHeightSumCum<Sats, Sats, Sats>,
    pub bitcoin: LazyBinaryFromHeightSumCum<Bitcoin, Sats, Sats>,
    pub dollars: Option<LazyBinaryFromHeightSumCum<Dollars, Dollars, Dollars>>,
}

impl ValueBinaryFromHeight {
    pub fn from_computed<SatsF, BitcoinF, DollarsF>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, Sats>,
        height_source2: IterableBoxedVec<Height, Sats>,
        source1: &ValueFromHeightSumCum,
        source2: &ValueFromHeightSumCum,
    ) -> Self
    where
        SatsF: BinaryTransform<Sats, Sats, Sats>,
        BitcoinF: BinaryTransform<Sats, Sats, Bitcoin>,
        DollarsF: BinaryTransform<Dollars, Dollars, Dollars>,
    {
        let sats = LazyBinaryFromHeightSumCum::from_computed::<SatsF>(
            name,
            version,
            height_source1.boxed_clone(),
            height_source2.boxed_clone(),
            &source1.sats,
            &source2.sats,
        );

        let bitcoin = LazyBinaryFromHeightSumCum::from_computed::<BitcoinF>(
            &format!("{name}_btc"),
            version,
            height_source1,
            height_source2,
            &source1.sats,
            &source2.sats,
        );

        // For dollars: use from_derived since the height is now lazy (LazyVecFrom2)
        // The rest (cumulative, dateindex) is still ComputedHeightDerivedSumCum
        let dollars = source1
            .dollars
            .as_ref()
            .zip(source2.dollars.as_ref())
            .map(|(d1, d2)| {
                LazyBinaryFromHeightSumCum::from_derived::<DollarsF>(
                    &format!("{name}_usd"),
                    version,
                    d1.height.boxed_clone(),
                    d2.height.boxed_clone(),
                    &d1.rest,
                    &d2.rest,
                )
            });

        Self {
            sats,
            bitcoin,
            dollars,
        }
    }

    pub fn from_derived<SatsF, BitcoinF, DollarsF>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, Sats>,
        height_source2: IterableBoxedVec<Height, Sats>,
        source1: &ValueHeightDerivedSumCum,
        source2: &ValueHeightDerivedSumCum,
    ) -> Self
    where
        SatsF: BinaryTransform<Sats, Sats, Sats>,
        BitcoinF: BinaryTransform<Sats, Sats, Bitcoin>,
        DollarsF: BinaryTransform<Dollars, Dollars, Dollars>,
    {
        let sats = LazyBinaryFromHeightSumCum::from_derived::<SatsF>(
            name,
            version,
            height_source1.boxed_clone(),
            height_source2.boxed_clone(),
            &source1.sats,
            &source2.sats,
        );

        let bitcoin = LazyBinaryFromHeightSumCum::from_derived::<BitcoinF>(
            &format!("{name}_btc"),
            version,
            height_source1,
            height_source2,
            &source1.sats,
            &source2.sats,
        );

        let dollars = source1
            .dollars
            .as_ref()
            .zip(source2.dollars.as_ref())
            .map(|(d1, d2)| {
                LazyBinaryFromHeightSumCum::from_derived::<DollarsF>(
                    &format!("{name}_usd"),
                    version,
                    d1.height_cumulative.boxed_clone(),
                    d2.height_cumulative.boxed_clone(),
                    d1,
                    d2,
                )
            });

        Self {
            sats,
            bitcoin,
            dollars,
        }
    }

    pub fn from_lazy<SatsF, BitcoinF, DollarsF, S1T, S2T>(
        name: &str,
        version: Version,
        source1: &LazyValueFromHeightSumCum<S1T, S2T>,
        source2: &LazyValueFromHeightSumCum<S1T, S2T>,
    ) -> Self
    where
        SatsF: BinaryTransform<Sats, Sats, Sats>,
        BitcoinF: BinaryTransform<Sats, Sats, Bitcoin>,
        DollarsF: BinaryTransform<Dollars, Dollars, Dollars>,
        S1T: ComputedVecValue + JsonSchema,
        S2T: ComputedVecValue + JsonSchema,
    {
        let sats = LazyBinaryFromHeightSumCum::from_derived::<SatsF>(
            name,
            version,
            source1.sats.height.boxed_clone(),
            source2.sats.height.boxed_clone(),
            &source1.sats.rest,
            &source2.sats.rest,
        );

        let bitcoin = LazyBinaryFromHeightSumCum::from_derived::<BitcoinF>(
            &format!("{name}_btc"),
            version,
            source1.sats.height.boxed_clone(),
            source2.sats.height.boxed_clone(),
            &source1.sats.rest,
            &source2.sats.rest,
        );

        let dollars = source1
            .dollars
            .as_ref()
            .zip(source2.dollars.as_ref())
            .map(|(d1, d2)| {
                LazyBinaryFromHeightSumCum::from_derived::<DollarsF>(
                    &format!("{name}_usd"),
                    version,
                    d1.height.boxed_clone(),
                    d2.height.boxed_clone(),
                    &d1.rest,
                    &d2.rest,
                )
            });

        Self {
            sats,
            bitcoin,
            dollars,
        }
    }
}
