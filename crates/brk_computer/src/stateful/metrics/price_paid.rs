//! Price paid metrics and percentiles.
//!
//! Tracks min/max price paid for UTXOs and price distribution percentiles.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, EagerVec, Exit, GenericStoredVec, ImportableVec, PcoVec};

use crate::{
    Indexes,
    grouped::{ComputedVecsFromHeight, PricePercentiles, Source, VecBuilderOptions},
    stateful::states::CohortState,
};

use super::ImportConfig;

/// Price paid metrics.
#[derive(Clone, Traversable)]
pub struct PricePaidMetrics {
    /// Minimum price paid for any UTXO at this height
    pub height_to_min_price_paid: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_min_price_paid: ComputedVecsFromHeight<Dollars>,

    /// Maximum price paid for any UTXO at this height
    pub height_to_max_price_paid: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_max_price_paid: ComputedVecsFromHeight<Dollars>,

    /// Price distribution percentiles (median, quartiles, etc.)
    pub price_percentiles: Option<PricePercentiles>,
}

impl PricePaidMetrics {
    /// Import price paid metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let extended = cfg.extended();
        let last = VecBuilderOptions::default().add_last();

        Ok(Self {
            height_to_min_price_paid: EagerVec::forced_import(
                cfg.db,
                &cfg.name("min_price_paid"),
                cfg.version + v0,
            )?,
            indexes_to_min_price_paid: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("min_price_paid"),
                Source::None,
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,
            height_to_max_price_paid: EagerVec::forced_import(
                cfg.db,
                &cfg.name("max_price_paid"),
                cfg.version + v0,
            )?,
            indexes_to_max_price_paid: ComputedVecsFromHeight::forced_import(
                cfg.db,
                &cfg.name("max_price_paid"),
                Source::None,
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,
            price_percentiles: extended
                .then(|| {
                    PricePercentiles::forced_import(
                        cfg.db,
                        &cfg.name(""),
                        cfg.version + v0,
                        cfg.indexes,
                        true,
                    )
                })
                .transpose()?,
        })
    }

    /// Push min/max price paid from state.
    pub fn truncate_push_minmax(&mut self, height: Height, state: &CohortState) -> Result<()> {
        self.height_to_min_price_paid.truncate_push(
            height,
            state
                .price_to_amount_first_key_value()
                .map(|(&dollars, _)| dollars)
                .unwrap_or(Dollars::NAN),
        )?;
        self.height_to_max_price_paid.truncate_push(
            height,
            state
                .price_to_amount_last_key_value()
                .map(|(&dollars, _)| dollars)
                .unwrap_or(Dollars::NAN),
        )?;
        Ok(())
    }

    /// Push price percentiles from state at date boundary.
    /// Only called when at the last height of a day.
    pub fn truncate_push_percentiles(
        &mut self,
        dateindex: DateIndex,
        state: &CohortState,
    ) -> Result<()> {
        if let Some(price_percentiles) = self.price_percentiles.as_mut() {
            let percentile_prices = state.compute_percentile_prices();
            price_percentiles.truncate_push(dateindex, &percentile_prices)?;
        }
        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.height_to_min_price_paid.write()?;
        self.height_to_max_price_paid.write()?;
        if let Some(price_percentiles) = self.price_percentiles.as_mut() {
            price_percentiles.write()?;
        }
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = vec![
            &mut self.height_to_min_price_paid,
            &mut self.height_to_max_price_paid,
        ];
        if let Some(pp) = self.price_percentiles.as_mut() {
            vecs.extend(
                pp.vecs
                    .iter_mut()
                    .flatten()
                    .filter_map(|v| v.dateindex.as_mut())
                    .map(|v| v as &mut dyn AnyStoredVec),
            );
        }
        vecs.into_par_iter()
    }

    /// Validate computed versions or reset if mismatched.
    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        if let Some(price_percentiles) = self.price_percentiles.as_mut() {
            price_percentiles.validate_computed_version_or_reset(base_version)?;
        }
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_min_price_paid.compute_min_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_min_price_paid)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_max_price_paid.compute_max_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_max_price_paid)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub fn compute_rest_part1(
        &mut self,
        indexes: &crate::indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_min_price_paid.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_min_price_paid),
        )?;

        self.indexes_to_max_price_paid.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_max_price_paid),
        )?;

        Ok(())
    }
}
