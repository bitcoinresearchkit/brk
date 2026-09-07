use axum::{http::HeaderMap, response::Response};
use serde_json::to_vec;

use crate::{
    AppState, CacheParams, CacheStrategy, CdnCacheMode,
    error::Result,
    extended::{HeaderMapExtended, ResponseExtended},
    raw_body::RawBodyPermit,
};

pub async fn serve(state: AppState, headers: HeaderMap) -> Result<Response> {
    // This hash read validates completed publication before a cheap 304.
    let hash = state.read(|q| q.mempool_txids_hash()).await?;
    let params = CacheParams::resolve(&CacheStrategy::LiveHash(hash), CdnCacheMode::Live);
    if params.matches_etag(&headers) {
        return Ok(Response::new_not_modified(&params));
    }
    let bodies = state.mempool_txid_bodies.clone();
    state
        .read_body(
            &state.sync_query,
            &state.mempool_txid_bodies,
            move |query, permit| {
                // Recheck publication and capture array/hash together after admission.
                let (txids, hash) = query.mempool_txids_with_hash()?;
                let params =
                    CacheParams::resolve(&CacheStrategy::LiveHash(hash), CdnCacheMode::Live);
                if params.matches_etag(&headers) {
                    return Ok(Some(Response::new_not_modified(&params)));
                }
                let Some(permit) = permit.or_else(|| RawBodyPermit::try_acquire(&bodies)) else {
                    return Ok(None);
                };
                let bytes = to_vec(&txids)?;
                Ok(Some(permit.response(
                    params,
                    bytes.into(),
                    HeaderMapExtended::insert_content_type_application_json,
                )))
            },
        )
        .await
        .map_err(Into::into)
}
