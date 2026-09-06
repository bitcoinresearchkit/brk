use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use bitview_query::RepresentationId;
use brk_types::{Timestamp, Version};
use serde_json::to_vec;

use crate::{
    AppState, CacheParams, CdnCacheMode, Error,
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
        .run_admitted(move |query| {
            let value = query.historical_price(timestamp)?;
            let bytes = to_vec(&value)?;
            let params = CacheParams::resolve(
                &AppState::representation_strategy(Version::ONE, RepresentationId::content(&bytes)),
                CdnCacheMode::Live,
            );
            if params.matches_etag(&headers) {
                return Ok(Response::new_not_modified(&params));
            }
            let Some(permit) = RawBodyPermit::try_acquire(&bodies) else {
                return Ok(
                    Error::overloaded("Historical price response capacity exhausted")
                        .into_response(),
                );
            };
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
