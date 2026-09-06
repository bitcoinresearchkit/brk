use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use serde_json::to_vec;

use crate::{
    AppState, CacheParams, CacheStrategy, CdnCacheMode, Error,
    error::Result,
    extended::{HeaderMapExtended, ResponseExtended},
    raw_body::RawBodyPermit,
};

pub async fn serve(state: AppState, headers: HeaderMap) -> Result<Response> {
    // This hash read validates completed publication before a cheap 304.
    let params = CacheParams::resolve(&state.mempool_txids_strategy()?, CdnCacheMode::Live);
    if params.matches_etag(&headers) {
        return Ok(Response::new_not_modified(&params));
    }
    let bodies = state.mempool_txid_bodies.clone();
    state
        .run_admitted(move |query| {
            let Some(permit) = RawBodyPermit::try_acquire(&bodies) else {
                return Ok(
                    Error::overloaded("Mempool txid response capacity exhausted").into_response(),
                );
            };
            // Recheck publication and capture array/hash together after admission.
            let (txids, hash) = query.mempool_txids_with_hash()?;
            let params = CacheParams::resolve(&CacheStrategy::LiveHash(hash), CdnCacheMode::Live);
            if params.matches_etag(&headers) {
                return Ok(Response::new_not_modified(&params));
            }
            let bytes = to_vec(&txids)?;
            let mut response = AppState::assemble_response(
                params,
                Ok(permit.bytes(bytes.into())),
                HeaderMapExtended::insert_content_type_application_json,
            );
            response.extensions_mut().insert(permit);
            Ok(response)
        })
        .await
        .map_err(Into::into)
}
