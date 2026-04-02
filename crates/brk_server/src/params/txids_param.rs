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
    pub fn from_query(query: &str) -> Self {
        Self {
            txids: query
                .split('&')
                .filter_map(|pair| {
                    let (key, val) = pair.split_once('=')?;
                    if key == "txId[]" || key == "txId%5B%5D" {
                        Txid::from_str(val).ok()
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}
