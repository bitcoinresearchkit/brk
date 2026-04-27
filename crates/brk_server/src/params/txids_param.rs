use std::str::FromStr;

use schemars::JsonSchema;

use brk_types::Txid;

/// Query parameter for transaction-times endpoint.
#[derive(JsonSchema)]
pub struct TxidsParam {
    #[serde(rename = "txId[]")]
    pub txids: Vec<Txid>,
}

impl TxidsParam {
    /// Parsed manually from URI since serde_urlencoded doesn't support repeated keys.
    /// Rejects unknown keys to prevent cache-busting via injected query params.
    pub fn from_query(query: &str) -> Result<Self, String> {
        if query.is_empty() {
            return Ok(Self { txids: Vec::new() });
        }
        let mut txids = Vec::new();
        for pair in query.split('&') {
            let (key, val) = pair.split_once('=').ok_or_else(|| {
                format!("malformed query parameter `{pair}`, expected `txId[]=<txid>`")
            })?;
            if key == "txId[]" || key == "txId%5B%5D" {
                let txid = Txid::from_str(val).map_err(|e| format!("invalid txid `{val}`: {e}"))?;
                txids.push(txid);
            } else {
                return Err(format!(
                    "unknown query parameter `{key}`, expected `txId[]`"
                ));
            }
        }
        Ok(Self { txids })
    }
}
