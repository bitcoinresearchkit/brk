use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::UnaryTransform;

use crate::internal::{ComputedVecValue, LazyDistribution, TxDerivedDistribution};

#[derive(Clone, Traversable)]
pub struct LazyBlockRollingDistribution<T, S1T>
where
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
{
    pub _6b: LazyDistribution<Height, T, S1T>,
}

/// Lazy analog of `TxDerivedDistribution<T>`: per-block + 6-block rolling,
/// each derived by transforming the corresponding source distribution.
#[derive(Clone, Traversable)]
pub struct LazyTxDerivedDistribution<T, S1T>
where
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
{
    pub block: LazyDistribution<Height, T, S1T>,
    #[traversable(flatten)]
    pub rolling: LazyBlockRollingDistribution<T, S1T>,
}

impl<T, S1T> LazyTxDerivedDistribution<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn from_tx_derived<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: &TxDerivedDistribution<S1T>,
    ) -> Self {
        let block = LazyDistribution::from_distribution::<F>(name, version, &source.block);
        let rolling = LazyBlockRollingDistribution {
            _6b: LazyDistribution::from_distribution::<F>(
                &format!("{name}_6b"),
                version,
                &source.rolling._6b,
            ),
        };
        Self { block, rolling }
    }
}
