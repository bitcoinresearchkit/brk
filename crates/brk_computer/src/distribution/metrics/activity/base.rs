use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Indexes, Sats, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::internal::ComputedFromHeightCumulativeSum;

use crate::{blocks, distribution::metrics::ImportConfig, prices};

use super::CoreActivity;

#[derive(Deref, DerefMut, Traversable)]
pub struct ActivityMetrics<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: CoreActivity<M>,

    pub coinblocks_destroyed: ComputedFromHeightCumulativeSum<StoredF64, M>,
    pub coindays_destroyed: ComputedFromHeightCumulativeSum<StoredF64, M>,
}

impl ActivityMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            core: CoreActivity::forced_import(cfg)?,
            coinblocks_destroyed: cfg
                .import_cumulative_sum("coinblocks_destroyed", Version::ONE)?,
            coindays_destroyed: cfg.import_cumulative_sum("coindays_destroyed", Version::ONE)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.core
            .min_len()
            .min(self.coinblocks_destroyed.height.len())
            .min(self.coindays_destroyed.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        sent: Sats,
        satblocks_destroyed: Sats,
        satdays_destroyed: Sats,
    ) -> Result<()> {
        self.core.truncate_push(height, sent)?;
        self.coinblocks_destroyed.height.truncate_push(
            height,
            StoredF64::from(Bitcoin::from(satblocks_destroyed)),
        )?;
        self.coindays_destroyed.height.truncate_push(
            height,
            StoredF64::from(Bitcoin::from(satdays_destroyed)),
        )?;
        Ok(())
    }

    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.core.sent.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.coinblocks_destroyed.height as &mut dyn AnyStoredVec,
            &mut self.coindays_destroyed.height as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
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
        let core_refs: Vec<&CoreActivity> = others.iter().map(|o| &o.core).collect();
        self.core
            .compute_from_stateful(starting_indexes, &core_refs, exit)?;

        sum_others!(self, starting_indexes, others, exit; coinblocks_destroyed.height);
        sum_others!(self, starting_indexes, others, exit; coindays_destroyed.height);
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.core
            .compute_rest_part1(blocks, prices, starting_indexes, exit)?;

        let window_starts = blocks.count.window_starts();

        self.coinblocks_destroyed
            .compute_rest(starting_indexes.height, &window_starts, exit)?;

        self.coindays_destroyed
            .compute_rest(starting_indexes.height, &window_starts, exit)?;

        Ok(())
    }
}
