use std::sync::Arc;

use axum::{
    body::{Body, Bytes},
    response::Response,
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

mod body;
mod bytes;

use body::RetainedBody;
use bytes::RetainedBytes;

/// One admitted encoded response, from allocation through the final body/frame owner.
#[derive(Clone)]
pub struct RawBodyPermit {
    _permit: Arc<OwnedSemaphorePermit>,
}

impl RawBodyPermit {
    // At most 32 MB of raw serialized payloads (8 * the 4 MB block bound),
    // plus bounded-per-response encoder state and one admitted decoder.
    #[cfg(feature = "chain")]
    pub const CAPACITY: usize = 8;

    pub fn try_acquire(budget: &Arc<Semaphore>) -> Option<Self> {
        budget.clone().try_acquire_owned().ok().map(|permit| Self {
            _permit: Arc::new(permit),
        })
    }

    pub fn bytes(&self, bytes: Bytes) -> Bytes {
        Bytes::from_owner(RetainedBytes {
            bytes,
            _permit: self.clone(),
        })
    }

    /// Called outside compression so encoder state and emitted frames retain
    /// admission too, even after the encoder consumes its original raw input.
    pub fn retain(mut response: Response) -> Response {
        let Some(permit) = response.extensions_mut().remove::<Self>() else {
            return response;
        };
        response.map(|body| Body::new(RetainedBody { body, permit }))
    }
}

#[cfg(test)]
#[path = "../../tests/unit/raw_body.rs"]
mod tests;
