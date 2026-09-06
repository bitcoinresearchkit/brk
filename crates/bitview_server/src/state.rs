use std::{future::Future, path::PathBuf, sync::Arc, time::Instant};

use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, HeaderValue, Response, header},
    response::IntoResponse,
};
use bitview_query::{AsyncQuery, Query, RepresentationId};
#[cfg(feature = "chain")]
use bitview_query::{
    BlockTemplateSource, ResolvedAddrChainTxs, ResolvedBlockTemplateDiff, ResolvedConfirmedTx,
    ResolvedCpfp, ResolvedPoolBlocks, ResolvedRawTransaction, ResolvedRbf, ResolvedTransaction,
};
use brk_error::{Error as BrkError, Result};
use brk_rpc::AsyncClient;
#[cfg(feature = "chain")]
use brk_types::{
    Addr, BlockHashPrefix, Height, MempoolBlock, NextBlockHash, PoolSlug, RecommendedFees, TxIndex,
    TxStatus, Txid, TxidPrefix, Version,
};
use derive_more::Deref;
use jiff::Timestamp;
use serde::Serialize;
use serde_json::to_vec;
use tokio::sync::Semaphore;

#[cfg(feature = "chain")]
use crate::prepared_json::PreparedJson;
#[cfg(feature = "series")]
use crate::series_bodies::SeriesBodies;
use crate::{CacheParams, CacheStrategy, CdnCacheMode, Error, Website, extended::ResponseExtended};

#[derive(Clone, Deref)]
pub struct AppState {
    #[deref]
    pub query: AsyncQuery,
    pub sync_query: Arc<Semaphore>,
    pub disk_query: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub raw_block_bodies: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub historical_price_bodies: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub mempool_txid_bodies: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub broadcast_requests: Arc<Semaphore>,
    pub node: AsyncClient,
    #[cfg(feature = "series")]
    pub series_bodies: SeriesBodies,
    #[cfg(feature = "urpd")]
    pub urpd_query: Arc<Semaphore>,
    #[cfg(feature = "urpd")]
    pub urpd_bodies: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub mining_pools_body: Arc<PreparedJson>,
    pub data_path: PathBuf,
    pub website: Website,
    pub started_at: Timestamp,
    pub started_instant: Instant,
    pub max_weight: usize,
    pub max_utxos: usize,
    pub cdn_cache_mode: CdnCacheMode,
}

impl AppState {
    /// Keep blocking admission with the job, including after request cancellation.
    pub async fn run_admitted<T: Send + 'static>(
        &self,
        f: impl FnOnce(&Query) -> Result<T> + Send + 'static,
    ) -> Result<T> {
        self.run_with_admission(&self.sync_query, f).await
    }

    pub async fn run_with_admission<T: Send + 'static>(
        &self,
        admission: &Arc<Semaphore>,
        f: impl FnOnce(&Query) -> Result<T> + Send + 'static,
    ) -> Result<T> {
        let permit = admission
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| BrkError::Internal("query admission closed"))?;
        self.run(move |q| {
            let _permit = permit;
            f(q)
        })
        .await
    }

    /// Resolve one confirmed address page and its exact chain anchor once.
    #[cfg(feature = "chain")]
    pub async fn addr_chain_txs_preflight(
        &self,
        version: Version,
        addr: &Addr,
        after_txid: Option<Txid>,
        limit: usize,
    ) -> Result<(ResolvedAddrChainTxs, CacheStrategy)> {
        let addr = addr.clone();
        self.run_admitted(move |q| {
            let resolved = q.resolve_addr_chain_txs(&addr, after_txid, limit)?;
            let strategy = Self::addr_chain_txs_strategy(version, &resolved);
            Ok((resolved, strategy))
        })
        .await
    }

    #[cfg(feature = "chain")]
    pub fn addr_chain_txs_strategy(
        version: Version,
        resolved: &ResolvedAddrChainTxs,
    ) -> CacheStrategy {
        CacheStrategy::ActivityBound(version, BlockHashPrefix::from(&resolved.activity_anchor()))
    }

    /// Resolve an exact confirmed transaction once and bind its response to
    /// the block that currently contains it.
    #[cfg(feature = "chain")]
    pub async fn confirmed_tx_preflight(
        &self,
        version: Version,
        txid: &Txid,
    ) -> Result<(ResolvedConfirmedTx, CacheStrategy)> {
        let txid = *txid;
        self.run_admitted(move |q| {
            let tx = q.resolve_confirmed_tx(&txid)?;
            let strategy = Self::representation_strategy(version, tx.identity());
            Ok((tx, strategy))
        })
        .await
    }

    /// Resolve exact raw transaction bytes and derive their witness-aware cache strategy.
    #[cfg(feature = "chain")]
    pub async fn raw_transaction_preflight(
        &self,
        version: Version,
        txid: &Txid,
    ) -> Result<(ResolvedRawTransaction, CacheStrategy)> {
        let txid = *txid;
        self.run_admitted(move |q| {
            let transaction = q.resolve_raw_transaction(&txid)?;
            let strategy = Self::representation_strategy(version, transaction.identity());
            Ok((transaction, strategy))
        })
        .await
    }

    /// Resolve exact transaction JSON and derive its content- or block-bound cache strategy.
    #[cfg(feature = "chain")]
    pub async fn transaction_preflight(
        &self,
        version: Version,
        txid: &Txid,
    ) -> Result<(ResolvedTransaction, CacheStrategy)> {
        let txid = *txid;
        self.run_admitted(move |q| {
            let transaction = q.resolve_transaction(&txid)?;
            let strategy = Self::representation_strategy(version, transaction.identity());
            Ok((transaction, strategy))
        })
        .await
    }

    /// Resolve exact CPFP JSON and derive its content- or block-bound cache strategy.
    #[cfg(feature = "chain")]
    pub async fn cpfp_preflight(
        &self,
        version: Version,
        txid: &Txid,
    ) -> Result<(ResolvedCpfp, CacheStrategy)> {
        let txid = *txid;
        self.run_admitted(move |q| {
            let cpfp = q.resolve_cpfp(&txid)?;
            let strategy = Self::representation_strategy(version, cpfp.identity());
            Ok((cpfp, strategy))
        })
        .await
    }

    /// Resolve one exact RBF tree. Empty responses get an exact strategy here.
    #[cfg(feature = "chain")]
    pub async fn rbf_preflight(
        &self,
        version: Version,
        txid: &Txid,
    ) -> Result<(ResolvedRbf, Option<CacheStrategy>)> {
        let txid = *txid;
        self.run_admitted(move |q| {
            let rbf = q.resolve_rbf(&txid)?;
            let strategy = rbf
                .identity()
                .map(|identity| Self::representation_strategy(version, identity));
            Ok((rbf, strategy))
        })
        .await
    }

    #[cfg(feature = "chain")]
    pub fn representation_strategy(version: Version, identity: RepresentationId) -> CacheStrategy {
        match identity {
            RepresentationId::Content(hash) => CacheStrategy::LiveHash(hash),
            // Anchor identity and availability are distinct: a displaced
            // confirmed transaction must be resolved again even at depth.
            RepresentationId::Block { hash, .. } => {
                CacheStrategy::Live(format!("tx3-{version}-{hash}").into())
            }
        }
    }

    /// Resolve a transaction status and its exact cache strategy without
    /// dispatching a second query.
    #[cfg(feature = "chain")]
    pub async fn tx_status_preflight(
        &self,
        version: Version,
        txid: &Txid,
    ) -> Result<(TxStatus, CacheStrategy)> {
        let txid = *txid;
        self.run_admitted(move |q| {
            let status = q.transaction_status(&txid)?;
            let strategy = match status.block_hash {
                Some(hash) => CacheStrategy::Live(format!("tx-status3-{version}-{hash}").into()),
                None => CacheStrategy::LiveHash(*TxidPrefix::from(txid)),
            };
            Ok((status, strategy))
        })
        .await
    }

    /// Resolve the complete text representation before deriving its validator.
    #[cfg(feature = "chain")]
    pub async fn txid_by_index_preflight(
        &self,
        version: Version,
        index: TxIndex,
    ) -> Result<(Txid, CacheStrategy)> {
        self.run_admitted(move |q| {
            let txid = q.txid_by_index(index)?;
            let strategy = CacheStrategy::Live(format!("tx-index3-{version}-{txid}").into());
            Ok((txid, strategy))
        })
        .await
    }

    /// Resolve transaction first-seen times and their exact response validator
    /// from one mempool snapshot.
    #[cfg(feature = "chain")]
    pub fn transaction_times_preflight(&self, txids: &[Txid]) -> Result<(Vec<u64>, CacheStrategy)> {
        self.sync(|q| {
            let (times, hash) = q.transaction_times_with_hash(txids)?;
            Ok((times, CacheStrategy::LiveHash(hash)))
        })
    }

    /// Resolve one latest pool-block page and its exact activity anchor once.
    #[cfg(feature = "chain")]
    pub async fn pool_blocks_preflight(
        &self,
        slug: PoolSlug,
        before_height: Option<Height>,
        limit: usize,
    ) -> Result<ResolvedPoolBlocks> {
        self.run_admitted(move |q| q.resolve_pool_blocks(slug, before_height, limit))
            .await
    }

    /// Resolve every projected mempool-block statistic from one snapshot.
    #[cfg(feature = "chain")]
    pub fn mempool_blocks(&self) -> Result<Vec<MempoolBlock>> {
        self.sync(|query| query.mempool_blocks())
    }

    /// Resolve recommended fees from one projected-mempool snapshot.
    #[cfg(feature = "chain")]
    pub fn recommended_fees(&self) -> Result<RecommendedFees> {
        self.sync(|query| query.recommended_fees())
    }

    /// Resolve the projected-next-block hash and its matching cache strategy once.
    #[cfg(feature = "chain")]
    pub fn mempool_hash_preflight(&self) -> Result<(NextBlockHash, CacheStrategy)> {
        self.sync(|q| {
            let hash = q.mempool_hash()?;
            let strategy = CacheStrategy::LiveHash(hash.into());
            Ok((hash, strategy))
        })
    }

    /// Resolve the order-sensitive mempool-txid validator without copying the list.
    #[cfg(feature = "chain")]
    pub fn mempool_txids_strategy(&self) -> Result<CacheStrategy> {
        self.sync(|q| q.mempool_txids_hash().map(CacheStrategy::LiveHash))
    }

    /// Capture a complete published block template before ETag handling.
    #[cfg(feature = "chain")]
    pub fn block_template_preflight(&self) -> Result<BlockTemplateSource> {
        self.sync(|q| q.resolve_block_template())
    }

    /// Validate historical availability before ETag handling.
    #[cfg(feature = "chain")]
    pub fn block_template_diff_preflight(
        &self,
        since: NextBlockHash,
    ) -> Result<ResolvedBlockTemplateDiff> {
        self.sync(|q| q.resolve_block_template_diff(since))
    }

    pub fn assemble_response(
        params: CacheParams,
        result: Result<Bytes>,
        apply_content_headers: impl FnOnce(&mut HeaderMap),
    ) -> Response<Body> {
        match result {
            Ok(bytes) => {
                let mut response = Response::new(Body::from(bytes));
                let headers = response.headers_mut();
                apply_content_headers(headers);
                params.apply_to(headers);
                response
            }
            Err(error) => Error::from(error).into_response(),
        }
    }

    /// Shared response pipeline: ETag short-circuit, body computation on the
    /// query thread, and header assembly. Used by [`AppState::respond`]
    /// (strategy-driven) and series endpoints, which build [`CacheParams`]
    /// directly from query resolution.
    pub async fn respond_with_params<F>(
        &self,
        headers: &HeaderMap,
        params: CacheParams,
        apply_content_headers: impl FnOnce(&mut HeaderMap),
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<Bytes> + Send + 'static,
    {
        Self::respond_with_future(headers, params, async {
            Ok((self.run_admitted(f).await?, apply_content_headers))
        })
        .await
    }

    /// Defer body work and content-header preparation until after revalidation.
    pub async fn respond_with_future<H: FnOnce(&mut HeaderMap)>(
        headers: &HeaderMap,
        params: CacheParams,
        body: impl Future<Output = Result<(Bytes, H)>>,
    ) -> Response<Body> {
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified(&params);
        }

        match body.await {
            Ok((bytes, apply_content_headers)) => {
                Self::assemble_response(params, Ok(bytes), apply_content_headers)
            }
            Err(error) => Error::from(error).into_response(),
        }
    }

    /// Strategy-driven cached response.
    async fn respond<F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        content_type: &'static str,
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<Bytes> + Send + 'static,
    {
        let params = CacheParams::resolve(&strategy, self.cdn_cache_mode);
        self.respond_with_params(
            headers,
            params,
            |h| {
                h.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
            },
            f,
        )
        .await
    }

    fn respond_immediate(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        content_type: &'static str,
        bytes: impl FnOnce() -> Bytes,
    ) -> Response<Body> {
        let params = CacheParams::resolve(&strategy, self.cdn_cache_mode);
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified(&params);
        }

        Self::assemble_response(params, Ok(bytes()), |headers| {
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
        })
    }

    /// Immediate JSON response whose value is only built after ETag validation.
    pub fn respond_json_immediate<T: Serialize>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        value: impl FnOnce() -> T,
    ) -> Response<Body> {
        self.respond_immediate(headers, strategy, "application/json", || {
            Bytes::from(to_vec(&value()).unwrap())
        })
    }

    /// Immediate JSON response for values already resolved during preflight.
    pub fn respond_json_value<T: Serialize>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        value: T,
    ) -> Response<Body> {
        self.respond_json_immediate(headers, strategy, || value)
    }

    /// Immediate text response for values already resolved during preflight.
    #[cfg(feature = "chain")]
    pub fn respond_text_value(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        value: String,
    ) -> Response<Body> {
        self.respond_immediate(headers, strategy, "text/plain", || Bytes::from(value))
    }

    /// Immediate JSON response whose validator is derived from its exact bytes.
    #[cfg(feature = "chain")]
    pub fn respond_json_content_value<T: Serialize>(
        &self,
        headers: &HeaderMap,
        value: T,
    ) -> Response<Body> {
        let bytes = Bytes::from(to_vec(&value).unwrap());
        self.respond_json_content_bytes(headers, bytes)
    }

    /// JSON response with HTTP cache validation.
    pub async fn respond_json<T, F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> Response<Body>
    where
        T: Serialize + Send + 'static,
        F: FnOnce(&Query) -> Result<T> + Send + 'static,
    {
        self.respond(headers, strategy, "application/json", move |q| {
            let value = f(q)?;
            Ok(Bytes::from(to_vec(&value).unwrap()))
        })
        .await
    }

    /// Pre-serialized JSON response with HTTP cache validation.
    pub async fn respond_json_bytes<F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<Vec<u8>> + Send + 'static,
    {
        self.respond(headers, strategy, "application/json", move |q| {
            f(q).map(Bytes::from)
        })
        .await
    }

    /// JSON response whose representation identity is produced with its bytes.
    #[cfg(feature = "chain")]
    pub async fn respond_json_bound<F>(
        &self,
        headers: &HeaderMap,
        version: Version,
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<(Vec<u8>, RepresentationId)> + Send + 'static,
    {
        let request_headers = headers.clone();
        let cdn_cache_mode = self.cdn_cache_mode;
        let outcome = self
            .run_admitted(move |query| {
                let initial_tip = query.tip_hash_prefix();
                let (bytes, identity) = f(query)?;
                let current_tip = query.tip_hash_prefix();
                if matches!(identity, RepresentationId::Block { .. }) && initial_tip != current_tip
                {
                    return Err(BrkError::StateUpdating);
                }
                let strategy = Self::representation_strategy(version, identity);
                let params = CacheParams::resolve(&strategy, cdn_cache_mode);
                if params.matches_etag(&request_headers) {
                    return Ok((params, None));
                }
                Ok((params, Some(Bytes::from(bytes))))
            })
            .await;

        let (params, body) = match outcome {
            Ok((params, None)) => return ResponseExtended::new_not_modified(&params),
            Ok((params, Some(body))) => (params, body),
            Err(error) => return Error::from(error).into_response(),
        };
        Self::assemble_response(params, Ok(body), |headers| {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        })
    }

    /// JSON response whose validator is derived from the exact serialized value.
    #[cfg(any(
        feature = "chain",
        feature = "price",
        feature = "urpd",
        all(test, feature = "series")
    ))]
    pub async fn respond_json_content<T, F>(&self, headers: &HeaderMap, f: F) -> Response<Body>
    where
        T: Serialize + Send + 'static,
        F: FnOnce(&Query) -> Result<T> + Send + 'static,
    {
        let bytes = self
            .run_admitted(move |query| {
                let value = f(query)?;
                Ok(Bytes::from(to_vec(&value)?))
            })
            .await;
        match bytes {
            Ok(bytes) => self.respond_json_content_bytes(headers, bytes),
            Err(error) => Error::from(error).into_response(),
        }
    }

    pub fn respond_json_content_bytes(&self, headers: &HeaderMap, bytes: Bytes) -> Response<Body> {
        let RepresentationId::Content(hash) = RepresentationId::content(&bytes) else {
            unreachable!("content identity constructor returned a block identity");
        };
        self.respond_immediate(
            headers,
            CacheStrategy::LiveHash(hash),
            "application/json",
            || bytes,
        )
    }

    /// Text response with HTTP cache validation.
    pub async fn respond_text<F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<String> + Send + 'static,
    {
        self.respond(headers, strategy, "text/plain", move |q| {
            let value = f(q)?;
            Ok(Bytes::from(value))
        })
        .await
    }

    /// Binary response with HTTP cache validation.
    pub async fn respond_bytes<T, F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> Response<Body>
    where
        T: Into<Vec<u8>> + Send + 'static,
        F: FnOnce(&Query) -> Result<T> + Send + 'static,
    {
        self.respond(headers, strategy, "application/octet-stream", move |q| {
            let value = f(q)?;
            Ok(Bytes::from(value.into()))
        })
        .await
    }
}

#[cfg(test)]
#[path = "../tests/unit/state.rs"]
mod tests;
