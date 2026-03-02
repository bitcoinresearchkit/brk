use brk_error::Result;
use brk_types::{Height, StoredF32};
use vecdb::{Exit, ReadableVec};

use super::RsiChain;
use crate::{ComputeIndexes, blocks};

pub(super) fn compute(
    chain: &mut RsiChain,
    blocks: &blocks::Vecs,
    returns_source: &impl ReadableVec<Height, StoredF32>,
    rma_days: usize,
    stoch_sma_days: usize,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    let ws_rma = blocks.count.start_vec(rma_days);
    let ws_sma = blocks.count.start_vec(stoch_sma_days);

    // Gains = max(return, 0)
    chain.gains.height.compute_transform(
        starting_indexes.height,
        returns_source,
        |(h, r, ..)| (h, StoredF32::from((*r).max(0.0))),
        exit,
    )?;

    // Losses = max(-return, 0)
    chain.losses.height.compute_transform(
        starting_indexes.height,
        returns_source,
        |(h, r, ..)| (h, StoredF32::from((-*r).max(0.0))),
        exit,
    )?;

    // Average gain = RMA of gains
    chain.average_gain.height.compute_rolling_rma(
        starting_indexes.height,
        ws_rma,
        &chain.gains.height,
        exit,
    )?;

    // Average loss = RMA of losses
    chain.average_loss.height.compute_rolling_rma(
        starting_indexes.height,
        ws_rma,
        &chain.losses.height,
        exit,
    )?;

    // RSI = 100 * avg_gain / (avg_gain + avg_loss)
    chain.rsi.height.compute_transform2(
        starting_indexes.height,
        &chain.average_gain.height,
        &chain.average_loss.height,
        |(h, g, l, ..)| {
            let sum = *g + *l;
            let rsi = if sum == 0.0 { 50.0 } else { 100.0 * *g / sum };
            (h, StoredF32::from(rsi))
        },
        exit,
    )?;

    // Rolling min/max of RSI over rma_days window
    chain.rsi_min.height.compute_rolling_min_from_starts(
        starting_indexes.height,
        ws_rma,
        &chain.rsi.height,
        exit,
    )?;

    chain.rsi_max.height.compute_rolling_max_from_starts(
        starting_indexes.height,
        ws_rma,
        &chain.rsi.height,
        exit,
    )?;

    // StochRSI = (rsi - rsi_min) / (rsi_max - rsi_min) * 100
    chain.stoch_rsi.height.compute_transform3(
        starting_indexes.height,
        &chain.rsi.height,
        &chain.rsi_min.height,
        &chain.rsi_max.height,
        |(h, r, mn, mx, ..)| {
            let range = *mx - *mn;
            let stoch = if range == 0.0 {
                StoredF32::NAN
            } else {
                StoredF32::from((*r - *mn) / range * 100.0)
            };
            (h, stoch)
        },
        exit,
    )?;

    // StochRSI K = SMA of StochRSI
    chain.stoch_rsi_k.height.compute_rolling_average(
        starting_indexes.height,
        ws_sma,
        &chain.stoch_rsi.height,
        exit,
    )?;

    // StochRSI D = SMA of K
    chain.stoch_rsi_d.height.compute_rolling_average(
        starting_indexes.height,
        ws_sma,
        &chain.stoch_rsi_k.height,
        exit,
    )?;

    Ok(())
}
