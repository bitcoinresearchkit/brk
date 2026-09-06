use std::{
    pin::Pin,
    task::{Context, Poll, ready},
};

use axum::{
    Error as AxumError,
    body::{Body, Bytes, HttpBody},
};
use http_body::{Frame, SizeHint};

use super::RawBodyPermit;

pub struct RetainedBody {
    pub body: Body,
    pub permit: RawBodyPermit,
}

impl HttpBody for RetainedBody {
    type Data = Bytes;
    type Error = AxumError;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, AxumError>>> {
        let frame = ready!(Pin::new(&mut self.body).poll_frame(cx));
        Poll::Ready(
            frame
                .map(|result| result.map(|frame| frame.map_data(|bytes| self.permit.bytes(bytes)))),
        )
    }

    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.body.size_hint()
    }
}
