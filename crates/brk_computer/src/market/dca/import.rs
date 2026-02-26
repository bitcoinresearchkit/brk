use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod, Vecs};
use crate::{
    indexes,
    internal::{
        ComputedFromHeightLast, Price,
        ValueFromHeightLast,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        // DCA by period - stack (KISS)
        let period_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueFromHeightLast::forced_import(
                db,
                &format!("{name}_dca_stack"),
                version,
                indexes,
            )
        })?;

        // DCA by period - average price
        let period_average_price = ByDcaPeriod::try_new(|name, _days| {
            Price::forced_import(
                db,
                &format!("{name}_dca_average_price"),
                version,
                indexes,
            )
        })?;

        let period_returns = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_dca_returns"),
                version,
                indexes,
            )
        })?;

        // DCA by period - CAGR
        let period_cagr = ByDcaCagr::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(db, &format!("{name}_dca_cagr"), version, indexes)
        })?;

        // DCA by period - profitability
        let period_days_in_profit = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_dca_days_in_profit"),
                version + Version::ONE,
                indexes,
            )
        })?;

        let period_days_in_loss = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_dca_days_in_loss"),
                version + Version::ONE,
                indexes,
            )
        })?;

        let period_min_return = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_dca_min_return"),
                version,
                indexes,
            )
        })?;

        let period_max_return = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_dca_max_return"),
                version,
                indexes,
            )
        })?;

        // Lump sum by period - stack (KISS)
        let period_lump_sum_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueFromHeightLast::forced_import(
                db,
                &format!("{name}_lump_sum_stack"),
                version,
                indexes,
            )
        })?;

        // Lump sum by period - returns
        let period_lump_sum_returns = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_lump_sum_returns"),
                version,
                indexes,
            )
        })?;

        // Lump sum by period - profitability
        let period_lump_sum_days_in_profit = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_lump_sum_days_in_profit"),
                version + Version::ONE,
                indexes,
            )
        })?;

        let period_lump_sum_days_in_loss = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_lump_sum_days_in_loss"),
                version + Version::ONE,
                indexes,
            )
        })?;

        let period_lump_sum_min_return = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_lump_sum_min_return"),
                version,
                indexes,
            )
        })?;

        let period_lump_sum_max_return = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_lump_sum_max_return"),
                version,
                indexes,
            )
        })?;

        // DCA by year class - stack (KISS)
        let class_stack = ByDcaClass::try_new(|name, _year, _day1| {
            ValueFromHeightLast::forced_import(
                db,
                &format!("{name}_stack"),
                version,
                indexes,
            )
        })?;

        // DCA by year class - average price
        let class_average_price = ByDcaClass::try_new(|name, _year, _day1| {
            Price::forced_import(db, &format!("{name}_average_price"), version, indexes)
        })?;

        let class_returns = ByDcaClass::try_new(|name, _year, _day1| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_returns"),
                version,
                indexes,
            )
        })?;

        // DCA by year class - profitability
        let class_days_in_profit = ByDcaClass::try_new(|name, _year, _day1| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_days_in_profit"),
                version,
                indexes,
            )
        })?;

        let class_days_in_loss = ByDcaClass::try_new(|name, _year, _day1| {
            ComputedFromHeightLast::forced_import(
                db,
                &format!("{name}_days_in_loss"),
                version,
                indexes,
            )
        })?;

        let class_min_return = ByDcaClass::try_new(|name, _year, _day1| {
            ComputedFromHeightLast::forced_import(db, &format!("{name}_min_return"), version, indexes)
        })?;

        let class_max_return = ByDcaClass::try_new(|name, _year, _day1| {
            ComputedFromHeightLast::forced_import(db, &format!("{name}_max_return"), version, indexes)
        })?;

        Ok(Self {
            dca_sats_per_day: ImportableVec::forced_import(db, "dca_sats_per_day", version)?,
            period_stack,
            period_average_price,
            period_returns,
            period_cagr,
            period_days_in_profit,
            period_days_in_loss,
            period_min_return,
            period_max_return,
            period_lump_sum_stack,
            period_lump_sum_returns,
            period_lump_sum_days_in_profit,
            period_lump_sum_days_in_loss,
            period_lump_sum_min_return,
            period_lump_sum_max_return,
            class_stack,
            class_average_price,
            class_returns,
            class_days_in_profit,
            class_days_in_loss,
            class_min_return,
            class_max_return,
        })
    }
}
