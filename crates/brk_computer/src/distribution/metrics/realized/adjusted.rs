use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    distribution::metrics::ImportConfig,
    internal::{PerBlockCumulativeWithSums, RatioCents64, RollingWindows},
};

#[derive(Traversable)]
pub struct AdjustedSopr<M: StorageMode = Rw> {
    pub ratio: RollingWindows<StoredF64, M>,
    pub transfer_volume: PerBlockCumulativeWithSums<Cents, Cents, M>,
    pub value_destroyed: PerBlockCumulativeWithSums<Cents, Cents, M>,
}

impl AdjustedSopr {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            ratio: cfg.import("asopr", Version::ONE)?,
            transfer_volume: cfg.import("adj_value_created", Version::ONE)?,
            value_destroyed: cfg.import("adj_value_destroyed", Version::ONE)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        starting_indexes: &Indexes,
        base_transfer_volume: &impl ReadableVec<Height, Cents>,
        base_value_destroyed: &impl ReadableVec<Height, Cents>,
        under_1h_transfer_volume: &impl ReadableVec<Height, Cents>,
        under_1h_value_destroyed: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self.transfer_volume.base.height.compute_subtract(
            starting_indexes.height,
            base_transfer_volume,
            under_1h_transfer_volume,
            exit,
        )?;
        self.value_destroyed.base.height.compute_subtract(
            starting_indexes.height,
            base_value_destroyed,
            under_1h_value_destroyed,
            exit,
        )?;

        self.transfer_volume
            .compute_rest(starting_indexes.height, exit)?;
        self.value_destroyed
            .compute_rest(starting_indexes.height, exit)?;

        for ((sopr, tv), vd) in self
            .ratio
            .as_mut_array()
            .into_iter()
            .zip(self.transfer_volume.sum.as_array())
            .zip(self.value_destroyed.sum.as_array())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &tv.height,
                &vd.height,
                exit,
            )?;
        }

        Ok(())
    }
}
