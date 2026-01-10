//! OHLC computed aggregations combining height, dateindex, and period indexes.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Close, High, Low, Open, Version};
use schemars::JsonSchema;
use vecdb::Database;

use crate::indexes;
use crate::internal::{
    ComputedFromHeightAndDateFirst, ComputedFromHeightAndDateLast, ComputedFromHeightAndDateMax, ComputedFromHeightAndDateMin,
    ComputedVecValue,
};

/// Combined OHLC computed vecs with all indexes (height + dateindex + periods + difficultyepoch).
///
/// Access pattern: `ohlc.{open,high,low,close}.{height,dateindex,weekindex,...,difficultyepoch}`
#[derive(Clone, Traversable)]
pub struct ComputedOHLC<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema + From<f64>,
    f64: From<T>,
{
    pub open: ComputedFromHeightAndDateFirst<Open<T>>,
    pub high: ComputedFromHeightAndDateMax<High<T>>,
    pub low: ComputedFromHeightAndDateMin<Low<T>>,
    pub close: ComputedFromHeightAndDateLast<Close<T>>,
}

impl<T> ComputedOHLC<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema + From<f64> + 'static,
    f64: From<T>,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            open: ComputedFromHeightAndDateFirst::forced_import(
                db,
                &format!("{name}_open"),
                version,
                indexes,
            )?,
            high: ComputedFromHeightAndDateMax::forced_import_raw(
                db,
                &format!("{name}_high"),
                version,
                indexes,
            )?,
            low: ComputedFromHeightAndDateMin::forced_import_raw(
                db,
                &format!("{name}_low"),
                version,
                indexes,
            )?,
            close: ComputedFromHeightAndDateLast::forced_import(
                db,
                &format!("{name}_close"),
                version,
                indexes,
            )?,
        })
    }
}
