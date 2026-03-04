use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod, Vecs};
use crate::{
    indexes,
    internal::{PercentFromHeight, Price, ValueFromHeight},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let period_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueFromHeight::forced_import(db, &format!("dca_stack_{name}"), version, indexes)
        })?;

        let period_cost_basis = ByDcaPeriod::try_new(|name, _days| {
            Price::forced_import(db, &format!("dca_cost_basis_{name}"), version, indexes)
        })?;

        let period_return = ByDcaPeriod::try_new(|name, _days| {
            PercentFromHeight::forced_import(
                db,
                &format!("dca_return_{name}"),
                version,
                indexes,
            )
        })?;

        let period_cagr = ByDcaCagr::try_new(|name, _days| {
            PercentFromHeight::forced_import(
                db,
                &format!("dca_cagr_{name}"),
                version,
                indexes,
            )
        })?;

        let period_lump_sum_stack = ByDcaPeriod::try_new(|name, _days| {
            ValueFromHeight::forced_import(db, &format!("lump_sum_stack_{name}"), version, indexes)
        })?;

        let period_lump_sum_return = ByDcaPeriod::try_new(|name, _days| {
            PercentFromHeight::forced_import(
                db,
                &format!("lump_sum_return_{name}"),
                version,
                indexes,
            )
        })?;

        let class_stack = ByDcaClass::try_new(|name, _year, _day1| {
            ValueFromHeight::forced_import(db, &format!("dca_stack_{name}"), version, indexes)
        })?;

        let class_cost_basis = ByDcaClass::try_new(|name, _year, _day1| {
            Price::forced_import(db, &format!("dca_cost_basis_{name}"), version, indexes)
        })?;

        let class_return = ByDcaClass::try_new(|name, _year, _day1| {
            PercentFromHeight::forced_import(
                db,
                &format!("dca_return_{name}"),
                version,
                indexes,
            )
        })?;

        Ok(Self {
            dca_sats_per_day: ImportableVec::forced_import(db, "dca_sats_per_day", version)?,
            period_stack,
            period_cost_basis,
            period_return,
            period_cagr,
            period_lump_sum_stack,
            period_lump_sum_return,
            class_stack,
            class_cost_basis,
            class_return,
        })
    }
}
