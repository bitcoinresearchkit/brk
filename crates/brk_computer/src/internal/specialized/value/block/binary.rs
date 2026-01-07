use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec};

use crate::internal::{BinaryBlockSumCum, DerivedValueBlockSumCum, ValueBlockSumCum};

/// Lazy value vecs computed from two ValueBlockSumCum sources via binary transforms.
/// Used for computing coinbase = subsidy + fee.
#[derive(Clone, Traversable)]
pub struct ValueBinaryBlock {
    pub sats: BinaryBlockSumCum<Sats, Sats, Sats>,
    pub bitcoin: BinaryBlockSumCum<Bitcoin, Sats, Sats>,
    pub dollars: Option<BinaryBlockSumCum<Dollars, Dollars, Dollars>>,
}

impl ValueBinaryBlock {
    pub fn from_computed<SatsF, BitcoinF, DollarsF>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, Sats>,
        height_source2: IterableBoxedVec<Height, Sats>,
        source1: &ValueBlockSumCum,
        source2: &ValueBlockSumCum,
    ) -> Self
    where
        SatsF: BinaryTransform<Sats, Sats, Sats>,
        BitcoinF: BinaryTransform<Sats, Sats, Bitcoin>,
        DollarsF: BinaryTransform<Dollars, Dollars, Dollars>,
    {
        let sats = BinaryBlockSumCum::from_computed::<SatsF>(
            name,
            version,
            height_source1.boxed_clone(),
            height_source2.boxed_clone(),
            &source1.sats,
            &source2.sats,
        );

        let bitcoin = BinaryBlockSumCum::from_computed::<BitcoinF>(
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
                BinaryBlockSumCum::from_computed::<DollarsF>(
                    &format!("{name}_usd"),
                    version,
                    d1.height.boxed_clone(),
                    d2.height.boxed_clone(),
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

    pub fn from_derived<SatsF, BitcoinF, DollarsF>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, Sats>,
        height_source2: IterableBoxedVec<Height, Sats>,
        source1: &DerivedValueBlockSumCum,
        source2: &DerivedValueBlockSumCum,
    ) -> Self
    where
        SatsF: BinaryTransform<Sats, Sats, Sats>,
        BitcoinF: BinaryTransform<Sats, Sats, Bitcoin>,
        DollarsF: BinaryTransform<Dollars, Dollars, Dollars>,
    {
        let sats = BinaryBlockSumCum::from_derived::<SatsF>(
            name,
            version,
            height_source1.boxed_clone(),
            height_source2.boxed_clone(),
            &source1.sats,
            &source2.sats,
        );

        let bitcoin = BinaryBlockSumCum::from_derived::<BitcoinF>(
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
                BinaryBlockSumCum::from_derived::<DollarsF>(
                    &format!("{name}_usd"),
                    version,
                    d1.height_cumulative.0.boxed_clone(),
                    d2.height_cumulative.0.boxed_clone(),
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
}
