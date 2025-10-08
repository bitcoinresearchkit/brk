use std::str::FromStr;

use aide::{
    axum::{ApiRouter, routing::get_with},
    transform::TransformOperation,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use bitcoin::{Address as BitcoinAddress, Network, PublicKey, ScriptBuf};
use brk_structs::{
    AddressBytes, AddressBytesHash, AnyAddressDataIndexEnum, Bitcoin, Dollars, OutputType, Sats,
    TypeIndex,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{AnyIterableVec, VecIterator};

use crate::extended::TransformResponseExtended;

use super::AppState;

#[derive(Debug, Serialize, JsonSchema)]
/// Address information
struct AddressInfo {
    /// Bitcoin address string
    #[schemars(example = &"04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f")]
    address: String,

    #[schemars(example = OutputType::P2PK65)]
    r#type: OutputType,

    #[schemars(example = TypeIndex::new(0))]
    type_index: TypeIndex,

    /// Total satoshis ever sent from this address
    #[schemars(example = Sats::new(0))]
    total_sent: Sats,

    /// Total satoshis ever received by this address
    #[schemars(example = Sats::new(5001008380))]
    total_received: Sats,

    /// Number of unspent transaction outputs (UTXOs)
    #[schemars(example = 10)]
    utxo_count: u32,

    /// Current spendable balance in satoshis (total_received - total_sent)
    #[schemars(example = Sats::new(5001008380))]
    balance: Sats,

    /// Current balance value in USD at current market price
    #[schemars(example = Some(Dollars::mint(6_157_891.64)))]
    balance_usd: Option<Dollars>,

    /// Estimated total USD value at time of deposit for coins currently in this address (not including coins that were later sent out). Not suitable for tax calculations
    #[schemars(example = Some(Dollars::mint(6.2)))]
    estimated_total_invested: Option<Dollars>,

    /// Estimated average BTC price at time of deposit for coins currently in this address (USD). Not suitable for tax calculations
    #[schemars(example = Some(Dollars::mint(0.12)))]
    estimated_avg_entry_price: Option<Dollars>,
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

#[derive(Deserialize, JsonSchema)]
struct AddressPath {
    /// Bitcoin address string
    #[schemars(example = &"04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f")]
    address: String,
}

async fn get_address_info(
    Path(AddressPath { address }): Path<AddressPath>,
    state: State<AppState>,
) -> Result<Json<AddressInfo>, (StatusCode, Json<&'static str>)> {
    let interface = state.interface;
    let indexer = interface.indexer();
    let computer = interface.computer();
    let stores = &indexer.stores;

    let script = if let Ok(address) = BitcoinAddress::from_str(&address) {
        if !address.is_valid_for_network(Network::Bitcoin) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json("The provided address isn't the Bitcoin Network."),
            ));
        }
        let address = address.assume_checked();
        address.script_pubkey()
    } else if let Ok(pubkey) = PublicKey::from_str(&address) {
        ScriptBuf::new_p2pk(&pubkey)
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json("The provided address is invalid."),
        ));
    };

    let type_ = OutputType::from(&script);
    let Ok(bytes) = AddressBytes::try_from((&script, type_)) else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to convert the address to bytes"),
        ));
    };
    let hash = AddressBytesHash::from((&bytes, type_));

    let Ok(Some(type_index)) = stores
        .addressbyteshash_to_typeindex
        .get(&hash)
        .map(|opt| opt.map(|cow| cow.into_owned()))
    else {
        return Err((
            StatusCode::NOT_FOUND,
            Json("Address not found in the blockchain (no transaction history)"),
        ));
    };

    let stateful = &computer.stateful;
    let price = computer.price.as_ref().map(|v| {
        *v.timeindexes_to_price_close
            .dateindex
            .as_ref()
            .unwrap()
            .iter()
            .last()
            .unwrap()
            .1
            .into_owned()
    });

    let any_address_index = match type_ {
        OutputType::P2PK33 => stateful
            .p2pk33addressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2PK65 => stateful
            .p2pk65addressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2PKH => stateful
            .p2pkhaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2SH => stateful
            .p2shaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2TR => stateful
            .p2traddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2WPKH => stateful
            .p2wpkhaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2WSH => stateful
            .p2wshaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        OutputType::P2A => stateful
            .p2aaddressindex_to_anyaddressindex
            .iter()
            .unwrap_get_inner(type_index.into()),
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json("The provided address uses an unsupported type"),
            ));
        }
    };

    let address_data = match any_address_index.to_enum() {
        AnyAddressDataIndexEnum::Loaded(index) => stateful
            .loadedaddressindex_to_loadedaddressdata
            .iter()
            .unwrap_get_inner(index),
        AnyAddressDataIndexEnum::Empty(index) => stateful
            .emptyaddressindex_to_emptyaddressdata
            .iter()
            .unwrap_get_inner(index)
            .into(),
    };

    let balance = address_data.balance();

    Ok(Json(AddressInfo {
        address: address.to_string(),
        r#type: type_,
        type_index,
        utxo_count: address_data.utxo_count,
        total_sent: address_data.sent,
        total_received: address_data.received,
        balance,
        balance_usd: price.map(|p| p * Bitcoin::from(balance)),
        estimated_total_invested: price.map(|_| address_data.realized_cap),
        estimated_avg_entry_price: price.map(|_| address_data.realized_price()),
    }))
}

fn get_address_info_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Chain")
        .summary("Address information")
        .description("Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.).")
        .with_ok_response::<AddressInfo, _>(|res| res)
        .with_not_modified()
        .with_bad_request()
        .with_not_found()
        .with_server_error()
}

pub trait AddressesRoutes {
    fn add_addresses_routes(self) -> Self;
}

impl AddressesRoutes for ApiRouter<AppState> {
    fn add_addresses_routes(self) -> Self {
        self.api_route(
            "/api/chain/address/{address}",
            get_with(get_address_info, get_address_info_docs),
        )
    }
}
