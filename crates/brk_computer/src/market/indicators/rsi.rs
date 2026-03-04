use brk_error::Result;
use brk_types::{BasisPoints16, Height, Indexes, StoredF32};
use vecdb::{Exit, ReadableVec};

use super::RsiChain;
use crate::blocks;

pub(super) fn compute(
    chain: &mut RsiChain,
    blocks: &blocks::Vecs,
    returns_source: &impl ReadableVec<Height, StoredF32>,
    rma_days: usize,
    stoch_sma_days: usize,
    starting_indexes: &Indexes,
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

    // RSI = avg_gain / (avg_gain + avg_loss), stored as ratio (0–1)
    chain.rsi.bps.height.compute_transform2(
        starting_indexes.height,
        &chain.average_gain.height,
        &chain.average_loss.height,
        |(h, g, l, ..)| {
            let sum = *g + *l;
            let rsi = if sum == 0.0 { 0.5 } else { *g / sum };
            (h, BasisPoints16::from(rsi as f64))
        },
        exit,
    )?;

    // Rolling min/max of RSI over rma_days window
    chain.rsi_min.bps.height.compute_rolling_min_from_starts(
        starting_indexes.height,
        ws_rma,
        &chain.rsi.bps.height,
        exit,
    )?;

    chain.rsi_max.bps.height.compute_rolling_max_from_starts(
        starting_indexes.height,
        ws_rma,
        &chain.rsi.bps.height,
        exit,
    )?;

    // StochRSI = (rsi - rsi_min) / (rsi_max - rsi_min), stored as ratio (0–1)
    chain.stoch_rsi.bps.height.compute_transform3(
        starting_indexes.height,
        &chain.rsi.bps.height,
        &chain.rsi_min.bps.height,
        &chain.rsi_max.bps.height,
        |(h, r, mn, mx, ..)| {
            let range = f64::from(*mx) - f64::from(*mn);
            let stoch = if range == 0.0 {
                BasisPoints16::ZERO
            } else {
                BasisPoints16::from((f64::from(*r) - f64::from(*mn)) / range)
            };
            (h, stoch)
        },
        exit,
    )?;

    // StochRSI K = SMA of StochRSI
    chain.stoch_rsi_k.bps.height.compute_rolling_average(
        starting_indexes.height,
        ws_sma,
        &chain.stoch_rsi.bps.height,
        exit,
    )?;

    // StochRSI D = SMA of K
    chain.stoch_rsi_d.bps.height.compute_rolling_average(
        starting_indexes.height,
        ws_sma,
        &chain.stoch_rsi_k.bps.height,
        exit,
    )?;

    Ok(())
}
