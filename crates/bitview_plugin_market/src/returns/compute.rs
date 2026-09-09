use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_indexer::Indexer;
use brk_error::Result;
use brk_exit::Exit;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::Vecs;

pub fn compute(vecs: &mut Vecs, indexer: &Indexer, blocks: &BlocksVecs, exit: &Exit) -> Result<()> {
    let starting_lengths = indexer.safe_lengths();

    let _24h_price_return_ratio = &vecs.periods._24h.ratio.height;

    vecs.sd_24h
        .as_mut_array()
        .into_par_iter()
        .try_for_each(|sd| {
            sd.compute_all(
                &blocks.lookback,
                &starting_lengths,
                exit,
                _24h_price_return_ratio,
            )
        })
}
