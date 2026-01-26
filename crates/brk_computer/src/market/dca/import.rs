use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod, DCA_CLASS_NAMES, DCA_PERIOD_NAMES, Vecs};
use crate::{
    indexes,
    internal::{
        ComputedFromDateLast, LazyBinaryFromDateLast, PercentageDiffCloseDollars, Price,
        ValueFromDateLast,
    },
    market::lookback,
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
        lookback: &lookback::Vecs,
    ) -> Result<Self> {
        // DCA by period - stack (KISS)
        let period_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueFromDateLast::forced_import(db, &format!("{name}_dca_stack"), version, true, indexes)
        })?;

        // DCA by period - average price
        let period_average_price = ByDcaPeriod::try_new(|name, _days| {
            Price::forced_import(db, &format!("{name}_dca_average_price"), version, indexes)
        })?;

        let period_returns =
            DCA_PERIOD_NAMES
                .zip_ref(&period_average_price)
                .map(|(name, average_price)| {
                    LazyBinaryFromDateLast::from_computed_both_last::<PercentageDiffCloseDollars>(
                        &format!("{name}_dca_returns"),
                        version,
                        &price.usd.split.close,
                        average_price,
                    )
                });

        // DCA by period - CAGR
        let period_cagr = ByDcaCagr::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(db, &format!("{name}_dca_cagr"), version, indexes)
        })?;

        // DCA by period - profitability
        let period_days_in_profit = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_dca_days_in_profit"),
                version + Version::ONE,
                indexes,
            )
        })?;

        let period_days_in_loss = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_dca_days_in_loss"),
                version + Version::ONE,
                indexes,
            )
        })?;

        let period_max_drawdown = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_dca_max_drawdown"),
                version,
                indexes,
            )
        })?;

        let period_max_return = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_dca_max_return"),
                version,
                indexes,
            )
        })?;

        // Lump sum by period - stack (KISS)
        let period_lump_sum_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueFromDateLast::forced_import(
                db,
                &format!("{name}_lump_sum_stack"),
                version,
                true,
                indexes,
            )
        })?;

        // Lump sum by period - returns
        let period_lump_sum_returns = DCA_PERIOD_NAMES
            .zip_ref(&lookback.price_ago.as_dca_period())
            .map(|(name, lookback_price)| {
                LazyBinaryFromDateLast::from_derived_last_and_computed_last::<
                    PercentageDiffCloseDollars,
                >(
                    &format!("{name}_lump_sum_returns"),
                    version,
                    price.usd.split.close.dateindex.boxed_clone(),
                    &price.usd.split.close.rest,
                    lookback_price,
                )
            });

        // Lump sum by period - profitability
        let period_lump_sum_days_in_profit = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_lump_sum_days_in_profit"),
                version + Version::ONE,
                indexes,
            )
        })?;

        let period_lump_sum_days_in_loss = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_lump_sum_days_in_loss"),
                version + Version::ONE,
                indexes,
            )
        })?;

        let period_lump_sum_max_drawdown = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_lump_sum_max_drawdown"),
                version,
                indexes,
            )
        })?;

        let period_lump_sum_max_return = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_lump_sum_max_return"),
                version,
                indexes,
            )
        })?;

        // DCA by year class - stack (KISS)
        let class_stack = ByDcaClass::try_new(|name, _year, _dateindex| {
            ValueFromDateLast::forced_import(db, &format!("{name}_stack"), version, true, indexes)
        })?;

        // DCA by year class - average price
        let class_average_price = ByDcaClass::try_new(|name, _year, _dateindex| {
            Price::forced_import(db, &format!("{name}_average_price"), version, indexes)
        })?;

        let class_returns =
            DCA_CLASS_NAMES
                .zip_ref(&class_average_price)
                .map(|(name, average_price)| {
                    LazyBinaryFromDateLast::from_computed_both_last::<PercentageDiffCloseDollars>(
                        &format!("{name}_returns"),
                        version,
                        &price.usd.split.close,
                        average_price,
                    )
                });

        // DCA by year class - profitability
        let class_days_in_profit = ByDcaClass::try_new(|name, _year, _dateindex| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_days_in_profit"),
                version,
                indexes,
            )
        })?;

        let class_days_in_loss = ByDcaClass::try_new(|name, _year, _dateindex| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_days_in_loss"),
                version,
                indexes,
            )
        })?;

        let class_max_drawdown = ByDcaClass::try_new(|name, _year, _dateindex| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_max_drawdown"),
                version,
                indexes,
            )
        })?;

        let class_max_return = ByDcaClass::try_new(|name, _year, _dateindex| {
            ComputedFromDateLast::forced_import(
                db,
                &format!("{name}_max_return"),
                version,
                indexes,
            )
        })?;

        Ok(Self {
            period_stack,
            period_average_price,
            period_returns,
            period_cagr,
            period_days_in_profit,
            period_days_in_loss,
            period_max_drawdown,
            period_max_return,
            period_lump_sum_stack,
            period_lump_sum_returns,
            period_lump_sum_days_in_profit,
            period_lump_sum_days_in_loss,
            period_lump_sum_max_drawdown,
            period_lump_sum_max_return,
            class_stack,
            class_average_price,
            class_returns,
            class_days_in_profit,
            class_days_in_loss,
            class_max_drawdown,
            class_max_return,
        })
    }
}
