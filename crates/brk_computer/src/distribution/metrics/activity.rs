use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Indexes, Sats, StoredF64, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    internal::{ComputedFromHeightCumulativeSum, RollingEmas2w, ValueFromHeightCumulative},
};

use super::ImportConfig;

/// Activity metrics for a cohort.
#[derive(Traversable)]
pub struct ActivityMetrics<M: StorageMode = Rw> {
    /// Total satoshis sent at each height + derived indexes
    pub sent: ValueFromHeightCumulative<M>,

    /// 14-day EMA of sent supply (sats, btc, usd)
    pub sent_ema: RollingEmas2w<M>,

    /// Coin-blocks destroyed (in BTC)
    pub coinblocks_destroyed: ComputedFromHeightCumulativeSum<StoredF64, M>,

    /// Coin-days destroyed (in BTC)
    pub coindays_destroyed: ComputedFromHeightCumulativeSum<StoredF64, M>,
}

impl ActivityMetrics {
    /// Import activity metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            sent: cfg.import_value_cumulative("sent", Version::ZERO)?,
            sent_ema: cfg.import_emas_2w("sent", Version::ZERO)?,

            coinblocks_destroyed: cfg
                .import_cumulative_sum("coinblocks_destroyed", Version::ONE)?,
            coindays_destroyed: cfg.import_cumulative_sum("coindays_destroyed", Version::ONE)?,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub(crate) fn min_len(&self) -> usize {
        self.sent
            .base
            .sats
            .height
            .len()
            .min(self.coinblocks_destroyed.height.len())
            .min(self.coindays_destroyed.height.len())
    }

    /// Push activity state values to height-indexed vectors.
    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        sent: Sats,
        satblocks_destroyed: Sats,
        satdays_destroyed: Sats,
    ) -> Result<()> {
        self.sent.base.sats.height.truncate_push(height, sent)?;
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

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.sent.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.coinblocks_destroyed.height as &mut dyn AnyStoredVec,
            &mut self.coindays_destroyed.height as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }

    /// Validate computed versions against base version.
    pub(crate) fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        // Validation logic for computed vecs
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        macro_rules! sum_others {
            ($($field:tt).+) => {
                self.$($field).+.compute_sum_of_others(
                    starting_indexes.height,
                    &others.iter().map(|v| &v.$($field).+).collect::<Vec<_>>(),
                    exit,
                )?
            };
        }

        sum_others!(sent.base.sats.height);
        sum_others!(coinblocks_destroyed.height);
        sum_others!(coindays_destroyed.height);
        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = blocks.count.window_starts();

        // 14-day EMA of sent (sats and dollars)
        self.sent_ema.compute(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent.base.sats.height,
            &self.sent.base.cents.height,
            exit,
        )?;

        self.coinblocks_destroyed
            .compute_rest(starting_indexes.height, &window_starts, exit)?;

        self.coindays_destroyed
            .compute_rest(starting_indexes.height, &window_starts, exit)?;

        Ok(())
    }
}
