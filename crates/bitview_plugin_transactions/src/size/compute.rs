use brk_error::Result;

use bitview_plugin_indexer::Indexer;
use brk_exit::Exit;

use super::Vecs;

pub fn compute(
    vecs: &mut Vecs,
    indexer: &Indexer,
    mappings: &bitview_plugin_mappings::Vecs,
    exit: &Exit,
) -> Result<()> {
    let starting_lengths = indexer.safe_lengths();

    vecs.weight.derive_from(
        mappings,
        &starting_lengths,
        &indexer.vecs().transactions.first_tx_index,
        &indexer.vecs().transactions.weight,
        exit,
    )?;

    Ok(())
}
