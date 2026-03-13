use brk_traversable::Traversable;
use brk_types::{TxIndex, Version};
use schemars::JsonSchema;
use vecdb::{LazyVecFrom2, UnaryTransform};

use crate::internal::{ComputedVecValue, LazyTxDerivedDistribution, TxDerivedDistribution};

/// Like `LazyPerTxDistribution` but with a lazy-derived distribution
/// (transformed from another type's distribution rather than eagerly computed).
#[derive(Clone, Traversable)]
pub struct LazyPerTxDistributionTransformed<T, S1, S2, DSource>
where
    T: ComputedVecValue + JsonSchema,
    S1: ComputedVecValue,
    S2: ComputedVecValue,
    DSource: ComputedVecValue,
{
    pub txindex: LazyVecFrom2<TxIndex, T, TxIndex, S1, TxIndex, S2>,
    #[traversable(flatten)]
    pub distribution: LazyTxDerivedDistribution<T, DSource>,
}

impl<T, S1, S2, DSource> LazyPerTxDistributionTransformed<T, S1, S2, DSource>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1: ComputedVecValue + JsonSchema,
    S2: ComputedVecValue + JsonSchema,
    DSource: ComputedVecValue + JsonSchema,
{
    pub(crate) fn new<F: UnaryTransform<DSource, T>>(
        name: &str,
        version: Version,
        txindex: LazyVecFrom2<TxIndex, T, TxIndex, S1, TxIndex, S2>,
        source_distribution: &TxDerivedDistribution<DSource>,
    ) -> Self {
        let distribution =
            LazyTxDerivedDistribution::from_tx_derived::<F>(name, version, source_distribution);
        Self {
            txindex,
            distribution,
        }
    }
}
