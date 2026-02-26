use brk_error::Result;
use brk_types::{Day1, StoredF32};
use vecdb::{AnyStoredVec, AnyVec, Exit, VecIndex, WritableVec};

use super::{MacdChain, smoothing::compute_ema, timeframe::{collect_closes, date_to_period}};
use crate::{ComputeIndexes, prices};

#[allow(clippy::too_many_arguments)]
pub(super) fn compute(
    chain: &mut MacdChain,
    tf: &str,
    prices: &prices::Vecs,
    h2d: &[Day1],
    total_heights: usize,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    let source_version = prices.usd.price.version();

    chain
        .line
        .height
        .validate_computed_version_or_reset(source_version)?;
    chain
        .signal
        .height
        .validate_computed_version_or_reset(source_version)?;

    chain.line.height.truncate_if_needed_at(
        chain
            .line
            .height
            .len()
            .min(starting_indexes.height.to_usize()),
    )?;
    chain.signal.height.truncate_if_needed_at(
        chain
            .signal
            .height
            .len()
            .min(starting_indexes.height.to_usize()),
    )?;

    chain
        .histogram
        .height
        .validate_computed_version_or_reset(source_version)?;
    chain.histogram.height.truncate_if_needed_at(
        chain
            .histogram
            .height
            .len()
            .min(starting_indexes.height.to_usize()),
    )?;

    let start_height = chain.line.height.len();
    if start_height >= total_heights {
        return Ok(());
    }

    // Collect close prices at timeframe level
    let closes = collect_closes(tf, prices);
    let closes_f32: Vec<f32> = closes.iter().map(|d| **d as f32).collect();

    // Compute MACD in-memory
    let ema12 = compute_ema(&closes_f32, 12);
    let ema26 = compute_ema(&closes_f32, 26);

    let macd_line: Vec<f32> = ema12.iter().zip(ema26.iter()).map(|(a, b)| a - b).collect();

    let macd_signal = compute_ema(&macd_line, 9);

    let macd_histogram: Vec<f32> = macd_line.iter().zip(macd_signal.iter()).map(|(a, b)| a - b).collect();

    // Expand to Height
    (start_height..total_heights).for_each(|h| {
        let pi = date_to_period(tf, h2d[h]);
        chain.line.height.push(if pi < macd_line.len() {
            StoredF32::from(macd_line[pi])
        } else {
            StoredF32::NAN
        });
        chain.signal.height.push(if pi < macd_signal.len() {
            StoredF32::from(macd_signal[pi])
        } else {
            StoredF32::NAN
        });
        chain.histogram.height.push(if pi < macd_histogram.len() {
            StoredF32::from(macd_histogram[pi])
        } else {
            StoredF32::NAN
        });
    });

    {
        let _lock = exit.lock();
        chain.line.height.write()?;
        chain.signal.height.write()?;
        chain.histogram.height.write()?;
    }

    Ok(())
}
