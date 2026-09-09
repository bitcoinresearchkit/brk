use bitview_cohort::{ADDR_TYPE_IDS, AddrTypeId, WithAddrTypes};
use bitview_compute::NumericValue;
use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{
    ColumnId, Formattable, Ident, LazyColumnVec, PcoVec, PcoVecValue, ReadOnlyColumnarVec,
    ReadableColumnarVec,
};

use crate::{IndexSources, LazyPerBlock, Resolutions};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyColumnPerBlock<T, C>
where
    T: PcoVecValue + Formattable + PartialOrd + Serialize + JsonSchema,
    C: ColumnId,
{
    pub height: LazyColumnVec<ReadOnlyColumnarVec<PcoVec<Height, T>, C>, C>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub resolutions: Box<Resolutions<T>>,
}

impl<T, C> LazyColumnPerBlock<T, C>
where
    T: PcoVecValue + Formattable + PartialOrd + Serialize + JsonSchema + 'static,
    C: ColumnId,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, T>, C>,
        column: C,
        indexes: &IndexSources,
    ) -> Self {
        let height = source.column(name, version, column);
        let resolutions = Resolutions::from_source(name, &height, version, indexes);

        Self {
            height,
            resolutions: Box::new(resolutions),
        }
    }
}

impl<T> LazyColumnPerBlock<T, AddrTypeId>
where
    T: NumericValue + JsonSchema + 'static,
{
    pub fn with_addr_types(
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, T>, AddrTypeId>,
        indexes: &IndexSources,
    ) -> WithAddrTypes<Self, LazyPerBlock<T>> {
        let all_source = source.sum_columns(name, version, ADDR_TYPE_IDS);
        let all = LazyPerBlock::from_height_source::<Ident>(name, version, &all_source, indexes);
        let by_addr_type = AddrTypeId::series(|column, type_name| {
            LazyColumnPerBlock::new(
                &format!("{type_name}_{name}"),
                version,
                source,
                column,
                indexes,
            )
        });

        WithAddrTypes { all, by_addr_type }
    }
}
