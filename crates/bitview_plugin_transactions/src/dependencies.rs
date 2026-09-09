use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_inputs::Vecs as InputsVecs;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_plugin_price::Vecs as PriceVecs;

pub struct Dependencies<'a> {
    pub indexer: &'a Indexer,
    pub inputs: &'a InputsVecs,
    pub mappings: &'a MappingsVecs,
    pub blocks: &'a BlocksVecs,
    pub price: &'a PriceVecs,
}
