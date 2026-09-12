use bitview_plugin_coinflow::Vecs as CoinflowVecs;
use bitview_plugin_cointime::Vecs as CointimeVecs;
use bitview_plugin_distribution::{UTXOStates, Vecs as DistributionVecs};
use bitview_plugin_indexer::Indexer;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_plugin_price::Vecs as PriceVecs;

pub struct Dependencies<'a> {
    pub indexer: &'a Indexer,
    pub price: &'a PriceVecs,
    pub mappings: &'a MappingsVecs,
    pub distribution: &'a DistributionVecs,
    pub utxo_states: &'a UTXOStates,
    pub cointime: &'a CointimeVecs,
    pub coinflow: &'a CoinflowVecs,
}
