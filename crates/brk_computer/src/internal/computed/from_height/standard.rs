use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    TreeNode, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{
    AnyExportableVec, Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, IterableVec,
    PcoVec,
};

use crate::{
    ComputeIndexes, indexes,
    internal::{LazyVecsBuilder, Source},
    utils::OptionExt,
};

use crate::internal::{ComputedVecValue, EagerVecsBuilder, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeight<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: Option<EagerVec<PcoVec<Height, T>>>,
    pub height_extra: EagerVecsBuilder<Height, T>,
    pub dateindex: EagerVecsBuilder<DateIndex, T>,
    pub weekindex: LazyVecsBuilder<WeekIndex, T, DateIndex, WeekIndex>,
    pub difficultyepoch: LazyVecsBuilder<DifficultyEpoch, T, Height, DifficultyEpoch>,
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
    T: ComputedVecValue + Ord + From<f64> + JsonSchema + 'static,
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
        let height = source
            .is_compute()
            .then(|| EagerVec::forced_import(db, name, version + VERSION).unwrap());

        let height_extra = EagerVecsBuilder::forced_import(
            db,
            name,
            version + VERSION,
            options.copy_self_extra(),
        )?;

        let dateindex = EagerVecsBuilder::forced_import(db, name, version + VERSION, options)?;

        let options = options.remove_percentiles();

        let height_source = source.vec().or(height.as_ref().map(|v| v.boxed_clone()));

        Ok(Self {
            weekindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.weekindex_to_weekindex.boxed_clone(),
                options.into(),
            ),
            monthindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.monthindex_to_monthindex.boxed_clone(),
                options.into(),
            ),
            quarterindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
                options.into(),
            ),
            semesterindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
                options.into(),
            ),
            yearindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.yearindex_to_yearindex.boxed_clone(),
                options.into(),
            ),
            decadeindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
                options.into(),
            ),
            // halvingepoch: StorableVecGeneator::forced_import(db, name, version + VERSION , format, options)?,
            difficultyepoch: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                height_source,
                &height_extra,
                indexes
                    .block
                    .difficultyepoch_to_difficultyepoch
                    .boxed_clone(),
                options.into(),
            ),
            height,
            height_extra,
            dateindex,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    {
        compute(self.height.um())?;

        let height: Option<&EagerVec<PcoVec<Height, T>>> = None;
        self.compute_rest(indexes, starting_indexes, exit, height)
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        height_vec: Option<&impl IterableVec<Height, T>>,
    ) -> Result<()> {
        if let Some(height) = height_vec {
            self.height_extra
                .extend(starting_indexes.height, height, exit)?;

            self.dateindex.compute(
                starting_indexes.dateindex,
                height,
                &indexes.time.dateindex_to_first_height,
                &indexes.time.dateindex_to_height_count,
                exit,
            )?;
        } else {
            let height = self.height.u();

            self.height_extra
                .extend(starting_indexes.height, height, exit)?;

            self.dateindex.compute(
                starting_indexes.dateindex,
                height,
                &indexes.time.dateindex_to_first_height,
                &indexes.time.dateindex_to_height_count,
                exit,
            )?;
        }

        Ok(())
    }
}

impl<T> Traversable for ComputedVecsFromHeight<T>
where
    T: ComputedVecValue + JsonSchema,
{
    fn to_tree_node(&self) -> TreeNode {
        let height_extra_node = self.height_extra.to_tree_node();
        TreeNode::Branch(
            [
                self.height
                    .as_ref()
                    .map(|nested| ("height".to_string(), nested.to_tree_node())),
                if height_extra_node.is_empty() {
                    None
                } else {
                    Some(("height_extra".to_string(), height_extra_node))
                },
                Some(("dateindex".to_string(), self.dateindex.to_tree_node())),
                Some(("weekindex".to_string(), self.weekindex.to_tree_node())),
                Some((
                    "difficultyepoch".to_string(),
                    self.difficultyepoch.to_tree_node(),
                )),
                Some(("monthindex".to_string(), self.monthindex.to_tree_node())),
                Some(("quarterindex".to_string(), self.quarterindex.to_tree_node())),
                Some((
                    "semesterindex".to_string(),
                    self.semesterindex.to_tree_node(),
                )),
                Some(("yearindex".to_string(), self.yearindex.to_tree_node())),
                Some(("decadeindex".to_string(), self.decadeindex.to_tree_node())),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
        .merge_branches()
        .unwrap()
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        let mut regular_iter: Box<dyn Iterator<Item = &dyn AnyExportableVec>> =
            Box::new(self.height_extra.iter_any_exportable());
        regular_iter = Box::new(regular_iter.chain(self.dateindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.weekindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.difficultyepoch.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.monthindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.quarterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.semesterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.yearindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.decadeindex.iter_any_exportable()));
        if let Some(ref x) = self.height {
            regular_iter = Box::new(regular_iter.chain(x.iter_any_exportable()));
        }
        regular_iter
    }
}
