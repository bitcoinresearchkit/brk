use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use derive_more::Deref;
use vecdb::{CachedVec, ReadableCloneableVec, ReadableVec};

use bitview_compute::{
    CachedWindowStartVec, LazyIndexedVec, LazyPerBlockCumulativeRolling, Windows,
};

#[derive(Clone, Deref, Traversable)]
pub struct CachedSpendableOutputCount {
    #[deref]
    #[traversable(flatten)]
    pub views: LazyPerBlockCumulativeRolling<StoredU64>,
    #[traversable(skip)]
    cumulative: CachedVec<LazyIndexedVec<Height, StoredU64, StoredU64, StoredU64>>,
}

impl CachedSpendableOutputCount {
    pub fn new(
        version: Version,
        op_return_count: &impl ReadableCloneableVec<Height, StoredU64>,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Self {
        let cumulative = CachedVec::wrap(LazyIndexedVec::new(
            "spendable_output_count_cumulative",
            version,
            op_return_count,
            &mappings.output_count(),
            |_, op_return, total| total - op_return,
        ));
        let views = LazyPerBlockCumulativeRolling::from_cumulative_source(
            "spendable_output_count",
            version,
            &cumulative,
            cached_starts,
            mappings,
        );

        Self { views, cumulative }
    }

    pub fn cumulative_source(
        &self,
    ) -> &(impl ReadableVec<Height, StoredU64> + Clone + 'static + use<>) {
        &self.cumulative
    }

    pub fn invalidate(&self) {
        self.cumulative.invalidate();
    }
}
