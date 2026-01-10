//! OHLC computed aggregations combining height, dateindex, and period indexes.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Close, High, Low, Open, Version};
use schemars::JsonSchema;
use vecdb::Database;

use crate::indexes;
use crate::internal::{
    ComputedHeightDateFirst, ComputedHeightDateLast, ComputedHeightDateMax, ComputedHeightDateMin,
    ComputedVecValue,
};

/// Combined OHLC computed vecs with all indexes (height + dateindex + periods + difficultyepoch).
///
/// Access pattern: `ohlc.{open,high,low,close}.{height,dateindex,weekindex,...,difficultyepoch}`
#[derive(Clone, Traversable)]
pub struct OHLCComputedVecs<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema + From<f64>,
    f64: From<T>,
{
    pub open: ComputedHeightDateFirst<Open<T>>,
    pub high: ComputedHeightDateMax<High<T>>,
    pub low: ComputedHeightDateMin<Low<T>>,
    pub close: ComputedHeightDateLast<Close<T>>,
}

impl<T> OHLCComputedVecs<T>
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
            open: ComputedHeightDateFirst::forced_import(
                db,
                &format!("{name}_open"),
                version,
                indexes,
            )?,
            high: ComputedHeightDateMax::forced_import(
                db,
                &format!("{name}_high"),
                version,
                indexes,
            )?,
            low: ComputedHeightDateMin::forced_import(
                db,
                &format!("{name}_low"),
                version,
                indexes,
            )?,
            close: ComputedHeightDateLast::forced_import(
                db,
                &format!("{name}_close"),
                version,
                indexes,
            )?,
        })
    }
}
