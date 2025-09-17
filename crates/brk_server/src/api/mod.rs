use std::str::FromStr;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use bitcoin::{Address, Network, absolute::LockTime};
use bitcoincore_rpc::bitcoin;
use brk_interface::{IdParam, Index, PaginatedIndexParam, PaginationParam, Params, ParamsOpt};
use brk_structs::{
    AddressBytesHash, AnyAddressDataIndexEnum, Bitcoin, OutputType, Txid, TxidPrefix,
};
use serde_json::Number;
use vecdb::{AnyIterableVec, VecIterator};

use super::AppState;

mod explorer;
mod vecs;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

const TO_SEPARATOR: &str = "_to_";

impl ApiRoutes for Router<AppState> {
    fn add_api_routes(self) -> Self {
        self.route(
            "/api/address/{address}",
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
                    dbg!(&hash);
                    dbg!(
                        &address,
                        address.address_type(),
                        address.script_pubkey(),
                        OutputType::from(&address)
                    );

                    let Ok(Some(addri)) = stores
                        .addressbyteshash_to_typeindex
                        .get(&hash)
                        .map(|opt| opt.map(|cow| cow.into_owned())) else {
                            return "Unknown address".into_response();
                        };

                    println!("Script pubkey: {}", address.script_pubkey());
                    println!("Address type: {:?}", address.address_type());
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
                    Json(serde_json::json!({
                        "address": address,
                        "type": output_type,
                        "index": addri,
                        "chain_stats": {
                            "funded_txo_count":	serde_json::Value::Null,
                            "funded_txo_sum": addr_data.received,
                            "spent_txo_count": serde_json::Value::Null,
                            "spent_txo_sum": addr_data.sent,
                            "utxo_count": addr_data.utxos,
                            "balance": amount,
                            "balance_usd": price.map_or(serde_json::Value::Null, |p| serde_json::Value::Number(Number::from_f64( *(p * Bitcoin::from(amount))).unwrap())),
                            "realized_value": addr_data.realized_cap,
                            "tx_count":	serde_json::Value::Null,
                            "avg_cost_basis": addr_data.realized_price()
                        },
                        "mempool_stats": serde_json::Value::Null
                    }))
                    .into_response()
                },
            ),
        )
        .route(
            "/api/tx/{txid}",
            get(
                async |Path(txid): Path<String>, state: State<AppState>| -> Response {
                    let Ok(txid) = bitcoin::Txid::from_str(&txid) else {
                        return "Invalid txid".into_response()
                    };
                    let txid = Txid::from(txid);
                    let prefix = TxidPrefix::from(&txid);
                    let interface = state.interface;
                    let indexer = interface.indexer();
                    let Ok(Some(txindex)) = indexer
                        .stores
                        .txidprefix_to_txindex
                        .get(&prefix)
                        .map(|opt| opt.map(|cow| cow.into_owned())) else {
                            return "Unknown transaction".into_response();
                        };
                    let version = indexer
                        .vecs
                        .txindex_to_txversion
                        .iter()
                        .unwrap_get_inner(txindex);
                    let rawlocktime = indexer
                        .vecs
                        .txindex_to_rawlocktime
                        .iter()
                        .unwrap_get_inner(txindex);
                    let locktime = LockTime::from(rawlocktime);

                    Json(serde_json::json!({
                        "txid": txid,
                        "index": txindex,
                        "version": version,
                        "locktime": locktime
                    }))
                    .into_response()
                },
            ),
        )
        .route(
            "/api/vecs/index-count",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_index_count()).into_response()
            }),
        )
        .route(
            "/api/vecs/id-count",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_vecid_count()).into_response()
            }),
        )
        .route(
            "/api/vecs/vec-count",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_vec_count()).into_response()
            }),
        )
        .route(
            "/api/vecs/indexes",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_indexes()).into_response()
            }),
        )
        .route(
            "/api/vecs/accepted-indexes",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_accepted_indexes()).into_response()
            }),
        )
        .route(
            "/api/vecs/ids",
            get(
                async |State(app_state): State<AppState>,
                       Query(pagination): Query<PaginationParam>|
                       -> Response {
                    Json(app_state.interface.get_vecids(pagination)).into_response()
                },
            ),
        )
        .route(
            "/api/vecs/index-to-ids",
            get(
                async |State(app_state): State<AppState>,
                       Query(paginated_index): Query<PaginatedIndexParam>|
                       -> Response {
                    Json(app_state.interface.get_index_to_vecids(paginated_index)).into_response()
                },
            ),
        )
        .route(
            "/api/vecs/id-to-indexes",
            get(
                async |State(app_state): State<AppState>,
                       Query(param): Query<IdParam>|
                       -> Response {
                    Json(app_state.interface.get_vecid_to_indexes(param.id)).into_response()
                },
            ),
        )
        // .route("/api/vecs/variants", get(variants_handler))
        .route("/api/vecs/query", get(vecs::handler))
        .route(
            "/api/vecs/{variant}",
            get(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(variant): Path<String>,
                       Query(params_opt): Query<ParamsOpt>,
                       state: State<AppState>|
                       -> Response {
                    let variant = variant.replace("-", "_");
                    let mut split = variant.split(TO_SEPARATOR);

                    if let Ok(index) = Index::try_from(split.next().unwrap()) {
                        let params = Params::from((
                            (index, split.collect::<Vec<_>>().join(TO_SEPARATOR)),
                            params_opt,
                        ));
                        vecs::handler(uri, headers, Query(params), state).await
                    } else {
                        "Bad variant".into_response()
                    }
                },
            ),
        )
        .route(
            "/api",
            get(|| async {
                Redirect::temporary(
                    "https://github.com/bitcoinresearchkit/brk/tree/main/crates/brk_server#api",
                )
            }),
        )
    }
}
