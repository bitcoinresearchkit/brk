use brk_error::Result;
use vecdb::Exit;

use super::MacdChain;
use crate::{ComputeIndexes, blocks, prices};

#[allow(clippy::too_many_arguments)]
pub(super) fn compute(
    chain: &mut MacdChain,
    blocks: &blocks::Vecs,
    prices: &prices::Vecs,
    fast_days: usize,
    slow_days: usize,
    signal_days: usize,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    let close = &prices.price.usd.height;
    let ws_fast = blocks.count.start_vec(fast_days);
    let ws_slow = blocks.count.start_vec(slow_days);
    let ws_signal = blocks.count.start_vec(signal_days);

    chain
        .ema_fast
        .height
        .compute_rolling_ema(starting_indexes.height, ws_fast, close, exit)?;

    chain
        .ema_slow
        .height
        .compute_rolling_ema(starting_indexes.height, ws_slow, close, exit)?;

    // MACD line = ema_fast - ema_slow
    chain.line.height.compute_subtract(
        starting_indexes.height,
        &chain.ema_fast.height,
        &chain.ema_slow.height,
        exit,
    )?;

    // Signal = EMA of MACD line
    chain.signal.height.compute_rolling_ema(
        starting_indexes.height,
        ws_signal,
        &chain.line.height,
        exit,
    )?;

    // Histogram = line - signal
    chain.histogram.height.compute_subtract(
        starting_indexes.height,
        &chain.line.height,
        &chain.signal.height,
        exit,
    )?;

    Ok(())
}
