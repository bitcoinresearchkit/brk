use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ImportableVec};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod, Vecs};
use super::vecs::{ClassVecs, PeriodVecs};
use crate::{
    indexes,
    internal::{AmountPerBlock, PercentPerBlock, Price},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let stack = ByDcaPeriod::try_new(|name, _days| {
            AmountPerBlock::forced_import(db, &format!("dca_stack_{name}"), version, indexes)
        })?;

        let cost_basis = ByDcaPeriod::try_new(|name, _days| {
            Price::forced_import(db, &format!("dca_cost_basis_{name}"), version, indexes)
        })?;

        let r#return = ByDcaPeriod::try_new(|name, _days| {
            PercentPerBlock::forced_import(db, &format!("dca_return_{name}"), version, indexes)
        })?;

        let cagr = ByDcaCagr::try_new(|name, _days| {
            PercentPerBlock::forced_import(db, &format!("dca_cagr_{name}"), version, indexes)
        })?;

        let lump_sum_stack = ByDcaPeriod::try_new(|name, _days| {
            AmountPerBlock::forced_import(db, &format!("lump_sum_stack_{name}"), version, indexes)
        })?;

        let lump_sum_return = ByDcaPeriod::try_new(|name, _days| {
            PercentPerBlock::forced_import(
                db,
                &format!("lump_sum_return_{name}"),
                version,
                indexes,
            )
        })?;

        let class_stack = ByDcaClass::try_new(|name, _year, _day1| {
            AmountPerBlock::forced_import(db, &format!("dca_stack_{name}"), version, indexes)
        })?;

        let class_cost_basis = ByDcaClass::try_new(|name, _year, _day1| {
            Price::forced_import(db, &format!("dca_cost_basis_{name}"), version, indexes)
        })?;

        let class_return = ByDcaClass::try_new(|name, _year, _day1| {
            PercentPerBlock::forced_import(db, &format!("dca_return_{name}"), version, indexes)
        })?;

        Ok(Self {
            sats_per_day: ImportableVec::forced_import(db, "dca_sats_per_day", version)?,
            period: PeriodVecs {
                stack,
                cost_basis,
                r#return,
                cagr,
                lump_sum_stack,
                lump_sum_return,
            },
            class: ClassVecs {
                stack: class_stack,
                cost_basis: class_cost_basis,
                r#return: class_return,
            },
        })
    }
}
