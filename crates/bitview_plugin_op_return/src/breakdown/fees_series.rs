use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use brk_types::{Height, PartsPerMillion32, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{ColumnId, ReadableCloneableVec};

use bitview_compute::{
    CachedWindowStartVec, LazyColumnPerBlockCumulativeRolling, LazyPercentCumulativeRolling,
    RatioSats, Windows,
};

#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct FeesSeries<C: ColumnId> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub fees: LazyColumnPerBlockCumulativeRolling<Sats, C>,
    /// Fees of transactions in a breakdown bucket divided by all transaction
    /// fees over the same cumulative or trailing window.
    pub fee_share: LazyPercentCumulativeRolling<PartsPerMillion32>,
}

impl<C: ColumnId> FeesSeries<C> {
    pub fn new(
        prefix: &str,
        version: Version,
        fees: LazyColumnPerBlockCumulativeRolling<Sats, C>,
        chain_fees: &impl ReadableCloneableVec<Height, Sats>,
        cached_starts: &Windows<&CachedWindowStartVec>,
        mappings: &MappingsVecs,
    ) -> Self {
        let fee_share = LazyPercentCumulativeRolling::from_cumulative_ratio::<
            Sats,
            Sats,
            RatioSats<PartsPerMillion32>,
        >(
            &format!("{prefix}_fee_share"),
            version,
            fees.cumulative.resolutions.height_source(),
            chain_fees,
            cached_starts,
            mappings,
        );

        Self { fees, fee_share }
    }
}
