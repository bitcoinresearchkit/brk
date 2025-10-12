use aide::transform::{TransformOperation, TransformResponse};
use axum::Json;
use schemars::JsonSchema;

pub trait TransformResponseExtended<'t> {
    fn addresses_tag(self) -> Self;
    fn blocks_tag(self) -> Self;
    fn metrics_tag(self) -> Self;
    fn mining_tag(self) -> Self;
    fn server_tag(self) -> Self;
    fn transactions_tag(self) -> Self;

    /// 200
    fn ok_response<R>(self) -> Self
    where
        R: JsonSchema;
    /// 200
    fn ok_response_with<R, F>(self, f: F) -> Self
    where
        R: JsonSchema,
        F: FnOnce(TransformResponse<'_, R>) -> TransformResponse<'_, R>;
    /// 400
    fn bad_request(self) -> Self;
    /// 404
    fn not_found(self) -> Self;
    /// 304
    fn not_modified(self) -> Self;
    /// 500
    fn server_error(self) -> Self;
}

impl<'t> TransformResponseExtended<'t> for TransformOperation<'t> {
    fn addresses_tag(self) -> Self {
        self.tag("Addresses")
    }

    fn blocks_tag(self) -> Self {
        self.tag("Blocks")
    }

    fn metrics_tag(self) -> Self {
        self.tag("Metrics")
    }

    fn mining_tag(self) -> Self {
        self.tag("Mining")
    }

    fn server_tag(self) -> Self {
        self.tag("Server")
    }

    fn transactions_tag(self) -> Self {
        self.tag("Transactions")
    }

    fn ok_response<R>(self) -> Self
    where
        R: JsonSchema,
    {
        self.ok_response_with(|r: TransformResponse<'_, R>| r)
    }

    fn ok_response_with<R, F>(self, f: F) -> Self
    where
        R: JsonSchema,
        F: FnOnce(TransformResponse<'_, R>) -> TransformResponse<'_, R>,
    {
        self.response_with::<200, Json<R>, _>(|res| f(res.description("Successful response")))
    }

    fn bad_request(self) -> Self {
        self.response_with::<400, Json<String>, _>(|res| {
            res.description("Invalid request parameters")
        })
    }

    fn not_found(self) -> Self {
        self.response_with::<404, Json<String>, _>(|res| res.description("Resource not found"))
    }

    fn not_modified(self) -> Self {
        self.response_with::<304, (), _>(|res| {
            res.description("Not modified - content unchanged since last request")
        })
    }

    fn server_error(self) -> Self {
        self.response_with::<500, Json<String>, _>(|res| res.description("Internal server error"))
    }
}
