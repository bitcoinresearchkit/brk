use axum::{http::HeaderMap, response::Response};
use bitview_query::RepresentationId;
use brk_types::{Timestamp, Version};
use serde_json::to_vec;

use crate::{
    AppState, CacheParams, CdnCacheMode,
    error::Result,
    extended::{HeaderMapExtended, ResponseExtended},
    raw_body::RawBodyPermit,
};

pub async fn serve(
    state: AppState,
    headers: HeaderMap,
    timestamp: Option<Timestamp>,
) -> Result<Response> {
    let bodies = state.historical_price_bodies.clone();
    state
        .read_body(
            &state.sync_query,
            &state.historical_price_bodies,
            move |query, permit| {
                let value = query.historical_price(timestamp)?;
                let bytes = to_vec(&value)?;
                let params = CacheParams::resolve(
                    &AppState::representation_strategy(
                        Version::ONE,
                        RepresentationId::content(&bytes),
                    ),
                    CdnCacheMode::Live,
                );
                if params.matches_etag(&headers) {
                    return Ok(Some(Response::new_not_modified(&params)));
                }
                let Some(permit) = permit.or_else(|| RawBodyPermit::try_acquire(&bodies)) else {
                    return Ok(None);
                };
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
