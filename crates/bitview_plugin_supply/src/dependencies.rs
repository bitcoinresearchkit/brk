use bitview_plugin_indexer::Indexer;
use bitview_plugin_mining::Vecs as MiningVecs;
use bitview_plugin_outputs::Vecs as OutputsVecs;
use bitview_plugin_price::Vecs as PriceVecs;

pub struct Dependencies<'a> {
    pub indexer: &'a Indexer,
    pub outputs: &'a OutputsVecs,
    pub mining: &'a MiningVecs,
    pub price: &'a PriceVecs,
}
