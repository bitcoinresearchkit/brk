use std::str::FromStr;

use schemars::JsonSchema;

use brk_types::Txid;

const MAX_TXIDS: usize = 250;

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
                if txids.len() == MAX_TXIDS {
                    return Err(format!("too many txids, max {MAX_TXIDS} per request"));
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = "0000000000000000000000000000000000000000000000000000000000000001";
    const T2: &str = "0000000000000000000000000000000000000000000000000000000000000002";

    #[test]
    fn parses_empty_single_and_multi() {
        assert!(TxidsParam::from_query("").unwrap().txids.is_empty());
        assert_eq!(TxidsParam::from_query(&format!("txId[]={T1}")).unwrap().txids.len(), 1);
        assert_eq!(
            TxidsParam::from_query(&format!("txId%5B%5D={T1}&txId[]={T2}"))
                .unwrap()
                .txids
                .len(),
            2,
        );
    }

    #[test]
    fn rejects_unknown_key_and_invalid_txid() {
        assert!(TxidsParam::from_query("foo=bar").is_err());
        assert!(TxidsParam::from_query("txId[]=notahex").is_err());
        assert!(TxidsParam::from_query("noequals").is_err());
    }
}
