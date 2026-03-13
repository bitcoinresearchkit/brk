use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    distribution::metrics::ImportConfig,
    internal::{ComputedPerBlockCumulativeWithSums, RatioCents64, RollingWindows},
};

#[derive(Traversable)]
pub struct AdjustedSopr<M: StorageMode = Rw> {
    pub value_created: ComputedPerBlockCumulativeWithSums<Cents, Cents, M>,
    pub value_destroyed: ComputedPerBlockCumulativeWithSums<Cents, Cents, M>,
    pub ratio: RollingWindows<StoredF64, M>,
}

impl AdjustedSopr {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            value_created: cfg.import("adj_value_created", Version::ONE)?,
            value_destroyed: cfg.import("adj_value_destroyed", Version::ONE)?,
            ratio: cfg.import("asopr", Version::ONE)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        starting_indexes: &Indexes,
        base_value_created: &impl ReadableVec<Height, Cents>,
        base_value_destroyed: &impl ReadableVec<Height, Cents>,
        under_1h_value_created: &impl ReadableVec<Height, Cents>,
        under_1h_value_destroyed: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        // Compute value_created = base.value_created - under_1h.value_created
        self.value_created.raw.height.compute_subtract(
            starting_indexes.height,
            base_value_created,
            under_1h_value_created,
            exit,
        )?;
        self.value_destroyed.raw.height.compute_subtract(
            starting_indexes.height,
            base_value_destroyed,
            under_1h_value_destroyed,
            exit,
        )?;

        // Cumulatives (rolling sums are lazy)
        self.value_created
            .compute_rest(starting_indexes.height, exit)?;
        self.value_destroyed
            .compute_rest(starting_indexes.height, exit)?;

        // SOPR ratios from lazy rolling sums
        for ((sopr, vc), vd) in self
            .ratio
            .as_mut_array()
            .into_iter()
            .zip(self.value_created.sum.as_array())
            .zip(self.value_destroyed.sum.as_array())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &vc.height,
                &vd.height,
                exit,
            )?;
        }

        Ok(())
    }
}
