use aide::transform::{TransformOperation, TransformResponse};
use axum::Json;
use schemars::JsonSchema;

pub trait TransformResponseExtended<'t> {
    /// 200
    fn with_ok_response<R, F>(self, f: F) -> Self
    where
        R: JsonSchema,
        F: FnOnce(TransformResponse<'_, R>) -> TransformResponse<'_, R>;
    /// 400
    fn with_bad_request(self) -> Self;
    /// 404
    fn with_not_found(self) -> Self;
    /// 304
    fn with_not_modified(self) -> Self;
    /// 500
    fn with_server_error(self) -> Self;
}

impl<'t> TransformResponseExtended<'t> for TransformOperation<'t> {
    fn with_ok_response<R, F>(self, f: F) -> Self
    where
        R: JsonSchema,
        F: FnOnce(TransformResponse<'_, R>) -> TransformResponse<'_, R>,
    {
        self.response_with::<200, Json<R>, _>(|res| f(res.description("Successful response")))
    }

    fn with_bad_request(self) -> Self {
        self.response_with::<400, Json<String>, _>(|res| {
            res.description("Invalid request parameters")
        })
    }

    fn with_not_found(self) -> Self {
        self.response_with::<404, Json<String>, _>(|res| res.description("Resource not found"))
    }

    fn with_not_modified(self) -> Self {
        self.response_with::<304, (), _>(|res| {
            res.description("Not modified - content unchanged since last request")
        })
    }

    fn with_server_error(self) -> Self {
        self.response_with::<500, Json<String>, _>(|res| res.description("Internal server error"))
    }
}
