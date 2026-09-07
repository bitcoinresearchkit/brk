use bitview_traversable::Traversable;
use brk_types::{BasisPoints32, Height, StoredF32, Version};
use vecdb::{ReadableVec, TypedVec};

use crate::{FixedToRatio, Identity, IndexSources, LazyPerBlock};

/// Basis-point and decimal views derived lazily from one source.
#[derive(Clone, Traversable)]
pub struct LazyBasisPointsPerBlock {
    /// Unitless ratio in basis points; 10,000 represents 1.0. Floored to whole
    /// basis points, with u32::MAX reserved for undefined values.
    pub bps: LazyPerBlock<BasisPoints32>,
    /// Unitless decimal ratio derived as basis points divided by 10,000.
    pub ratio: LazyPerBlock<StoredF32, BasisPoints32>,
}

impl LazyBasisPointsPerBlock {
    pub fn from_height_source<V>(
        name: &str,
        version: Version,
        source: V,
        indexes: &IndexSources,
    ) -> Self
    where
        V: TypedVec<I = Height, T = BasisPoints32>
            + ReadableVec<Height, BasisPoints32>
            + Clone
            + 'static,
    {
        let bps = LazyPerBlock::from_height_source::<Identity<BasisPoints32>>(
            &format!("{name}_bps"),
            version,
            source,
            indexes,
        );
        let ratio = LazyPerBlock::from_lazy::<FixedToRatio, BasisPoints32>(name, version, &bps);
        Self { bps, ratio }
    }
}
