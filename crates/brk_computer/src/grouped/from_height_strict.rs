use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Height, Version};
use vecdb::{AnyExportableVec, Database, EagerVec, Exit, ImportableVec, PcoVec};

use crate::{Indexes, indexes};

use super::{ComputedVecValue, EagerVecsBuilder, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeightStrict<T>
where
    T: ComputedVecValue + PartialOrd,
{
    pub height: EagerVec<PcoVec<Height, T>>,
    pub height_extra: EagerVecsBuilder<Height, T>,
    pub difficultyepoch: EagerVecsBuilder<DifficultyEpoch, T>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromHeightStrict<T>
where
    T: ComputedVecValue + Ord + From<f64>,
    f64: From<T>,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let height = EagerVec::forced_import(db, name, version + VERSION + Version::ZERO)?;

        let height_extra = EagerVecsBuilder::forced_import(
            db,
            name,
            version + VERSION + Version::ZERO,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            height,
            height_extra,
            difficultyepoch: EagerVecsBuilder::forced_import(
                db,
                name,
                version + VERSION + Version::ZERO,
                options,
            )?,
            // halvingepoch: StorableVecGeneator::forced_import(db, name, version + VERSION + Version::ZERO, format, options)?,
        })
    }

    pub fn compute<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    {
        compute(&mut self.height)?;

        self.height_extra
            .extend(starting_indexes.height, &self.height, exit)?;

        self.difficultyepoch.compute(
            starting_indexes.difficultyepoch,
            &self.height,
            &indexes.difficultyepoch_to_first_height,
            &indexes.difficultyepoch_to_height_count,
            exit,
        )?;

        Ok(())
    }
}

impl<T> Traversable for ComputedVecsFromHeightStrict<T>
where
    T: ComputedVecValue,
{
    fn to_tree_node(&self) -> brk_traversable::TreeNode {
        let height_extra_node = self.height_extra.to_tree_node();
        brk_traversable::TreeNode::Branch(
            [
                Some(("height".to_string(), self.height.to_tree_node())),
                if height_extra_node.is_empty() {
                    None
                } else {
                    Some(("height_extra".to_string(), height_extra_node))
                },
                Some((
                    "difficultyepoch".to_string(),
                    self.difficultyepoch.to_tree_node(),
                )),
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
            Box::new(self.height.iter_any_exportable());
        regular_iter = Box::new(regular_iter.chain(self.height_extra.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.difficultyepoch.iter_any_exportable()));
        regular_iter
    }
}
