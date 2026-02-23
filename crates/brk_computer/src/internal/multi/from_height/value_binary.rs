use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Sats, Version};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, ReadableCloneableVec};

use crate::internal::{ComputedVecValue, LazyBinaryFromHeightSumCum, LazyValueFromHeightSumCum};

/// Lazy value vecs computed from two ValueFromHeightSumCum sources via binary transforms.
/// Used for computing coinbase = subsidy + fee.
#[derive(Clone, Traversable)]
pub struct ValueBinaryFromHeight {
    pub sats: LazyBinaryFromHeightSumCum<Sats, Sats, Sats>,
    pub btc: LazyBinaryFromHeightSumCum<Bitcoin, Sats, Sats>,
    pub usd: LazyBinaryFromHeightSumCum<Dollars, Dollars, Dollars>,
}

impl ValueBinaryFromHeight {
    pub(crate) fn from_lazy<SatsF, BitcoinF, DollarsF, S1T, S2T>(
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
            source1.sats.height.read_only_boxed_clone(),
            source2.sats.height.read_only_boxed_clone(),
            &source1.sats.rest,
            &source2.sats.rest,
        );

        let btc = LazyBinaryFromHeightSumCum::from_derived::<BitcoinF>(
            &format!("{name}_btc"),
            version,
            source1.sats.height.read_only_boxed_clone(),
            source2.sats.height.read_only_boxed_clone(),
            &source1.sats.rest,
            &source2.sats.rest,
        );

        let usd = LazyBinaryFromHeightSumCum::from_derived::<DollarsF>(
            &format!("{name}_usd"),
            version,
            source1.usd.height.read_only_boxed_clone(),
            source2.usd.height.read_only_boxed_clone(),
            &source1.usd.rest,
            &source2.usd.rest,
        );

        Self {
            sats,
            btc,
            usd,
        }
    }
}
