use schemars::JsonSchema;
use serde::Deserialize;

use crate::Txid;

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct AddressTxidsParam {
    /// Txid to paginate from (return transactions before this one)
    pub after_txid: Option<Txid>,
    /// Maximum number of results to return. Defaults to 25 if not specified.
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    25
}
