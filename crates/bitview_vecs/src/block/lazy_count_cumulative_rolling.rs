use bitview_collections::Windows;
use bitview_transforms::StoredU16ToStoredU64;
use bitview_traversable::Traversable;
use brk_types::{Height, StoredU16, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Ident, LazyVec, ReadableCloneableVec};

use crate::{CumulativeCountVec, IndexSources, LazyPerBlock, RollingTotals};

#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct LazyCountPerBlockCumulativeRolling {
    /// Value for the represented block. At time-period indexes, the value is
    /// taken from the period's final block.
    pub block: LazyVec<Height, StoredU64, Height, StoredU16>,
    /// Cumulative value through the represented block. At time-period indexes,
    /// the value is taken at the period's final block.
    pub cumulative: LazyPerBlock<StoredU64>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rolling: RollingTotals<StoredU64>,
    #[traversable(skip)]
    cumulative_source: CumulativeCountVec,
}

impl LazyCountPerBlockCumulativeRolling {
    pub fn from_height_source(
        name: &str,
        version: Version,
        source: &(impl ReadableCloneableVec<Height, StoredU16> + ?Sized),
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Self {
        let cumulative_source = CumulativeCountVec::new(source);
        let block = LazyVec::transformed::<StoredU16ToStoredU64>(
            name,
            version,
            source.read_only_boxed_clone(),
        );
        let cumulative = LazyPerBlock::from_height_source::<Ident>(
            &format!("{name}_cumulative"),
            version,
            &cumulative_source,
            indexes,
        );
        let rolling = RollingTotals::new(name, version, &cumulative_source, window_starts, indexes);

        Self {
            block,
            cumulative,
            rolling,
            cumulative_source,
        }
    }

    #[inline(always)]
    pub fn cumulative_source(&self) -> CumulativeCountVec {
        self.cumulative_source.clone()
    }
}
