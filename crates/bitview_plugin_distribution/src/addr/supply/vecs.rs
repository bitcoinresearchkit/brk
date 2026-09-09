use bitview_cohort::{AddrTypeId, WithAddrTypes};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{ColumnarPerBlock, LazyColumnSpotValuePerBlock, LazySpotValuePerBlock};
use brk_error::Result;
use brk_types::{Cents, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use rayon::{iter, prelude::*};
use vecdb::{
    AnyStoredVec, AnyVec, CacheBudget, CachedBoxedVec, Database, Rw, StorageMode, WritableVec,
};

use super::AddrTypeToSupply;

/// Per-addr-type running supply (sats/btc/cents/usd) with an aggregated `all`.
/// Shared across predicate-based supply categories (exposed, reused, respent).
/// Sats are pushed stateful per block; cents/usd are derived post-hoc from
/// sats × spot price.
#[derive(Deref, DerefMut, Traversable)]
pub struct AddrSupplyVecs<M: StorageMode = Rw>(
    #[traversable(flatten)]
    pub  ColumnarPerBlock<
        Sats,
        AddrTypeId,
        WithAddrTypes<LazyColumnSpotValuePerBlock<AddrTypeId>, LazySpotValuePerBlock>,
        M,
    >,
);

impl AddrSupplyVecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        mappings: &MappingsVecs,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let name = format!("{name}_addr_supply");
        Ok(Self(ColumnarPerBlock::forced_import(
            cache,
            db,
            &format!("{name}_sats_by_type"),
            version,
            |source| {
                LazyColumnSpotValuePerBlock::with_addr_types(
                    &name, version, source, mappings, spot_price,
                )
            },
        )?))
    }

    pub fn min_resume_len(&self) -> usize {
        self.height.len()
    }

    pub fn par_iter_height_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        iter::once(self.stored_mut())
    }

    pub fn reset_height(&mut self) -> Result<()> {
        self.height.reset()?;
        Ok(())
    }

    #[inline(always)]
    pub fn push_supply(&mut self, supply: &AddrTypeToSupply) {
        self.push(supply.row());
    }
}
