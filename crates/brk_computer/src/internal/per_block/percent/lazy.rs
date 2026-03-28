use brk_traversable::Traversable;
use brk_types::{StoredF32, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{ReadableCloneableVec, UnaryTransform};

use crate::internal::{BpsType, LazyPerBlock, Percent, PercentPerBlock};

/// Fully lazy variant of `PercentPerBlock` — no stored vecs.
///
/// BPS values are lazily derived from a source `PercentPerBlock` via a unary transform,
/// and ratio/percent float views are chained from the lazy BPS.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyPercentPerBlock<B: BpsType>(
    pub Percent<LazyPerBlock<B, B>, LazyPerBlock<StoredF32, B>>,
);

impl<B: BpsType> LazyPercentPerBlock<B> {
    /// Create from a stored `PercentPerBlock` source via a BPS-to-BPS unary transform.
    pub(crate) fn from_percent<F: UnaryTransform<B, B>>(
        name: &str,
        version: Version,
        source: &PercentPerBlock<B>,
    ) -> Self {
        let bps = LazyPerBlock::from_computed::<F>(
            &format!("{name}_bps"),
            version,
            source.bps.height.read_only_boxed_clone(),
            &source.bps,
        );

        let ratio =
            LazyPerBlock::from_lazy::<B::ToRatio, B>(&format!("{name}_ratio"), version, &bps);

        let percent = LazyPerBlock::from_lazy::<B::ToPercent, B>(name, version, &bps);

        Self(Percent {
            bps,
            ratio,
            percent,
        })
    }
}
