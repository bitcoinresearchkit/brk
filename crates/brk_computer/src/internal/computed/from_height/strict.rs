use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Height, Version};
use schemars::JsonSchema;
use vecdb::{
    AnyExportableVec, Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec,
};

use crate::{ComputeIndexes, indexes};

use crate::internal::{ComputedVecValue, EagerVecsBuilder, LazyVecsBuilder, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeightStrict<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: EagerVec<PcoVec<Height, T>>,
    pub height_extra: EagerVecsBuilder<Height, T>,
    pub difficultyepoch: LazyVecsBuilder<DifficultyEpoch, T, Height, DifficultyEpoch>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromHeightStrict<T>
where
    T: ComputedVecValue + Ord + From<f64> + JsonSchema,
    f64: From<T>,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
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
            difficultyepoch: LazyVecsBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                Some(height.boxed_clone()),
                &height_extra,
                indexes.block.difficultyepoch_to_difficultyepoch.boxed_clone(),
                options.into(),
            ),
            height,
            height_extra,
            // halvingepoch: StorableVecGeneator::forced_import(db, name, version + VERSION + Version::ZERO, format, options)?,
        })
    }

    pub fn compute<F>(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    {
        compute(&mut self.height)?;

        self.height_extra
            .extend(starting_indexes.height, &self.height, exit)?;

        Ok(())
    }
}

impl<T> Traversable for ComputedVecsFromHeightStrict<T>
where
    T: ComputedVecValue + JsonSchema,
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
