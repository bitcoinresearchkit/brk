use axum::{body::Bytes, http::HeaderMap, response::Response};
use bitview_query::RepresentationId;
use serde::Serialize;
use serde_json::to_vec;

use crate::{CacheParams, extended::ResponseExtended};

/// Server-lifetime JSON with its validator computed once from the body.
pub struct PreparedJson {
    bytes: Bytes,
    params: CacheParams,
}

impl PreparedJson {
    pub fn new(value: impl Serialize) -> Self {
        let bytes = Bytes::from(to_vec(&value).unwrap());
        let RepresentationId::Content(hash) = RepresentationId::content(&bytes) else {
            unreachable!()
        };
        let params = CacheParams::revalidate(format!("c{hash:x}").into());
        Self { bytes, params }
    }

    pub fn respond(&self, headers: &HeaderMap) -> Response {
        Response::json_bytes(headers, &self.params, || self.bytes.clone())
    }

    #[cfg(feature = "series")]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
#[path = "../../tests/unit/prepared_json.rs"]
mod tests;
