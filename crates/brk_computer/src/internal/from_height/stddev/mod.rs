mod extended;

pub use extended::*;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{ComputeIndexes, blocks, indexes};

use crate::internal::ComputedFromHeight;

fn period_suffix(period: &str) -> String {
    if period.is_empty() {
        String::new()
    } else {
        format!("_{period}")
    }
}

#[derive(Traversable)]
pub struct ComputedFromHeightStdDev<M: StorageMode = Rw> {
    days: usize,
    pub sma: ComputedFromHeight<StoredF32, M>,
    pub sd: ComputedFromHeight<StoredF32, M>,
}

impl ComputedFromHeightStdDev {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        period: &str,
        days: usize,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = parent_version + Version::TWO;
        let p = period_suffix(period);

        let sma = ComputedFromHeight::forced_import(
            db,
            &format!("{name}_sma{p}"),
            version,
            indexes,
        )?;
        let sd = ComputedFromHeight::forced_import(
            db,
            &format!("{name}_sd{p}"),
            version,
            indexes,
        )?;

        Ok(Self { days, sma, sd })
    }

    pub(crate) fn compute_all(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        source: &impl ReadableVec<Height, StoredF32>,
    ) -> Result<()> {
        let window_starts = blocks.count.start_vec(self.days);

        self.sma.height.compute_rolling_average(
            starting_indexes.height,
            window_starts,
            source,
            exit,
        )?;

        self.sd.height.compute_rolling_sd(
            starting_indexes.height,
            window_starts,
            source,
            &self.sma.height,
            exit,
        )?;

        Ok(())
    }
}
