use std::iter;

use serde::Serialize;

use crate::{
    AGE_RANGE_NAMES, AMOUNT_RANGE_NAMES, CLASS_NAMES, ENTRY_NAMES, EPOCH_NAMES, LOSS_NAMES,
    PROFIT_NAMES, PROFITABILITY_RANGE_NAMES, SPENDABLE_TYPE_NAMES, TERM_NAMES, UTXO_ALL_NAME,
};

/// Display names for a cohort with id (for storage/API), short (for charts), and long (for tooltips/labels)
#[derive(Clone, Copy, Serialize)]
pub struct CohortName {
    pub id: &'static str,
    pub short: &'static str,
    pub long: &'static str,
}

impl CohortName {
    pub const fn new(id: &'static str, short: &'static str, long: &'static str) -> Self {
        Self { id, short, long }
    }

    /// Every canonical cohort name declared by this crate.
    pub fn all() -> impl Iterator<Item = &'static Self> {
        iter::once(&UTXO_ALL_NAME)
            .chain(TERM_NAMES.iter())
            .chain(AGE_RANGE_NAMES.iter())
            .chain(AMOUNT_RANGE_NAMES.iter())
            .chain(EPOCH_NAMES.iter())
            .chain(CLASS_NAMES.iter())
            .chain(ENTRY_NAMES.iter())
            .chain(SPENDABLE_TYPE_NAMES.iter())
            .chain(PROFIT_NAMES.iter())
            .chain(LOSS_NAMES.iter())
            .chain(PROFITABILITY_RANGE_NAMES.iter())
    }
}
