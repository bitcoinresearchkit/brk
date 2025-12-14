use schemars::JsonSchema;
use serde::Deserialize;

use crate::Vout;

#[derive(Deserialize, JsonSchema)]
pub struct TxidVoutPath {
    /// Bitcoin transaction id
    #[schemars(example = &"4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b")]
    pub txid: String,

    /// Output index
    #[schemars(example = 0)]
    pub vout: Vout,
}
