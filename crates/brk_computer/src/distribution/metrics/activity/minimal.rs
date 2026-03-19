use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{metrics::ImportConfig, state::{CohortState, CostBasisOps, RealizedOps}},
    internal::AmountPerBlockCumulativeWithSums,
    prices,
};

#[derive(Traversable)]
pub struct ActivityMinimal<M: StorageMode = Rw> {
    pub transfer_volume: AmountPerBlockCumulativeWithSums<M>,
}

impl ActivityMinimal {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        Ok(Self {
            transfer_volume: cfg.import("transfer_volume", v1)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.transfer_volume
            .base
            .sats
            .height
            .len()
    }

    #[inline(always)]
    pub(crate) fn push_state(
        &mut self,
        state: &CohortState<impl RealizedOps, impl CostBasisOps>,
    ) {
        self.transfer_volume.base.sats.height.push(state.sent);
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.transfer_volume.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.transfer_volume.base.cents.height,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.transfer_volume.base.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.transfer_volume.base.sats.height)
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
