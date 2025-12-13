use std::collections::BTreeMap;

use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_monitor::Mempool;
use brk_reader::Reader;
use brk_types::{
    Address, AddressStats, BlockInfo, BlockStatus, Height, Index, IndexInfo, Limit, MempoolInfo,
    Metric, MetricCount, RecommendedFees, Transaction, TreeNode, TxStatus, Txid, TxidPath, Utxo,
};
use tokio::task::spawn_blocking;

use crate::{
    Output, PaginatedIndexParam, PaginatedMetrics, PaginationParam, Params, Query,
    vecs::{IndexToVec, MetricToVec, Vecs},
};

#[derive(Clone)]
pub struct AsyncQuery(Query);

impl AsyncQuery {
    pub fn build(
        reader: &Reader,
        indexer: &Indexer,
        computer: &Computer,
        mempool: Option<Mempool>,
    ) -> Self {
        Self(Query::build(reader, indexer, computer, mempool))
    }

    pub fn inner(&self) -> &Query {
        &self.0
    }

    pub async fn get_height(&self) -> Height {
        self.0.get_height()
    }

    pub async fn get_address(&self, address: Address) -> Result<AddressStats> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_address(address)).await?
    }

    pub async fn get_address_txids(
        &self,
        address: Address,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<Vec<Txid>> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_address_txids(address, after_txid, limit)).await?
    }

    pub async fn get_address_utxos(&self, address: Address) -> Result<Vec<Utxo>> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_address_utxos(address)).await?
    }

    pub async fn get_transaction(&self, txid: TxidPath) -> Result<Transaction> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_transaction(txid)).await?
    }

    pub async fn get_transaction_status(&self, txid: TxidPath) -> Result<TxStatus> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_transaction_status(txid)).await?
    }

    pub async fn get_transaction_hex(&self, txid: TxidPath) -> Result<String> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_transaction_hex(txid)).await?
    }

    pub async fn get_block(&self, hash: String) -> Result<BlockInfo> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_block(&hash)).await?
    }

    pub async fn get_block_by_height(&self, height: Height) -> Result<BlockInfo> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_block_by_height(height)).await?
    }

    pub async fn get_block_status(&self, hash: String) -> Result<BlockStatus> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_block_status(&hash)).await?
    }

    pub async fn get_blocks(&self, start_height: Option<Height>) -> Result<Vec<BlockInfo>> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_blocks(start_height)).await?
    }

    pub async fn get_block_txids(&self, hash: String) -> Result<Vec<Txid>> {
        let query = self.0.clone();
        spawn_blocking(move || query.get_block_txids(&hash)).await?
    }

    pub async fn get_mempool_info(&self) -> Result<MempoolInfo> {
        self.0.get_mempool_info()
    }

    pub async fn get_mempool_txids(&self) -> Result<Vec<Txid>> {
        self.0.get_mempool_txids()
    }

    pub async fn get_recommended_fees(&self) -> Result<RecommendedFees> {
        self.0.get_recommended_fees()
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
    // ) -> Result<Vec<(String, &&dyn AnyExportableVec)>> {
    //     let query = self.0.clone();
    //     spawn_blocking(move || query.search_metric_with_index(metric, index)).await?
    // }

    // pub async fn format(
    //     &self,
    //     metrics: Vec<(String, &&dyn AnyExportableVec)>,
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

    pub async fn metric_to_indexes(&self, metric: Metric) -> Option<&Vec<Index>> {
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
