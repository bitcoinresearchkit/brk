mod extended;

pub use extended::*;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, Version};
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, ReadableVec, Rw, StorageMode, VecIndex, WritableVec};

use crate::{ComputeIndexes, blocks, indexes};

use crate::internal::ComputedFromHeightLast;

#[derive(Traversable)]
pub struct ComputedFromHeightStdDev<M: StorageMode = Rw> {
    days: usize,
    pub sma: ComputedFromHeightLast<StoredF32, M>,
    pub sd: ComputedFromHeightLast<StoredF32, M>,
}

impl ComputedFromHeightStdDev {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        days: usize,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = parent_version + Version::TWO;

        let sma = ComputedFromHeightLast::forced_import(
            db,
            &format!("{name}_sma"),
            version,
            indexes,
        )?;
        let sd = ComputedFromHeightLast::forced_import(
            db,
            &format!("{name}_sd"),
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
        // 1. Compute SMA using the appropriate lookback vec (or full-history SMA)
        if self.days != usize::MAX {
            let window_starts = blocks.count.start_vec(self.days);
            self.sma.height.compute_rolling_average(
                starting_indexes.height,
                window_starts,
                source,
                exit,
            )?;
        } else {
            // Full history SMA (days == usize::MAX)
            self.sma.height.compute_sma_(
                starting_indexes.height,
                source,
                self.days,
                exit,
                None,
            )?;
        }

        // Split borrows: sd is mutated, sma is read
        compute_sd(
            &mut self.sd,
            blocks,
            starting_indexes,
            exit,
            &self.sma.height,
            source,
        )
    }
}

fn compute_sd(
    sd: &mut ComputedFromHeightLast<StoredF32>,
    blocks: &blocks::Vecs,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
    sma: &impl ReadableVec<Height, StoredF32>,
    source: &impl ReadableVec<Height, StoredF32>,
) -> Result<()> {
    let source_version = source.version();

    sd.height
        .validate_computed_version_or_reset(source_version)?;

    let starting_height = Height::from(sd.height.len()).min(starting_indexes.height);

    let day_start = &blocks.count.height_24h_ago;
    let start = starting_height.to_usize();

    let mut n: usize = 0;
    let mut welford_sum: f64 = 0.0;
    let mut welford_sum_sq: f64 = 0.0;
    if start > 0 {
        let day_start_hist = day_start.collect_range_at(0, start);
        let source_hist = source.collect_range_at(0, start);
        let mut last_ds = Height::from(0_usize);
        for h in 0..start {
            let cur_ds = day_start_hist[h];
            if h == 0 || cur_ds != last_ds {
                let val = *source_hist[h] as f64;
                n += 1;
                welford_sum += val;
                welford_sum_sq += val * val;
                last_ds = cur_ds;
            }
        }
    }

    let source_len = source.len();
    let source_data = source.collect_range_at(start, source_len);
    let sma_data = sma.collect_range_at(start, sma.len());
    let mut last_day_start = if start > 0 {
        day_start
            .collect_one_at(start - 1)
            .unwrap_or(Height::from(0_usize))
    } else {
        Height::from(0_usize)
    };

    let day_start_data = day_start.collect_range_at(start, source_len);

    for (offset, ratio) in source_data.into_iter().enumerate() {
        let index = start + offset;
        let cur_day_start = day_start_data[offset];
        if index == 0 || cur_day_start != last_day_start {
            let val = *ratio as f64;
            n += 1;
            welford_sum += val;
            welford_sum_sq += val * val;
            last_day_start = cur_day_start;
        }

        let average = sma_data[offset];
        let avg_f64 = *average as f64;

        let sd_val = if n > 0 {
            let nf = n as f64;
            let variance =
                welford_sum_sq / nf - 2.0 * avg_f64 * welford_sum / nf + avg_f64 * avg_f64;
            StoredF32::from(variance.max(0.0).sqrt() as f32)
        } else {
            StoredF32::from(0.0_f32)
        };

        sd.height.truncate_push_at(index, sd_val)?;
    }

    {
        let _lock = exit.lock();
        sd.height.flush()?;
    }

    Ok(())
}
