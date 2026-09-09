use bitview_plugin_distribution::Vecs as DistributionVecs;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_market::Vecs as MarketVecs;
use bitview_plugin_mining::Vecs as MiningVecs;

pub struct Dependencies<'a> {
    pub indexer: &'a Indexer,
    pub mining: &'a MiningVecs,
    pub distribution: &'a DistributionVecs,
    pub market: &'a MarketVecs,
}
