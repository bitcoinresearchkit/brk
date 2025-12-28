use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Sats, StoredF64, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableCloneableVec,
    PcoVec,
};

use crate::{
    Indexes,
    grouped::{ComputedValueVecsFromHeight, ComputedVecsFromHeight, Source, VecBuilderOptions},
    indexes, price,
};

use super::ImportConfig;

/// Activity metrics for a cohort.
#[derive(Clone, Traversable)]
pub struct ActivityMetrics {
    /// Total satoshis sent at each height
    pub height_to_sent: EagerVec<PcoVec<Height, Sats>>,

    /// Sent amounts indexed by various dimensions
    pub indexes_to_sent: ComputedValueVecsFromHeight,

    /// Satoshi-blocks destroyed (supply * blocks_old when spent)
    pub height_to_satblocks_destroyed: EagerVec<PcoVec<Height, Sats>>,

    /// Satoshi-days destroyed (supply * days_old when spent)
    pub height_to_satdays_destroyed: EagerVec<PcoVec<Height, Sats>>,

    /// Coin-blocks destroyed (in BTC rather than sats)
    pub indexes_to_coinblocks_destroyed: ComputedVecsFromHeight<StoredF64>,

    /// Coin-days destroyed (in BTC rather than sats)
    pub indexes_to_coindays_destroyed: ComputedVecsFromHeight<StoredF64>,
}

impl ActivityMetrics {
    /// Import activity metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let compute_dollars = cfg.compute_dollars();
        let sum_cum = VecBuilderOptions::default().add_sum().add_cumulative();

        let height_to_sent: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(cfg.db, &cfg.name("sent"), cfg.version + v0)?;
        let indexes_to_sent = ComputedValueVecsFromHeight::forced_import(
            cfg.db,
            &cfg.name("sent"),
            Source::Vec(height_to_sent.boxed_clone()),
            cfg.version + v0,
            sum_cum,
            compute_dollars,
            cfg.indexes,
        )?;

        Ok(Self {
            height_to_sent,
            indexes_to_sent,

            height_to_satblocks_destroyed: EagerVec::forced_import(
                cfg.db,
                &cfg.name("satblocks_destroyed"),
                cfg.version + v0,
            )?,

            height_to_satdays_destroyed: EagerVec::forced_import(
                cfg.db,
                &cfg.name("satdays_destroyed"),
                cfg.version + v0,
            )?,

            indexes_to_coinblocks_destroyed: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("coinblocks_destroyed"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                sum_cum,
            )?,

            indexes_to_coindays_destroyed: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("coindays_destroyed"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                sum_cum,
            )?,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub fn min_len(&self) -> usize {
        self.height_to_sent
            .len()
            .min(self.height_to_satblocks_destroyed.len())
            .min(self.height_to_satdays_destroyed.len())
    }

    /// Push activity state values to height-indexed vectors.
    pub fn truncate_push(
        &mut self,
        height: Height,
        sent: Sats,
        satblocks_destroyed: Sats,
        satdays_destroyed: Sats,
    ) -> Result<()> {
        self.height_to_sent.truncate_push(height, sent)?;
        self.height_to_satblocks_destroyed
            .truncate_push(height, satblocks_destroyed)?;
        self.height_to_satdays_destroyed
            .truncate_push(height, satdays_destroyed)?;
        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.height_to_sent.write()?;
        self.height_to_satblocks_destroyed.write()?;
        self.height_to_satdays_destroyed.write()?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.height_to_sent as &mut dyn AnyStoredVec,
            &mut self.height_to_satblocks_destroyed as &mut dyn AnyStoredVec,
            &mut self.height_to_satdays_destroyed as &mut dyn AnyStoredVec,
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
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_sent.compute_sum_of_others(
            starting_indexes.height,
            &others.iter().map(|v| &v.height_to_sent).collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_satblocks_destroyed.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_satblocks_destroyed)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_satdays_destroyed.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_satdays_destroyed)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_sent.compute_rest(
            indexes,
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_sent),
        )?;

        self.indexes_to_coinblocks_destroyed
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_satblocks_destroyed,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_coindays_destroyed
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.height_to_satdays_destroyed,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
