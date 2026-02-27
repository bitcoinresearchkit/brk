use brk_error::Result;
use brk_types::{Day1, StoredF32};
use vecdb::{AnyStoredVec, AnyVec, Exit, VecIndex, WritableVec};

use super::{
    RsiChain,
    smoothing::{compute_rma, compute_rolling_max, compute_rolling_min, compute_sma},
    timeframe::{collect_returns, date_to_period},
};
use crate::{ComputeIndexes, market::returns::Vecs as ReturnsVecs};

#[allow(clippy::too_many_arguments)]
pub(super) fn compute(
    chain: &mut RsiChain,
    tf: &str,
    returns: &ReturnsVecs,
    h2d: &[Day1],
    total_heights: usize,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    let source_version = returns.price_returns._24h.height.version();

    let vecs = [
        &mut chain.gains.height,
        &mut chain.losses.height,
        &mut chain.average_gain.height,
        &mut chain.average_loss.height,
        &mut chain.rsi.height,
        &mut chain.rsi_min.height,
        &mut chain.rsi_max.height,
        &mut chain.stoch_rsi.height,
        &mut chain.stoch_rsi_k.height,
        &mut chain.stoch_rsi_d.height,
    ];

    for v in vecs {
        v.validate_computed_version_or_reset(source_version)?;
        v.truncate_if_needed_at(v.len().min(starting_indexes.height.to_usize()))?;
    }

    let start_height = chain.gains.height.len();
    if start_height >= total_heights {
        return Ok(());
    }

    // Collect returns at the appropriate timeframe level
    let period_returns = collect_returns(tf, returns);

    // Compute in-memory
    let gains: Vec<f32> = period_returns.iter().map(|r| r.max(0.0)).collect();
    let losses: Vec<f32> = period_returns.iter().map(|r| (-r).max(0.0)).collect();
    let avg_gain = compute_rma(&gains, 14);
    let avg_loss = compute_rma(&losses, 14);

    let rsi: Vec<f32> = avg_gain
        .iter()
        .zip(avg_loss.iter())
        .map(|(g, l)| {
            let sum = g + l;
            if sum == 0.0 { 50.0 } else { 100.0 * g / sum }
        })
        .collect();

    let rsi_min = compute_rolling_min(&rsi, 14);
    let rsi_max = compute_rolling_max(&rsi, 14);

    let stoch_rsi: Vec<f32> = rsi
        .iter()
        .zip(rsi_min.iter())
        .zip(rsi_max.iter())
        .map(|((r, mn), mx)| {
            let range = mx - mn;
            if range == 0.0 {
                50.0
            } else {
                (r - mn) / range * 100.0
            }
        })
        .collect();

    let stoch_rsi_k = compute_sma(&stoch_rsi, 3);
    let stoch_rsi_d = compute_sma(&stoch_rsi_k, 3);

    // Expand to Height
    macro_rules! expand {
        ($target:expr, $buffer:expr) => {
            for h in start_height..total_heights {
                let pi = date_to_period(tf, h2d[h]);
                let val = if pi < $buffer.len() {
                    StoredF32::from($buffer[pi])
                } else {
                    StoredF32::NAN
                };
                $target.push(val);
            }
        };
    }

    expand!(chain.gains.height, gains);
    expand!(chain.losses.height, losses);
    expand!(chain.average_gain.height, avg_gain);
    expand!(chain.average_loss.height, avg_loss);
    expand!(chain.rsi.height, rsi);
    expand!(chain.rsi_min.height, rsi_min);
    expand!(chain.rsi_max.height, rsi_max);
    expand!(chain.stoch_rsi.height, stoch_rsi);
    expand!(chain.stoch_rsi_k.height, stoch_rsi_k);
    expand!(chain.stoch_rsi_d.height, stoch_rsi_d);

    {
        let _lock = exit.lock();
        chain.gains.height.write()?;
        chain.losses.height.write()?;
        chain.average_gain.height.write()?;
        chain.average_loss.height.write()?;
        chain.rsi.height.write()?;
        chain.rsi_min.height.write()?;
        chain.rsi_max.height.write()?;
        chain.stoch_rsi.height.write()?;
        chain.stoch_rsi_k.height.write()?;
        chain.stoch_rsi_d.height.write()?;
    }

    Ok(())
}
