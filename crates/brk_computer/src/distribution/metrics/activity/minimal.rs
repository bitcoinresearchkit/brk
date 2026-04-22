use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{
        metrics::ImportConfig,
        state::{CohortState, CostBasisOps, RealizedOps},
    },
    internal::ValuePerBlockCumulativeRolling,
    prices,
};

#[derive(Traversable)]
pub struct ActivityMinimal<M: StorageMode = Rw> {
    pub transfer_volume: ValuePerBlockCumulativeRolling<M>,
}

impl ActivityMinimal {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        Ok(Self {
            transfer_volume: cfg.import("transfer_volume", v1)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.transfer_volume.block.sats.len()
    }

    #[inline(always)]
    pub(crate) fn push_state(&mut self, state: &CohortState<impl RealizedOps, impl CostBasisOps>) {
        self.transfer_volume.block.sats.push(state.sent);
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let inner = &mut self.transfer_volume.inner;
        vec![
            &mut inner.block.sats as &mut dyn AnyStoredVec,
            &mut inner.block.cents,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.transfer_volume.block.sats.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.transfer_volume.block.sats)
                .collect::<Vec<_>>(),
            exit,
        )?;

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.transfer_volume
            .compute_rest(starting_indexes.height, prices, exit)?;
        Ok(())
    }
}
