use brk_error::{Error, Result};
use corepc_jsonrpc::Response;
use serde::de::DeserializeOwned;

pub const MAX_RESPONSE_BYTES: usize = 64 * 1024;

/// Decode one bounded request/response exchange with our fixed request id.
pub fn decode<T: DeserializeOwned>(success: bool, bytes: &[u8]) -> Result<T> {
    let response: Response = serde_json::from_slice(bytes)?;
    if response.id != 1
        || response
            .jsonrpc
            .as_deref()
            .is_some_and(|version| version != "2.0")
    {
        return Err(Error::Internal("invalid node RPC response identity"));
    }
    let result = response.result()?;
    if !success {
        return Err(Error::Internal("node RPC HTTP error"));
    }
    Ok(result)
}
