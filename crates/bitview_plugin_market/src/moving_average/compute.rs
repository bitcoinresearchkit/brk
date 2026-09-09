use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_price::Vecs as PriceVecs;
use brk_error::Result;
use brk_exit::Exit;

use super::Vecs;

pub fn compute(
    vecs: &mut Vecs,
    indexer: &Indexer,
    blocks: &BlocksVecs,
    prices: &PriceVecs,
    exit: &Exit,
) -> Result<()> {
    let starting_lengths = indexer.safe_lengths();
    let close = &prices.spot.cents.height;
    vecs.sma.clear_if_recomputed_from(starting_lengths.height);

    vecs.ema.height.compute_rolling_ema_columns(
        starting_lengths.height,
        |period| blocks.lookback.start_vec(period.days()),
        close,
        exit,
    )?;

    Ok(())
}
