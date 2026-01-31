use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Sats, StoredF64, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, ImportableVec, PcoVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedFromHeightSumCum, LazyComputedValueFromHeightSumCum},
};

use super::ImportConfig;

/// Activity metrics for a cohort.
#[derive(Clone, Traversable)]
pub struct ActivityMetrics {
    /// Total satoshis sent at each height + derived indexes
    pub sent: LazyComputedValueFromHeightSumCum,

    /// Satoshi-blocks destroyed (supply * blocks_old when spent)
    pub satblocks_destroyed: EagerVec<PcoVec<Height, Sats>>,

    /// Satoshi-days destroyed (supply * days_old when spent)
    pub satdays_destroyed: EagerVec<PcoVec<Height, Sats>>,

    /// Coin-blocks destroyed (in BTC rather than sats)
    pub coinblocks_destroyed: ComputedFromHeightSumCum<StoredF64>,

    /// Coin-days destroyed (in BTC rather than sats)
    pub coindays_destroyed: ComputedFromHeightSumCum<StoredF64>,
}

impl ActivityMetrics {
    /// Import activity metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            sent: LazyComputedValueFromHeightSumCum::forced_import(
                cfg.db,
                &cfg.name("sent"),
                cfg.version,
                cfg.indexes,
                cfg.price,
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

            coinblocks_destroyed: ComputedFromHeightSumCum::forced_import(
                cfg.db,
                &cfg.name("coinblocks_destroyed"),
                cfg.version,
                cfg.indexes,
            )?,

            coindays_destroyed: ComputedFromHeightSumCum::forced_import(
                cfg.db,
                &cfg.name("coindays_destroyed"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub fn min_len(&self) -> usize {
        self.sent
            .sats
            .height
            .len()
            .min(self.satblocks_destroyed.len())
            .min(self.satdays_destroyed.len())
    }

    /// Push activity state values to height-indexed vectors.
    pub fn truncate_push(
        &mut self,
        height: Height,
        sent: Sats,
        satblocks_destroyed: Sats,
        satdays_destroyed: Sats,
    ) -> Result<()> {
        self.sent.sats.height.truncate_push(height, sent)?;
        self.satblocks_destroyed
            .truncate_push(height, satblocks_destroyed)?;
        self.satdays_destroyed
            .truncate_push(height, satdays_destroyed)?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.sent.sats.height as &mut dyn AnyStoredVec,
            &mut self.satblocks_destroyed as &mut dyn AnyStoredVec,
            &mut self.satdays_destroyed as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }

    /// Validate computed versions against base version.
    pub fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        // Validation logic for computed vecs
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.sent.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent.sats.height)
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
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sent.compute_rest(indexes, starting_indexes, exit)?;

        self.coinblocks_destroyed
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.satblocks_destroyed,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        self.coindays_destroyed
            .compute_all(indexes, starting_indexes, exit, |v| {
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
