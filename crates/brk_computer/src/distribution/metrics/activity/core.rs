use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Indexes, Sats, StoredF64, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    distribution::{metrics::ImportConfig, state::{CohortState, RealizedOps}},
    internal::PerBlockWithSum24h,
};

#[derive(Traversable)]
pub struct ActivityCore<M: StorageMode = Rw> {
    pub sent: PerBlockWithSum24h<Sats, M>,
    pub coindays_destroyed: PerBlockWithSum24h<StoredF64, M>,
}

impl ActivityCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        Ok(Self {
            sent: cfg.import("sent", v1)?,
            coindays_destroyed: cfg.import("coindays_destroyed", v1)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.sent
            .raw
            .height
            .len()
            .min(self.coindays_destroyed.raw.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        state: &CohortState<impl RealizedOps>,
    ) -> Result<()> {
        self.sent.raw.height.truncate_push(height, state.sent)?;
        self.coindays_destroyed.raw.height.truncate_push(
            height,
            StoredF64::from(Bitcoin::from(state.satdays_destroyed)),
        )?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.sent.raw.height as &mut dyn AnyStoredVec,
            &mut self.coindays_destroyed.raw.height,
        ]
    }

    pub(crate) fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        Ok(())
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.sent.raw.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent.raw.height)
                .collect::<Vec<_>>(),
            exit,
        )?;

        sum_others!(self, starting_indexes, others, exit; coindays_destroyed.raw.height);

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sent.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.sent.raw.height,
            exit,
        )?;
        self.coindays_destroyed.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.coindays_destroyed.raw.height,
            exit,
        )?;
        Ok(())
    }
}
