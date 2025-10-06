use allocative::Allocative;
use brk_error::Result;

use brk_structs::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    Version, WeekIndex, YearIndex,
};
use brk_traversable::Traversable;
use vecdb::{AnyCloneableIterableVec, AnyIterableVec, Database, EagerVec, Exit};

use crate::{
    Indexes,
    grouped::{LazyVecsBuilder, Source},
    indexes,
};

use super::{ComputedType, EagerVecsBuilder, VecBuilderOptions};

#[derive(Clone, Allocative)]
pub struct ComputedVecsFromHeight<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: Option<EagerVec<Height, T>>,
    pub height_extra: EagerVecsBuilder<Height, T>,
    pub dateindex: EagerVecsBuilder<DateIndex, T>,
    pub weekindex: LazyVecsBuilder<WeekIndex, T, DateIndex, WeekIndex>,
    pub difficultyepoch: EagerVecsBuilder<DifficultyEpoch, T>,
    pub monthindex: LazyVecsBuilder<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyVecsBuilder<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyVecsBuilder<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyVecsBuilder<YearIndex, T, DateIndex, YearIndex>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: LazyVecsBuilder<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromHeight<T>
where
    T: ComputedType + Ord + From<f64> + 'static,
    f64: From<T>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let height = source.is_compute().then(|| {
            EagerVec::forced_import_compressed(db, name, version + VERSION + Version::ZERO).unwrap()
        });

        let height_extra = EagerVecsBuilder::forced_import_compressed(
            db,
            name,
            version + VERSION + Version::ZERO,
            options.copy_self_extra(),
        )?;

        let dateindex = EagerVecsBuilder::forced_import_compressed(
            db,
            name,
            version + VERSION + Version::ZERO,
            options,
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            weekindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.weekindex_to_weekindex.boxed_clone(),
                options.into(),
            ),
            monthindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.monthindex_to_monthindex.boxed_clone(),
                options.into(),
            ),
            quarterindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.quarterindex_to_quarterindex.boxed_clone(),
                options.into(),
            ),
            semesterindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.semesterindex_to_semesterindex.boxed_clone(),
                options.into(),
            ),
            yearindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.yearindex_to_yearindex.boxed_clone(),
                options.into(),
            ),
            decadeindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.decadeindex_to_decadeindex.boxed_clone(),
                options.into(),
            ),
            // halvingepoch: StorableVecGeneator::forced_import(db, name, version + VERSION + Version::ZERO, format, options)?,
            height,
            height_extra,
            dateindex,
            difficultyepoch: EagerVecsBuilder::forced_import_compressed(
                db,
                name,
                version + VERSION + Version::ZERO,
                options,
            )?,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<Height, T>) -> Result<()>,
    {
        compute(self.height.as_mut().unwrap())?;

        let height: Option<&EagerVec<Height, T>> = None;
        self.compute_rest(indexes, starting_indexes, exit, height)
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        height_vec: Option<&impl AnyIterableVec<Height, T>>,
    ) -> Result<()> {
        if let Some(height) = height_vec {
            self.height_extra
                .extend(starting_indexes.height, height, exit)?;

            self.dateindex.compute(
                starting_indexes.dateindex,
                height,
                &indexes.dateindex_to_first_height,
                &indexes.dateindex_to_height_count,
                exit,
            )?;

            self.difficultyepoch.compute(
                starting_indexes.difficultyepoch,
                height,
                &indexes.difficultyepoch_to_first_height,
                &indexes.difficultyepoch_to_height_count,
                exit,
            )?;
        } else {
            let height = self.height.as_ref().unwrap();

            self.height_extra
                .extend(starting_indexes.height, height, exit)?;

            self.dateindex.compute(
                starting_indexes.dateindex,
                height,
                &indexes.dateindex_to_first_height,
                &indexes.dateindex_to_height_count,
                exit,
            )?;

            self.difficultyepoch.compute(
                starting_indexes.difficultyepoch,
                height,
                &indexes.difficultyepoch_to_first_height,
                &indexes.difficultyepoch_to_height_count,
                exit,
            )?;
        }

        Ok(())
    }
}

impl<T> Traversable for ComputedVecsFromHeight<T>
where
    T: ComputedType,
{
    fn to_tree_node(&self) -> brk_traversable::TreeNode {
        brk_traversable::TreeNode::List(
            [
                self.height.as_ref().map(|nested| nested.to_tree_node()),
                Some(self.height_extra.to_tree_node()),
                Some(self.dateindex.to_tree_node()),
                Some(self.weekindex.to_tree_node()),
                Some(self.difficultyepoch.to_tree_node()),
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
            Box::new(self.height_extra.iter_any_collectable());
        regular_iter = Box::new(regular_iter.chain(self.dateindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.weekindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.difficultyepoch.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.monthindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.quarterindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.semesterindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.yearindex.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.decadeindex.iter_any_collectable()));
        if let Some(ref x) = self.height {
            regular_iter = Box::new(regular_iter.chain(x.iter_any_collectable()));
        }
        regular_iter
    }
}
