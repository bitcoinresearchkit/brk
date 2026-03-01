mod extended;
mod extension;
mod price_extended;

pub use extended::*;
pub use extension::*;
pub use price_extended::*;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, StoredF32, Version};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{ComputeIndexes, indexes};

use super::ComputedFromHeight;

#[derive(Traversable)]
pub struct ComputedFromHeightRatio<M: StorageMode = Rw> {
    pub ratio: ComputedFromHeight<StoredF32, M>,
}

const VERSION: Version = Version::TWO;

impl ComputedFromHeightRatio {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        Ok(Self {
            ratio: ComputedFromHeight::forced_import(db, &format!("{name}_ratio"), v, indexes)?,
        })
    }

    /// Compute ratio = close_price / metric_price at height level (both in cents)
    pub(crate) fn compute_ratio(
        &mut self,
        starting_indexes: &ComputeIndexes,
        close_price: &impl ReadableVec<Height, Cents>,
        metric_price: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self.ratio.height.compute_transform2(
            starting_indexes.height,
            close_price,
            metric_price,
            |(i, close, price, ..)| {
                if price == Cents::ZERO {
                    (i, StoredF32::from(1.0))
                } else {
                    (i, StoredF32::from(f64::from(close) / f64::from(price)))
                }
            },
            exit,
        )?;
        Ok(())
    }
}
