//! Price paid metrics and percentiles.
//!
//! Tracks min/max price paid for UTXOs and price distribution percentiles.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Version};
use vecdb::{AnyStoredVec, EagerVec, Exit, GenericStoredVec, ImportableVec, PcoVec};

use crate::{
    Indexes,
    grouped::{ComputedVecsFromHeight, PricePercentiles, Source, VecBuilderOptions},
    stateful::cohorts::CohortState,
    states::Flushable,
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

    /// Push price percentiles from state.
    pub fn truncate_push_percentiles(&mut self, height: Height, state: &CohortState) -> Result<()> {
        if let Some(price_percentiles) = self.price_percentiles.as_mut() {
            let percentile_prices = state.compute_percentile_prices();
            price_percentiles.truncate_push(height, &percentile_prices)?;
        }
        Ok(())
    }

    /// Flush height-indexed vectors to disk.
    pub fn safe_flush(&mut self, exit: &Exit) -> Result<()> {
        self.height_to_min_price_paid.safe_write(exit)?;
        self.height_to_max_price_paid.safe_write(exit)?;
        if let Some(price_percentiles) = self.price_percentiles.as_mut() {
            price_percentiles.safe_flush(exit)?;
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
}
