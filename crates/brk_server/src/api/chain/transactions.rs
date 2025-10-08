use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    str::FromStr,
};

use aide::{
    axum::{ApiRouter, routing::get_with},
    transform::TransformOperation,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use bitcoin::{Transaction as BitcoinTransaction, consensus::Decodable};
use brk_parser::XORIndex;
use brk_structs::{TxIndex, Txid, TxidPrefix};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::VecIterator;

use crate::extended::{ResponseExtended, TransformResponseExtended};

use super::AppState;

#[derive(Serialize, JsonSchema)]
/// Transaction Information
struct TransactionInfo {
    #[schemars(
        with = "String",
        example = "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
    )]
    txid: Txid,
    #[schemars(example = TxIndex::new(0))]
    index: TxIndex,
    #[serde(flatten)]
    #[schemars(with = "serde_json::Value")]
    tx: BitcoinTransaction,
}

#[derive(Deserialize, JsonSchema)]
struct TxidPath {
    /// Bitcoin transaction id
    #[schemars(example = &"4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b")]
    txid: String,
}

async fn get_transaction_info(
    Path(TxidPath { txid }): Path<TxidPath>,
    state: State<AppState>,
) -> Result<Response, (StatusCode, Json<&'static str>)> {
    let Ok(txid) = bitcoin::Txid::from_str(&txid) else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json("The provided TXID appears to be invalid."),
        ));
    };

    let txid = Txid::from(txid);
    let prefix = TxidPrefix::from(&txid);
    let interface = state.interface;
    let indexer = interface.indexer();
    let Ok(Some(index)) = indexer
        .stores
        .txidprefix_to_txindex
        .get(&prefix)
        .map(|opt| opt.map(|cow| cow.into_owned()))
    else {
        return Err((
            StatusCode::NOT_FOUND,
            Json("Failed to found the TXID in the blockchain."),
        ));
    };

    let txid = indexer.vecs.txindex_to_txid.iter().unwrap_get_inner(index);

    let parser = interface.parser();
    let computer = interface.computer();

    let position = computer
        .blks
        .txindex_to_position
        .iter()
        .unwrap_get_inner(index);
    let len = indexer
        .vecs
        .txindex_to_total_size
        .iter()
        .unwrap_get_inner(index);

    let blk_index_to_blk_path = parser.blk_index_to_blk_path();

    let Some(blk_path) = blk_index_to_blk_path.get(&position.blk_index()) else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to read the transaction (get blk's path)"),
        ));
    };

    let mut xori = XORIndex::default();
    xori.add_assign(position.offset() as usize);

    let Ok(mut file) = File::open(blk_path) else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to read the transaction (open file)"),
        ));
    };

    if file
        .seek(SeekFrom::Start(position.offset() as u64))
        .is_err()
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to read the transaction (file seek)"),
        ));
    }

    let mut buffer = vec![0u8; *len as usize];
    if file.read_exact(&mut buffer).is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to read the transaction (read exact)"),
        ));
    }
    xori.bytes(&mut buffer, parser.xor_bytes());

    let mut reader = Cursor::new(buffer);
    let Ok(tx) = BitcoinTransaction::consensus_decode(&mut reader) else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed decode the transaction"),
        ));
    };

    let tx_info = TransactionInfo { txid, index, tx };

    let bytes = sonic_rs::to_vec(&tx_info).unwrap();

    Ok(Response::new_json_from_bytes(bytes))
}

fn get_transaction_info_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Chain")
        .summary("Transaction information")
        .description(
            "Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files.",
        )
        .with_ok_response::<TransactionInfo, _>(|res| res)
        .with_not_modified()
        .with_bad_request()
        .with_not_found()
        .with_server_error()
}

pub trait TransactionsRoutes {
    fn add_transactions_routes(self) -> Self;
}

impl TransactionsRoutes for ApiRouter<AppState> {
    fn add_transactions_routes(self) -> Self {
        self.api_route(
            "/api/chain/tx/{txid}",
            get_with(get_transaction_info, get_transaction_info_docs),
        )
    }
}
