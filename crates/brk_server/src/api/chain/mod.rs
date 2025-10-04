use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    str::FromStr,
};

use axum::{
    Json, Router,
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::get,
};
use bitcoin::{Address, Network, Transaction, consensus::Decodable};
use brk_parser::XORIndex;
use brk_structs::{
    AddressBytesHash, AnyAddressDataIndexEnum, Bitcoin, OutputType, TxIndex, Txid, TxidPrefix,
};
use serde::Serialize;
use sonic_rs::{Number, Value};
use vecdb::{AnyIterableVec, VecIterator};

use super::AppState;

pub trait ApiExplorerRoutes {
    fn add_api_explorer_routes(self) -> Self;
}

#[derive(Serialize)]
struct TxResponse {
    txid: Txid,
    index: TxIndex,
    tx: Transaction,
}

impl ApiExplorerRoutes for Router<AppState> {
    fn add_api_explorer_routes(self) -> Self {
        self.route(
            "/api/chain/address/{address}",
            get(
                async |Path(address): Path<String>, state: State<AppState>| -> Response {
                    let Ok(address) = Address::from_str(&address) else {
                        return "Invalid address".into_response();
                    };
                    if !address.is_valid_for_network(Network::Bitcoin) {
                        return "Invalid address".into_response();
                    }
                    let address = address.assume_checked();
                    let interface = state.interface;
                    let indexer = interface.indexer();
                    let computer = interface.computer();
                    let stores = &indexer.stores;
                    let hash = AddressBytesHash::from(&address);

                    let Ok(Some(addri)) = stores
                        .addressbyteshash_to_typeindex
                        .get(&hash)
                        .map(|opt| opt.map(|cow| cow.into_owned()))
                    else {
                        return "Unknown address".into_response();
                    };

                    let output_type = OutputType::from(&address);
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

                    let anyaddri = match output_type {
                        OutputType::P2PK33 => stateful
                            .p2pk33addressindex_to_anyaddressindex
                            .iter()
                            .unwrap_get_inner(addri.into()),
                        OutputType::P2PK65 => stateful
                            .p2pk65addressindex_to_anyaddressindex
                            .iter()
                            .unwrap_get_inner(addri.into()),
                        OutputType::P2PKH => stateful
                            .p2pkhaddressindex_to_anyaddressindex
                            .iter()
                            .unwrap_get_inner(addri.into()),
                        OutputType::P2SH => stateful
                            .p2shaddressindex_to_anyaddressindex
                            .iter()
                            .unwrap_get_inner(addri.into()),
                        OutputType::P2TR => stateful
                            .p2traddressindex_to_anyaddressindex
                            .iter()
                            .unwrap_get_inner(addri.into()),
                        OutputType::P2WPKH => stateful
                            .p2wpkhaddressindex_to_anyaddressindex
                            .iter()
                            .unwrap_get_inner(addri.into()),
                        OutputType::P2WSH => stateful
                            .p2wshaddressindex_to_anyaddressindex
                            .iter()
                            .unwrap_get_inner(addri.into()),
                        OutputType::P2A => stateful
                            .p2aaddressindex_to_anyaddressindex
                            .iter()
                            .unwrap_get_inner(addri.into()),

                        _ => unreachable!(),
                    };

                    let addr_data = match anyaddri.to_enum() {
                        AnyAddressDataIndexEnum::Loaded(loadedi) => stateful
                            .loadedaddressindex_to_loadedaddressdata
                            .iter()
                            .unwrap_get_inner(loadedi),
                        AnyAddressDataIndexEnum::Empty(emptyi) => stateful
                            .emptyaddressindex_to_emptyaddressdata
                            .iter()
                            .unwrap_get_inner(emptyi)
                            .into(),
                    };

                    let amount = addr_data.amount();
                    Json(sonic_rs::json!({
                        "address": address,
                        "type": output_type,
                        "index": addri,
                        "chain_stats": {
                            "funded_txo_count":	null,
                            "funded_txo_sum": addr_data.received,
                            "spent_txo_count": null,
                            "spent_txo_sum": addr_data.sent,
                            "utxo_count": addr_data.utxos,
                            "balance": amount,
                            "balance_usd": price.map_or(Value::new(), |p| {
                                Value::from(Number::from_f64(*(p * Bitcoin::from(amount))).unwrap())
                            }),
                            "realized_value": addr_data.realized_cap,
                            "tx_count":	null,
                            "avg_cost_basis": addr_data.realized_price()
                        },
                        "mempool_stats": null
                    }))
                    .into_response()
                },
            ),
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
                    let Ok(tx) = Transaction::consensus_decode(&mut reader) else {
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
