use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
};

use axum::body::{Bytes, HttpBody};
use http_body::Frame;

/// Empty content with no representation-length hint for Axum to turn into a
/// Content-Length header. A 304 length would have to describe the selected GET.
pub struct NotModifiedBody;

impl HttpBody for NotModifiedBody {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Infallible>>> {
        Poll::Ready(None)
    }

    fn is_end_stream(&self) -> bool {
        true
    }
}
