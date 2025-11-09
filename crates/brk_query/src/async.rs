use std::collections::BTreeMap;

use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_reader::Reader;
use brk_types::{
    Address, AddressStats, Height, Index, IndexInfo, Limit, Metric, MetricCount, Transaction,
    TreeNode, TxidPath,
};
#[cfg(feature = "tokio")]
use tokio::task::spawn_blocking;

use crate::{
    Output, PaginatedIndexParam, PaginatedMetrics, PaginationParam, Params, ParamsOpt, Query,
    vecs::{IndexToVec, MetricToVec, Vecs},
};

#[derive(Clone)]
#[cfg(feature = "tokio")]
pub struct AsyncQuery(Query);

impl AsyncQuery {
    pub async fn build(reader: &Reader, indexer: &Indexer, computer: &Computer) -> Self {
        Self(Query::build(reader, indexer, computer))
    }

    pub async fn get_height(&self) -> Height {
        self.0.get_height()
    }

    pub async fn get_address(&self, address: Address) -> Result<AddressStats> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_address(address)).await?
    }

    pub async fn get_transaction(&self, txid: TxidPath) -> Result<Transaction> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_transaction(txid)).await?
    }

    pub async fn match_metric(&self, metric: Metric, limit: Limit) -> Result<Vec<&'static str>> {
        let query = self.0.clone();
        spawn_blocking(move || Ok(query.match_metric(&metric, limit))).await?
    }

    // pub async fn search_metric_with_index(
    //     &self,
    //     metric: &str,
    //     index: Index,
    //     // params: &Params,
    // ) -> Result<Vec<(String, &&dyn AnyCollectableVec)>> {
    //     let query = self.0.clone();
    //     spawn_blocking(move || query.search_metric_with_index(metric, index)).await?
    // }

    // pub async fn format(
    //     &self,
    //     metrics: Vec<(String, &&dyn AnyCollectableVec)>,
    //     params: &ParamsOpt,
    // ) -> Result<Output> {
    //     let query = self.0.clone();
    //     spawn_blocking(move || query.format(metrics, params)).await?
    // }

    pub async fn search_and_format(&self, params: Params) -> Result<Output> {
        let query = self.0.clone();
        spawn_blocking(move || query.search_and_format(params)).await?
    }

    pub async fn metric_to_index_to_vec(&self) -> &BTreeMap<&str, IndexToVec<'_>> {
        self.0.metric_to_index_to_vec()
    }

    pub async fn index_to_metric_to_vec(&self) -> &BTreeMap<Index, MetricToVec<'_>> {
        self.0.index_to_metric_to_vec()
    }

    pub async fn metric_count(&self) -> MetricCount {
        self.0.metric_count()
    }

    pub async fn distinct_metric_count(&self) -> usize {
        self.0.distinct_metric_count()
    }

    pub async fn total_metric_count(&self) -> usize {
        self.0.total_metric_count()
    }

    pub async fn get_indexes(&self) -> &[IndexInfo] {
        self.0.get_indexes()
    }

    pub async fn get_metrics(&self, pagination: PaginationParam) -> PaginatedMetrics {
        self.0.get_metrics(pagination)
    }

    pub async fn get_metrics_catalog(&self) -> &TreeNode {
        self.0.get_metrics_catalog()
    }

    pub async fn get_index_to_vecids(&self, paginated_index: PaginatedIndexParam) -> Vec<&str> {
        self.0.get_index_to_vecids(paginated_index)
    }

    pub async fn metric_to_indexes(&self, metric: String) -> Option<&Vec<Index>> {
        self.0.metric_to_indexes(metric)
    }

    #[inline]
    pub async fn reader(&self) -> &Reader {
        self.0.reader()
    }

    #[inline]
    pub async fn indexer(&self) -> &Indexer {
        self.0.indexer()
    }

    #[inline]
    pub async fn computer(&self) -> &Computer {
        self.0.computer()
    }

    #[inline]
    pub async fn vecs(&self) -> &'static Vecs<'static> {
        self.0.vecs()
    }
}
