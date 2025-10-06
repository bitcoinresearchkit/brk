use allocative::Allocative;
use brk_error::Result;

use brk_structs::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use brk_traversable::Traversable;
use vecdb::{AnyCloneableIterableVec, AnyIterableVec, Database, EagerVec, Exit};

use crate::{Indexes, grouped::LazyVecsBuilder, indexes};

use super::{ComputedType, EagerVecsBuilder, Source, VecBuilderOptions};

#[derive(Clone, Allocative)]
pub struct ComputedVecsFromDateIndex<T>
where
    T: ComputedType + PartialOrd,
{
    pub dateindex: Option<EagerVec<DateIndex, T>>,
    pub dateindex_extra: EagerVecsBuilder<DateIndex, T>,
    pub weekindex: LazyVecsBuilder<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazyVecsBuilder<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyVecsBuilder<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyVecsBuilder<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyVecsBuilder<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazyVecsBuilder<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromDateIndex<T>
where
    T: ComputedType + 'static,
{
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<DateIndex, T>,
        version: Version,
        indexes: &indexes::Vecs,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let dateindex = source.is_compute().then(|| {
            EagerVec::forced_import_compressed(db, name, version + VERSION + Version::ZERO).unwrap()
        });

        let dateindex_extra = EagerVecsBuilder::forced_import_compressed(
            db,
            name,
            version + VERSION + Version::ZERO,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        let dateindex_source = source.vec().or(dateindex.as_ref().map(|v| v.boxed_clone()));

        Ok(Self {
            weekindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.weekindex_to_weekindex.boxed_clone(),
                options.into(),
            ),
            monthindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.monthindex_to_monthindex.boxed_clone(),
                options.into(),
            ),
            quarterindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.quarterindex_to_quarterindex.boxed_clone(),
                options.into(),
            ),
            semesterindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.semesterindex_to_semesterindex.boxed_clone(),
                options.into(),
            ),
            yearindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.yearindex_to_yearindex.boxed_clone(),
                options.into(),
            ),
            decadeindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.decadeindex_to_decadeindex.boxed_clone(),
                options.into(),
            ),
            dateindex,
            dateindex_extra,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<DateIndex, T>) -> Result<()>,
    {
        compute(self.dateindex.as_mut().unwrap())?;

        let dateindex: Option<&EagerVec<DateIndex, T>> = None;
        self.compute_rest(starting_indexes, exit, dateindex)
    }

    pub fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        dateindex: Option<&impl AnyIterableVec<DateIndex, T>>,
    ) -> Result<()> {
        if let Some(dateindex) = dateindex {
            self.dateindex_extra
                .extend(starting_indexes.dateindex, dateindex, exit)?;
        } else {
            let dateindex = self.dateindex.as_ref().unwrap();

            self.dateindex_extra
                .extend(starting_indexes.dateindex, dateindex, exit)?;
        }

        Ok(())
    }
}

impl<T> Traversable for ComputedVecsFromDateIndex<T>
where
    T: ComputedType,
{
    fn to_tree_node(&self) -> brk_traversable::TreeNode {
        brk_traversable::TreeNode::List(
            [
                self.dateindex.as_ref().map(|nested| nested.to_tree_node()),
                Some(self.dateindex_extra.to_tree_node()),
                Some(self.weekindex.to_tree_node()),
                Some(self.monthindex.to_tree_node()),
                Some(self.quarterindex.to_tree_node()),
                Some(self.semesterindex.to_tree_node()),
                Some(self.yearindex.to_tree_node()),
                Some(self.decadeindex.to_tree_node()),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
        .collect_unique_leaves()
    }

    fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
        let mut regular_iter: Box<dyn Iterator<Item = &dyn vecdb::AnyCollectableVec>> =
            Box::new(self.dateindex_extra.iter_any_collectable());
        regular_iter = Box::new(regular_iter.chain(self.weekindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.monthindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.quarterindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.semesterindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.yearindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.decadeindex.iter_any_collectable()));
        if let Some(ref x) = self.dateindex {
            regular_iter = Box::new(regular_iter.chain(x.iter_any_collectable()));
        }
        regular_iter
    }
}
