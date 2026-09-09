use bitview_plugin_indexer::Indexer;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_plugin_price::Vecs as PriceVecs;
use brk_error::Result;
use brk_exit::Exit;

use super::Vecs;
use crate::fees;

pub fn compute(
    vecs: &mut Vecs,
    indexer: &Indexer,
    mappings: &MappingsVecs,
    prices: &PriceVecs,
    fees_vecs: &fees::Vecs,
    exit: &Exit,
) -> Result<()> {
    let starting_height = indexer.safe_lengths().height;

    vecs.transfer_volume.compute_filtered_from_indexes(
        starting_height,
        &prices.spot.cents.height,
        &indexer.vecs().transactions.first_tx_index,
        &mappings.height.tx_index_count,
        &fees_vecs.input_value,
        |sats| !sats.is_max(),
        exit,
    )?;

    Ok(())
}
