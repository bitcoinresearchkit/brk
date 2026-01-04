use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod, DCA_CLASS_NAMES, DCA_PERIOD_NAMES, Vecs};
use crate::{
    indexes,
    internal::{
        ComputedValueVecsFromDateIndex, ComputedVecsFromDateIndex, LazyVecsFrom2FromDateIndex,
        PercentageDiffCloseDollars, Source, VecBuilderOptions,
    },
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
    ) -> Result<Self> {
        let last = VecBuilderOptions::default().add_last();

        // DCA by period - stack
        let period_stack = ByDcaPeriod::try_new(|name, _days| {
            ComputedValueVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_dca_stack"),
                Source::Compute,
                version,
                last,
                true,
                indexes,
            )
        })?;

        // DCA by period - avg price
        let period_avg_price = ByDcaPeriod::try_new(|name, _days| {
            ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_dca_avg_price"),
                Source::Compute,
                version,
                indexes,
                last,
            )
        })?;

        // DCA by period - returns (lazy, derived from price and avg_price)
        let period_returns =
            DCA_PERIOD_NAMES
                .zip_ref(&period_avg_price)
                .map(|(name, avg_price)| {
                    LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                        &format!("{name}_dca_returns"),
                        version,
                        &price.usd.timeindexes_to_price_close,
                        avg_price,
                    )
                });

        // DCA by period - CAGR
        let period_cagr = ByDcaCagr::try_new(|name, _days| {
            ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_dca_cagr"),
                Source::Compute,
                version,
                indexes,
                last,
            )
        })?;

        // Lump sum by period - stack
        let period_lump_sum_stack = ByDcaPeriod::try_new(|name, _days| {
            ComputedValueVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_lump_sum_stack"),
                Source::Compute,
                version,
                last,
                true,
                indexes,
            )
        })?;

        // DCA by year class - stack
        let class_stack = ByDcaClass::try_new(|name, _year, _dateindex| {
            ComputedValueVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_stack"),
                Source::Compute,
                version,
                last,
                true,
                indexes,
            )
        })?;

        // DCA by year class - avg price
        let class_avg_price = ByDcaClass::try_new(|name, _year, _dateindex| {
            ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_avg_price"),
                Source::Compute,
                version,
                indexes,
                last,
            )
        })?;

        // DCA by year class - returns (lazy)
        let class_returns = DCA_CLASS_NAMES
            .zip_ref(&class_avg_price)
            .map(|(name, avg_price)| {
                LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                    &format!("{name}_returns"),
                    version,
                    &price.usd.timeindexes_to_price_close,
                    avg_price,
                )
            });

        Ok(Self {
            period_stack,
            period_avg_price,
            period_returns,
            period_cagr,
            period_lump_sum_stack,
            class_stack,
            class_avg_price,
            class_returns,
        })
    }
}
