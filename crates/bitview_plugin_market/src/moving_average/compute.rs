use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_price::Vecs as PriceVecs;
use brk_error::Result;
use brk_exit::Exit;

use super::{Vecs, vecs::EmaPeriodId};

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

    for &period in EmaPeriodId::ALL {
        period
            .select_mut(&mut vecs.ema_stored)
            .compute_rolling_ema(
                starting_lengths.height,
                blocks.lookback.start_vec(period.days()),
                close,
                exit,
            )?;
    }

    Ok(())
}
