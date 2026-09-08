use bitview_cohort::{ByAddrType, ByType, Filter, OutputTypeId, SpendableType, SpendableTypeId};
use bitview_collections::Windows;
use bitview_traversable::Traversable;
use brk_types::{Height, PartsPerMillion32, StoredU16, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec};

use crate::{
    CountTotal, CumulativeCountVec, IndexSources, LazyColumnCountPerBlockCumulativeRolling,
    LazyColumnPerBlockCumulativeRolling, LazyPercentCumulativeRolling,
};

/// A shared total plus a typed count breakdown. The group determines membership;
/// the total component determines the denominator, independently of the columns.
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
// group's concrete fields or changing its column order.
macro_rules! impl_type_counts {
    ($group:ident, $column:ident, $column_from_type:expr) => {
        impl TypeCounts<$group<LazyColumnCountPerBlockCumulativeRolling>> {
            pub fn from_columnar_count_source(
                total: CountTotal,
                per_type_name: impl Fn(&str) -> String,
                version: Version,
                source: &ReadOnlyColumnarVec<PcoVec<Height, StoredU16>, $column>,
                indexes: &IndexSources,
                windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
            ) -> Self {
                let by_type = $group::new(|filter, name| {
                    let Filter::Type(output_type) = filter else {
                        unreachable!()
                    };
                    LazyColumnCountPerBlockCumulativeRolling::new(
                        &per_type_name(name),
                        version,
                        source,
                        ($column_from_type)(output_type),
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
                $group::new(|filter, type_name| {
                    let Filter::Type(output_type) = filter else {
                        unreachable!()
                    };
                    self.total.lazy_share(
                        &name(type_name),
                        version,
                        &self.by_type.get(output_type).cumulative_source(),
                        windows,
                        indexes,
                    )
                })
            }

            pub fn addr_type_counts(&self) -> ByAddrType<CumulativeCountVec> {
                ByAddrType::new(|filter| {
                    let Filter::Type(output_type) = filter else {
                        unreachable!()
                    };
                    self.by_type.get(output_type).cumulative_source()
                })
            }

            pub fn invalidate(&self) {
                self.total.invalidate();
                self.by_type.iter().for_each(|count| count.invalidate());
            }
        }

        impl TypeCounts<$group<LazyColumnPerBlockCumulativeRolling<StoredU64, $column>>> {
            pub fn from_columnar_source(
                cache: &'static CacheBudget,
                total: CountTotal,
                per_type_name: impl Fn(&str) -> String,
                version: Version,
                source: &ReadOnlyColumnarVec<PcoVec<Height, StoredU64>, $column>,
                indexes: &IndexSources,
                windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
            ) -> Self {
                let by_type = $group::new(|filter, name| {
                    let Filter::Type(output_type) = filter else {
                        unreachable!()
                    };
                    LazyColumnPerBlockCumulativeRolling::new(
                        cache,
                        &per_type_name(name),
                        version,
                        source,
                        ($column_from_type)(output_type),
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
                $group::new(|filter, type_name| {
                    let Filter::Type(output_type) = filter else {
                        unreachable!()
                    };
                    self.total.lazy_share(
                        &name(type_name),
                        version,
                        self.by_type
                            .get(output_type)
                            .cumulative
                            .resolutions
                            .height_source(),
                        windows,
                        indexes,
                    )
                })
            }
        }
    };
}

impl_type_counts!(ByType, OutputTypeId, OutputTypeId::from_output_type);
impl_type_counts!(SpendableType, SpendableTypeId, |output_type| {
    SpendableTypeId::from_output_type(output_type).expect("spendable output type column")
});
