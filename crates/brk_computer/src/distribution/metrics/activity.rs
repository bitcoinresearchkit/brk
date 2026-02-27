use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Sats, StoredF64, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode, WritableVec};

use crate::{
    ComputeIndexes, blocks,
    internal::{ComputedFromHeightCumulativeSum, LazyComputedValueFromHeightCumulative, ValueEmaFromHeight},
};

use super::ImportConfig;

/// Activity metrics for a cohort.
#[derive(Traversable)]
pub struct ActivityMetrics<M: StorageMode = Rw> {
    /// Total satoshis sent at each height + derived indexes
    pub sent: LazyComputedValueFromHeightCumulative<M>,

    /// 14-day EMA of sent supply (sats, btc, usd)
    pub sent_14d_ema: ValueEmaFromHeight<M>,

    /// Satoshi-blocks destroyed (supply * blocks_old when spent)
    pub satblocks_destroyed: M::Stored<EagerVec<PcoVec<Height, Sats>>>,

    /// Satoshi-days destroyed (supply * days_old when spent)
    pub satdays_destroyed: M::Stored<EagerVec<PcoVec<Height, Sats>>>,

    /// Coin-blocks destroyed (in BTC rather than sats)
    pub coinblocks_destroyed: ComputedFromHeightCumulativeSum<StoredF64, M>,

    /// Coin-days destroyed (in BTC rather than sats)
    pub coindays_destroyed: ComputedFromHeightCumulativeSum<StoredF64, M>,
}

impl ActivityMetrics {
    /// Import activity metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            sent: LazyComputedValueFromHeightCumulative::forced_import(
                cfg.db,
                &cfg.name("sent"),
                cfg.version,
                cfg.indexes,
            )?,

            sent_14d_ema: ValueEmaFromHeight::forced_import(
                cfg.db,
                &cfg.name("sent_14d_ema"),
                cfg.version,
                cfg.indexes,
            )?,

            satblocks_destroyed: EagerVec::forced_import(
                cfg.db,
                &cfg.name("satblocks_destroyed"),
                cfg.version,
            )?,

            satdays_destroyed: EagerVec::forced_import(
                cfg.db,
                &cfg.name("satdays_destroyed"),
                cfg.version,
            )?,

            coinblocks_destroyed: ComputedFromHeightCumulativeSum::forced_import(
                cfg.db,
                &cfg.name("coinblocks_destroyed"),
                cfg.version,
                cfg.indexes,
            )?,

            coindays_destroyed: ComputedFromHeightCumulativeSum::forced_import(
                cfg.db,
                &cfg.name("coindays_destroyed"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub(crate) fn min_len(&self) -> usize {
        self.sent
            .base
            .sats
            .height
            .len()
            .min(self.satblocks_destroyed.len())
            .min(self.satdays_destroyed.len())
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
        self.satblocks_destroyed
            .truncate_push(height, satblocks_destroyed)?;
        self.satdays_destroyed
            .truncate_push(height, satdays_destroyed)?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.sent.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.satblocks_destroyed as &mut dyn AnyStoredVec,
            &mut self.satdays_destroyed as &mut dyn AnyStoredVec,
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
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.sent.base.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent.base.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.satblocks_destroyed.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.satblocks_destroyed)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.satdays_destroyed.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.satdays_destroyed)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = blocks.count.window_starts();

        // 14-day rolling average of sent (sats and dollars)
        self.sent_14d_ema.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent.base.sats.height,
            &self.sent.base.usd.height,
            exit,
        )?;

        self.coinblocks_destroyed.compute(starting_indexes.height, &window_starts, exit, |v| {
            v.compute_transform(
                starting_indexes.height,
                &self.satblocks_destroyed,
                |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                exit,
            )?;
            Ok(())
        })?;

        self.coindays_destroyed.compute(starting_indexes.height, &window_starts, exit, |v| {
            v.compute_transform(
                starting_indexes.height,
                &self.satdays_destroyed,
                |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                exit,
            )?;
            Ok(())
        })?;

        Ok(())
    }
}
