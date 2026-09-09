use bitview_plugin_indexer::Indexer;
use bitview_plugin_inputs::Vecs as InputsVecs;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_plugin_outputs::Vecs as OutputsVecs;
use bitview_plugin_price::Vecs as PriceVecs;
use bitview_plugin_transactions::Vecs as TransactionsVecs;

pub struct Dependencies<'a> {
    pub indexer: &'a Indexer,
    pub mappings: &'a MappingsVecs,
    pub inputs: &'a InputsVecs,
    pub outputs: &'a OutputsVecs,
    pub transactions: &'a TransactionsVecs,
    pub price: &'a PriceVecs,
}
