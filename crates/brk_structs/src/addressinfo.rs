use schemars::JsonSchema;
use serde::Serialize;

use crate::{Dollars, OutputType, Sats, TypeIndex};

#[derive(Debug, Serialize, JsonSchema)]
/// Address information
pub struct AddressInfo {
    /// Bitcoin address string
    #[schemars(example = &"04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f")]
    pub address: String,

    #[schemars(example = OutputType::P2PK65)]
    pub r#type: OutputType,

    #[schemars(example = TypeIndex::new(0))]
    pub type_index: TypeIndex,

    /// Total satoshis ever sent from this address
    #[schemars(example = Sats::new(0))]
    pub total_sent: Sats,

    /// Total satoshis ever received by this address
    #[schemars(example = Sats::new(5001008380))]
    pub total_received: Sats,

    /// Number of unspent transaction outputs (UTXOs)
    #[schemars(example = 10)]
    pub utxo_count: u32,

    /// Current spendable balance in satoshis (total_received - total_sent)
    #[schemars(example = Sats::new(5001008380))]
    pub balance: Sats,

    /// Current balance value in USD at current market price
    #[schemars(example = Some(Dollars::mint(6_157_891.64)))]
    pub balance_usd: Option<Dollars>,

    /// Estimated total USD value at time of deposit for coins currently in this address (not including coins that were later sent out). Not suitable for tax calculations
    #[schemars(example = Some(Dollars::mint(6.2)))]
    pub estimated_total_invested: Option<Dollars>,

    /// Estimated average BTC price at time of deposit for coins currently in this address (USD). Not suitable for tax calculations
    #[schemars(example = Some(Dollars::mint(0.12)))]
    pub estimated_avg_entry_price: Option<Dollars>,
    //
    // Transaction count?
    // First/last activity timestamps?
    // Realized/unrealized gains?
    // Current value (balance × current price)?
    // "address": address,
    // "type": output_type,
    // "index": addri,
    // "chain_stats": {
    //     "funded_txo_count":	null,
    //     "funded_txo_sum": addr_data.received,
    //     "spent_txo_count": null,
    //     "spent_txo_sum": addr_data.sent,
    //     "utxo_count": addr_data.utxos,
    //     "balance": amount,
    //     "balance_usd": price.map_or(Value::new(), |p| {
    //         Value::from(Number::from_f64(*(p * Bitcoin::from(amount))).unwrap())
    //     }),
    //     "realized_value": addr_data.realized_cap,
    //     "tx_count":	null,
    //     "avg_cost_basis": addr_data.realized_price()
    // },
    // "mempool_stats": null
}
