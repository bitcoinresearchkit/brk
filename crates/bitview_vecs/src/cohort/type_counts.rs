use bitview_cohort::{ByAddrType, ByType, SpendableType};
use bitview_collections::Windows;
use bitview_traversable::Traversable;
use brk_types::{Height, PartsPerMillion32, StoredU16, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::ReadableCloneableVec;

use crate::{
    CountTotal, CumulativeCountVec, IndexSources, LazyCountPerBlockCumulativeRolling,
    LazyPerBlockCumulativeRolling, LazyPercentCumulativeRolling,
};

/// A shared total plus a typed count breakdown. The group determines membership;
/// the total component determines the denominator, independently of the per-type sources.
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct TypeCounts<S> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub total: CountTotal,
    #[traversable(flatten)]
    pub by_type: S,
}

pub type SpendableTypeCounts<V> = TypeCounts<SpendableType<V>>;
pub type OutputTypeCounts<V> = TypeCounts<ByType<V>>;

// Both typed domains use the same vector assembly, without erasing either
// group's concrete fields or changing its iteration order.
macro_rules! impl_type_counts {
    ($group:ident) => {
        impl TypeCounts<$group<LazyCountPerBlockCumulativeRolling>> {
            pub fn from_count_sources(
                total: CountTotal,
                per_type_name: impl Fn(&str) -> String,
                version: Version,
                sources: &$group<impl ReadableCloneableVec<Height, StoredU16>>,
                indexes: &IndexSources,
                windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
            ) -> Self {
                let by_type = sources.map_with_id(|id, source| {
                    LazyCountPerBlockCumulativeRolling::from_height_source(
                        &per_type_name(id.name()),
                        version,
                        source,
                        indexes,
                        windows,
                    )
                });
                Self { total, by_type }
            }

            pub fn lazy_shares(
                &self,
                version: Version,
                name: impl Fn(&str) -> String,
                windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
                indexes: &IndexSources,
            ) -> $group<LazyPercentCumulativeRolling<PartsPerMillion32>> {
                self.by_type.map_with_id(|id, source| {
                    self.total.lazy_share(
                        &name(id.name()),
                        version,
                        &source.cumulative_source(),
                        windows,
                        indexes,
                    )
                })
            }

            pub fn addr_type_counts(&self) -> ByAddrType<CumulativeCountVec> {
                ByAddrType::from_fn(|id| self.by_type.get(id.output_type()).cumulative_source())
            }
        }

        impl TypeCounts<$group<LazyPerBlockCumulativeRolling<StoredU64>>> {
            pub fn from_cumulative_sources(
                total: CountTotal,
                per_type_name: impl Fn(&str) -> String,
                version: Version,
                sources: &$group<impl ReadableCloneableVec<Height, StoredU64>>,
                indexes: &IndexSources,
                windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
            ) -> Self {
                let by_type = sources.map_with_id(|id, source| {
                    LazyPerBlockCumulativeRolling::from_cumulative_source(
                        &per_type_name(id.name()),
                        version,
                        source,
                        windows,
                        indexes,
                    )
                });
                Self { total, by_type }
            }

            pub fn lazy_shares(
                &self,
                version: Version,
                name: impl Fn(&str) -> String,
                windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
                indexes: &IndexSources,
            ) -> $group<LazyPercentCumulativeRolling<PartsPerMillion32>> {
                self.by_type.map_with_id(|id, source| {
                    self.total.lazy_share(
                        &name(id.name()),
                        version,
                        &source.cumulative.height,
                        windows,
                        indexes,
                    )
                })
            }
        }
    };
}

impl_type_counts!(ByType);
impl_type_counts!(SpendableType);
