use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_indexer::Indexer;
use brk_error::Result;
use brk_exit::Exit;

use super::Vecs;

pub fn compute(vecs: &mut Vecs, indexer: &Indexer, blocks: &BlocksVecs, exit: &Exit) -> Result<()> {
    let starting_height = indexer.safe_lengths().height;
    let window_starts = blocks.lookback.window_starts();

    vecs.total
        .compute_rest(starting_height, &window_starts, exit)?;
    Ok(())
}
