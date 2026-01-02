use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{BinaryTransform, IterableBoxedVec, IterableCloneableVec};

use crate::internal::{ComputedValueVecsFromHeight, LazyVecsFrom2FromHeight};

/// Lazy value vecs computed from two `ComputedValueVecsFromHeight` sources via binary transforms.
/// Used for computing coinbase = subsidy + fee.
#[derive(Clone, Traversable)]
pub struct LazyValueVecsFrom2FromHeight {
    pub sats: LazyVecsFrom2FromHeight<Sats, Sats, Sats>,
    pub bitcoin: LazyVecsFrom2FromHeight<Bitcoin, Sats, Sats>,
    pub dollars: Option<LazyVecsFrom2FromHeight<Dollars, Dollars, Dollars>>,
}

impl LazyValueVecsFrom2FromHeight {
    pub fn from_computed<SatsF, BitcoinF, DollarsF>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, Sats>,
        height_source2: IterableBoxedVec<Height, Sats>,
        source1: &ComputedValueVecsFromHeight,
        source2: &ComputedValueVecsFromHeight,
    ) -> Self
    where
        SatsF: BinaryTransform<Sats, Sats, Sats>,
        BitcoinF: BinaryTransform<Sats, Sats, Bitcoin>,
        DollarsF: BinaryTransform<Dollars, Dollars, Dollars>,
    {
        let sats = LazyVecsFrom2FromHeight::from_computed::<SatsF>(
            name,
            version,
            height_source1.boxed_clone(),
            height_source2.boxed_clone(),
            &source1.sats,
            &source2.sats,
        );

        let bitcoin = LazyVecsFrom2FromHeight::from_computed::<BitcoinF>(
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
                LazyVecsFrom2FromHeight::from_computed::<DollarsF>(
                    &format!("{name}_usd"),
                    version,
                    d1.height.as_ref().unwrap().boxed_clone(),
                    d2.height.as_ref().unwrap().boxed_clone(),
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
