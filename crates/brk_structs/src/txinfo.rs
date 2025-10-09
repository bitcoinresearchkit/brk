use schemars::JsonSchema;
use serde::Serialize;

use crate::{TxIndex, Txid};

#[derive(Serialize, JsonSchema)]
/// Transaction Information
pub struct TransactionInfo {
    #[schemars(
        with = "String",
        example = "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
    )]
    pub txid: Txid,
    #[schemars(example = TxIndex::new(0))]
    pub index: TxIndex,
    // #[serde(flatten)]
    // #[schemars(with = "serde_json::Value")]
    // pub tx: Transaction,
}
