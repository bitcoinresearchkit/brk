use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    str::FromStr,
};

use aide::{
    axum::{ApiRouter, IntoApiResponse, routing::get_with},
    transform::TransformOperation,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use bitcoin::{
    Address as BitcoinAddress, Network, Transaction as BitcoinTransaction, consensus::Decodable,
};
use brk_parser::XORIndex;
use brk_structs::{
    AddressBytesHash, AnyAddressDataIndexEnum, Bitcoin, Dollars, OutputType, Sats, TxIndex, Txid,
    TxidPrefix, TypeIndex,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{AnyIterableVec, VecIterator};

use super::AppState;

pub trait ApiExplorerRoutes {
    fn add_api_explorer_routes(self) -> Self;
}

#[derive(Debug, Serialize, JsonSchema)]
struct AddressDetails {
    /// Bitcoin address string
    address: String,

    r#type: OutputType,

    type_index: TypeIndex,

    /// Total satoshis ever sent from this address
    total_sent: Sats,

    /// Total satoshis ever received by this address
    total_received: Sats,

    /// Number of unspent transaction outputs (UTXOs)
    utxo_count: u32,

    /// Current spendable balance in satoshis (total_received - total_sent)
    balance: Sats,

    /// Current balance value in USD at current market price
    balance_usd: Option<Dollars>,

    /// Estimated total USD value at time of deposit for coins currently in this address (not including coins that were later sent out). Not suitable for tax calculations
    estimated_total_invested: Option<Dollars>,

    /// Estimated average BTC price at time of deposit for coins currently in this address (USD). Not suitable for tax calculations
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

#[derive(Serialize)]
struct TxResponse {
    txid: Txid,
    index: TxIndex,
    tx: BitcoinTransaction,
}

#[derive(Deserialize, JsonSchema)]
struct AddressPath {
    /// Bitcoin address string
    address: String,
}

#[derive(Deserialize, JsonSchema)]
struct TypeIndexPath {
    /// Index of type
    index: TypeIndex,
}

async fn get_address_details_from_address(
    Path(AddressPath { address }): Path<AddressPath>,
    state: State<AppState>,
) -> impl IntoApiResponse {
    let Ok(address) = BitcoinAddress::from_str(&address) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    if !address.is_valid_for_network(Network::Bitcoin) {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let address = address.assume_checked();
    let interface = state.interface;
    let indexer = interface.indexer();
    let computer = interface.computer();
    let stores = &indexer.stores;
    let hash = AddressBytesHash::from(&address);

    let r#type = OutputType::from(&address);

    let Ok(Some(type_index)) = stores
        .addressbyteshash_to_typeindex
        .get(&hash)
        .map(|opt| opt.map(|cow| cow.into_owned()))
    else {
        return StatusCode::NOT_FOUND.into_response();
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

    let any_address_index = match r#type {
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
        _ => unreachable!(),
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

    Json(AddressDetails {
        address: address.to_string(),
        r#type,
        type_index,
        utxo_count: address_data.utxo_count,
        total_sent: address_data.sent,
        total_received: address_data.received,
        balance,
        balance_usd: price.map(|p| p * Bitcoin::from(balance)),
        estimated_total_invested: price.map(|_| address_data.realized_cap),
        estimated_avg_entry_price: price.map(|_| address_data.realized_price()),
    })
    .into_response()
}

fn get_address_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Chain")
        .summary("Get address information")
        .description("Get Bitcoin address details")
        .response_with::<200, Json<AddressDetails>, _>(|res| {
            res.example(AddressDetails {
                address: "bc1qwzrryqr3ja8w7hnja2spmkgfdcgvqwp5swz4af4ngsjecfz0w0pqud7k38"
                    .to_string(),
                r#type: OutputType::P2WSH,
                type_index: TypeIndex::new(26158889),
                total_sent: Sats::new(5498948012620),
                total_received: Sats::new(5557954331207),
                utxo_count: 195,
                balance: Sats::new(59006318587),
                balance_usd: Some(Dollars::mint(73757839.22)),
                estimated_total_invested: Some(Dollars::mint(71943052.66)),
                estimated_avg_entry_price: Some(Dollars::mint(121900.0)),
            })
        })
        .response_with::<400, (), _>(|res| res.description("The address provided was invalid"))
        .response_with::<404, (), _>(|res| res.description("The address provided was not found"))
}

impl ApiExplorerRoutes for ApiRouter<AppState> {
    fn add_api_explorer_routes(self) -> Self {
        self.api_route(
            "/api/chain/address/{address}",
            get_with(get_address_details_from_address, get_address_docs),
        )
        .api_route(
            "/api/chain/address/p2pk65/{index}",
            get_with(get_address_details_from_address, get_address_docs),
        )
        .api_route(
            "/api/chain/address/p2pk33/{index}",
            get_with(get_address_details_from_address, get_address_docs),
        )
        .api_route(
            "/api/chain/address/p2pkh/{index}",
            get_with(get_address_details_from_address, get_address_docs),
        )
        .api_route(
            "/api/chain/address/p2sh/{index}",
            get_with(get_address_details_from_address, get_address_docs),
        )
        .api_route(
            "/api/chain/address/p2wpkh/{index}",
            get_with(get_address_details_from_address, get_address_docs),
        )
        .api_route(
            "/api/chain/address/p2wsh/{index}",
            get_with(get_address_details_from_address, get_address_docs),
        )
        .api_route(
            "/api/chain/address/p2tr/{index}",
            get_with(get_address_details_from_address, get_address_docs),
        )
        .api_route(
            "/api/chain/address/p2a/{index}",
            get_with(get_address_details_from_address, get_address_docs),
        )
        .route(
            "/api/chain/tx/{txid}",
            get(
                async |Path(txid): Path<String>, state: State<AppState>| -> Response {
                    let Ok(txid) = bitcoin::Txid::from_str(&txid) else {
                        return "Invalid txid".into_response();
                    };

                    let txid = Txid::from(txid);
                    let prefix = TxidPrefix::from(&txid);
                    let interface = state.interface;
                    let indexer = interface.indexer();
                    let Ok(Some(txindex)) = indexer
                        .stores
                        .txidprefix_to_txindex
                        .get(&prefix)
                        .map(|opt| opt.map(|cow| cow.into_owned()))
                    else {
                        return "Unknown transaction".into_response();
                    };

                    let txid = indexer
                        .vecs
                        .txindex_to_txid
                        .iter()
                        .unwrap_get_inner(txindex);

                    let parser = interface.parser();
                    let computer = interface.computer();

                    let position = computer
                        .blks
                        .txindex_to_position
                        .iter()
                        .unwrap_get_inner(txindex);
                    let len = indexer
                        .vecs
                        .txindex_to_total_size
                        .iter()
                        .unwrap_get_inner(txindex);

                    let blk_index_to_blk_path = parser.blk_index_to_blk_path();

                    let Some(blk_path) = blk_index_to_blk_path.get(&position.blk_index()) else {
                        return "Unknown blk index".into_response();
                    };

                    let mut xori = XORIndex::default();
                    xori.add_assign(position.offset() as usize);

                    let Ok(mut file) = File::open(blk_path) else {
                        return "Error opening blk file".into_response();
                    };

                    if file
                        .seek(SeekFrom::Start(position.offset() as u64))
                        .is_err()
                    {
                        return "Error seeking position in blk file".into_response();
                    }

                    let mut buffer = vec![0u8; *len as usize];
                    if file.read_exact(&mut buffer).is_err() {
                        return "File fail read exact".into_response();
                    }
                    xori.bytes(&mut buffer, parser.xor_bytes());

                    let mut reader = Cursor::new(buffer);
                    let Ok(tx) = BitcoinTransaction::consensus_decode(&mut reader) else {
                        return "Error decoding transaction".into_response();
                    };

                    let response = TxResponse {
                        txid,
                        index: txindex,
                        tx,
                    };

                    let bytes = sonic_rs::to_vec(&response).unwrap();

                    Response::builder()
                        .header("content-type", "application/json")
                        .body(bytes.into())
                        .unwrap()
                },
            ),
        )
    }
}
