use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod, Vecs};
use crate::{
    indexes,
    internal::{ComputedFromHeight, Price, ValueFromHeight},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let period_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueFromHeight::forced_import(db, &format!("{name}_dca_stack"), version, indexes)
        })?;

        let period_average_price = ByDcaPeriod::try_new(|name, _days| {
            Price::forced_import(db, &format!("{name}_dca_average_price"), version, indexes)
        })?;

        let period_returns = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeight::forced_import(db, &format!("{name}_dca_returns"), version, indexes)
        })?;

        let period_cagr = ByDcaCagr::try_new(|name, _days| {
            ComputedFromHeight::forced_import(db, &format!("{name}_dca_cagr"), version, indexes)
        })?;

        let period_lump_sum_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueFromHeight::forced_import(db, &format!("{name}_lump_sum_stack"), version, indexes)
        })?;

        let period_lump_sum_returns = ByDcaPeriod::try_new(|name, _days| {
            ComputedFromHeight::forced_import(
                db,
                &format!("{name}_lump_sum_returns"),
                version,
                indexes,
            )
        })?;

        let class_stack = ByDcaClass::try_new(|name, _year, _day1| {
            ValueFromHeight::forced_import(db, &format!("{name}_stack"), version, indexes)
        })?;

        let class_average_price = ByDcaClass::try_new(|name, _year, _day1| {
            Price::forced_import(db, &format!("{name}_average_price"), version, indexes)
        })?;

        let class_returns = ByDcaClass::try_new(|name, _year, _day1| {
            ComputedFromHeight::forced_import(db, &format!("{name}_returns"), version, indexes)
        })?;

        Ok(Self {
            dca_sats_per_day: ImportableVec::forced_import(db, "dca_sats_per_day", version)?,
            period_stack,
            period_average_price,
            period_returns,
            period_cagr,
            period_lump_sum_stack,
            period_lump_sum_returns,
            class_stack,
            class_average_price,
            class_returns,
        })
    }
}
