use bitview_compute::{LazyPerBlock, LazyPercentPerBlock};
use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_distribution::Vecs as DistributionVecs;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_price::Vecs as PriceVecs;
use brk_types::{PartsPerMillionSigned64, StoredF64};

pub struct Dependencies<'a> {
    pub indexer: &'a Indexer,
    pub price: &'a PriceVecs,
    pub blocks: &'a BlocksVecs,
    pub inflation_rate: &'a LazyPercentPerBlock<PartsPerMillionSigned64>,
    pub velocity_native: &'a LazyPerBlock<StoredF64>,
    pub velocity_fiat: &'a LazyPerBlock<StoredF64>,
    pub distribution: &'a DistributionVecs,
}
