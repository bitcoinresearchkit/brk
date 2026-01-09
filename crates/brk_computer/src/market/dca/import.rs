use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod, DCA_CLASS_NAMES, DCA_PERIOD_NAMES, Vecs};
use crate::{
    indexes,
    internal::{ComputedDateLast, LazyBinaryDateLast, PercentageDiffCloseDollars, ValueDateLast},
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
    ) -> Result<Self> {
        // DCA by period - stack (KISS)
        let period_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueDateLast::forced_import(db, &format!("{name}_dca_stack"), version, true, indexes)
        })?;

        // DCA by period - average price
        let period_average_price = ByDcaPeriod::try_new(|name, _days| {
            ComputedDateLast::forced_import(
                db,
                &format!("{name}_dca_average_price"),
                version,
                indexes,
            )
        })?;

        let period_returns =
            DCA_PERIOD_NAMES
                .zip_ref(&period_average_price)
                .map(|(name, average_price)| {
                    LazyBinaryDateLast::from_computed_both_last::<PercentageDiffCloseDollars>(
                        &format!("{name}_dca_returns"),
                        version,
                        &price.usd.split.close,
                        average_price,
                    )
                });

        // DCA by period - CAGR
        let period_cagr = ByDcaCagr::try_new(|name, _days| {
            ComputedDateLast::forced_import(db, &format!("{name}_dca_cagr"), version, indexes)
        })?;

        // Lump sum by period - stack (KISS)
        let period_lump_sum_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueDateLast::forced_import(
                db,
                &format!("{name}_lump_sum_stack"),
                version,
                true,
                indexes,
            )
        })?;

        // DCA by year class - stack (KISS)
        let class_stack = ByDcaClass::try_new(|name, _year, _dateindex| {
            ValueDateLast::forced_import(db, &format!("{name}_stack"), version, true, indexes)
        })?;

        // DCA by year class - average price
        let class_average_price = ByDcaClass::try_new(|name, _year, _dateindex| {
            ComputedDateLast::forced_import(db, &format!("{name}_average_price"), version, indexes)
        })?;

        let class_returns =
            DCA_CLASS_NAMES
                .zip_ref(&class_average_price)
                .map(|(name, average_price)| {
                    LazyBinaryDateLast::from_computed_both_last::<PercentageDiffCloseDollars>(
                        &format!("{name}_returns"),
                        version,
                        &price.usd.split.close,
                        average_price,
                    )
                });

        Ok(Self {
            period_stack,
            period_average_price,
            period_returns,
            period_cagr,
            period_lump_sum_stack,
            class_stack,
            class_average_price,
            class_returns,
        })
    }
}
