use schemars::JsonSchema;
use serde::Deserialize;

use brk_types::Txid;

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct AddrTxidsParam {
    /// Txid to paginate from (return transactions before this one)
    pub after_txid: Option<Txid>,
}
