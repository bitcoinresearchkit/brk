use brk_error::Result;

use brk_structs::{DifficultyEpoch, Height, Version};
use brk_traversable::Traversable;
use vecdb::{Database, EagerVec, Exit};

use crate::{Indexes, indexes};

use super::{ComputedType, EagerVecsBuilder, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeightStrict<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: EagerVec<Height, T>,
    pub height_extra: EagerVecsBuilder<Height, T>,
    pub difficultyepoch: EagerVecsBuilder<DifficultyEpoch, T>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromHeightStrict<T>
where
    T: ComputedType + Ord + From<f64>,
    f64: From<T>,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let height =
            EagerVec::forced_import_compressed(db, name, version + VERSION + Version::ZERO)?;

        let height_extra = EagerVecsBuilder::forced_import_compressed(
            db,
            name,
            version + VERSION + Version::ZERO,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            height,
            height_extra,
            difficultyepoch: EagerVecsBuilder::forced_import_compressed(
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
        F: FnMut(&mut EagerVec<Height, T>) -> Result<()>,
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
    T: ComputedType,
{
    fn to_tree_node(&self) -> brk_traversable::TreeNode {
        brk_traversable::TreeNode::List(
            [
                Some(self.height.to_tree_node()),
                Some(self.height_extra.to_tree_node()),
                Some(self.difficultyepoch.to_tree_node()),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
        .collect_unique_leaves()
    }
    fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
        let mut regular_iter: Box<dyn Iterator<Item = &dyn vecdb::AnyCollectableVec>> =
            Box::new(self.height.iter_any_collectable());
        regular_iter = Box::new(regular_iter.chain(self.height_extra.iter_any_collectable()));
        regular_iter = Box::new(regular_iter.chain(self.difficultyepoch.iter_any_collectable()));
        regular_iter
    }
}
