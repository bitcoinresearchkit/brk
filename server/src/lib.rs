use std::{collections::BTreeMap, time::Instant};

use api::{
    structs::{Index, IndexTypeToIndexEnum},
    ApiRoutes,
};
use axum::{body::Body, response::IntoResponse, routing::get, serve, Router};
use color_eyre::owo_colors::OwoColorize;
use computer::Computer;
use derive_deref::{Deref, DerefMut};
use indexer::Indexer;
use logger::{error, info};
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Value;
use storable_vec::{StorableVecIndex, StorableVecType, STATELESS};
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use website::WebsiteRoutes;

mod api;
mod traits;
mod website;

#[derive(Clone)]
pub struct AppState {
    vecs: &'static VecIdToIndexToVec,
    indexer: &'static Indexer<STATELESS>,
    computer: &'static Computer<STATELESS>,
}

#[derive(Default, Deref, DerefMut)]
pub struct VecIdToIndexToVec(BTreeMap<String, IndexToVec>);

impl VecIdToIndexToVec {
    pub fn insert<I, T>(&mut self, vec: &'static storable_vec::StorableVec<I, T, STATELESS>)
    where
        I: StorableVecIndex + IndexTypeToIndexEnum + Send + Sync,
        T: StorableVecType + Send + Sync + Serialize,
    {
        let file_name = vec.file_name();
        let split = file_name.split("_to_").collect::<Vec<_>>();
        if split.len() != 2 {
            panic!();
        }
        let index = vec.key_to_enum();
        if split[0] != index.to_string().to_lowercase() {
            dbg!(split[0], index.to_string());
            panic!();
        }
        let key = split[1].to_string().replace("_", "-");
        let prev = self.entry(key).or_default().insert(index, vec);
        if prev.is_some() {
            panic!()
        }
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct IndexToVec {
    pub index_to_vec: BTreeMap<Index, &'static (dyn AnyStatelessStorableVec + Send + Sync)>,
}

pub trait AnyStatelessStorableVec {
    fn key_to_enum(&self) -> Index;
    fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> storable_vec::Result<Vec<Value>>;
    // fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> storable_vec::Result<Vec<T>>;
}

impl<I, T> AnyStatelessStorableVec for storable_vec::StorableVec<I, T, STATELESS>
where
    I: StorableVecIndex + IndexTypeToIndexEnum + Send + Sync,
    T: StorableVecType + Send + Sync + Serialize,
{
    fn key_to_enum(&self) -> Index {
        I::to_enum()
    }

    fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> storable_vec::Result<Vec<Value>> {
        Ok(self
            .collect_range(from, to)?
            .into_iter()
            .map(|v| serde_json::to_value(v).unwrap())
            .collect::<Vec<_>>())
    }
}

trait StatelessVecs {
    fn parse(&'static self, vecs: &mut VecIdToIndexToVec);
}

impl StatelessVecs for Indexer<STATELESS> {
    fn parse(&'static self, vecs: &mut VecIdToIndexToVec) {
        vecs.insert(&self.vecs.addressindex_to_addresstype);
        vecs.insert(&self.vecs.addressindex_to_addresstypeindex);
        vecs.insert(&self.vecs.addressindex_to_height);
        vecs.insert(&self.vecs.height_to_blockhash);
        vecs.insert(&self.vecs.height_to_difficulty);
        vecs.insert(&self.vecs.height_to_first_addressindex);
        vecs.insert(&self.vecs.height_to_first_emptyindex);
        vecs.insert(&self.vecs.height_to_first_multisigindex);
        vecs.insert(&self.vecs.height_to_first_opreturnindex);
        vecs.insert(&self.vecs.height_to_first_pushonlyindex);
        vecs.insert(&self.vecs.height_to_first_txindex);
        vecs.insert(&self.vecs.height_to_first_txinindex);
        vecs.insert(&self.vecs.height_to_first_txoutindex);
        vecs.insert(&self.vecs.height_to_first_unknownindex);
        vecs.insert(&self.vecs.height_to_first_p2pk33index);
        vecs.insert(&self.vecs.height_to_first_p2pk65index);
        vecs.insert(&self.vecs.height_to_first_p2pkhindex);
        vecs.insert(&self.vecs.height_to_first_p2shindex);
        vecs.insert(&self.vecs.height_to_first_p2trindex);
        vecs.insert(&self.vecs.height_to_first_p2wpkhindex);
        vecs.insert(&self.vecs.height_to_first_p2wshindex);
        vecs.insert(&self.vecs.height_to_size);
        vecs.insert(&self.vecs.height_to_timestamp);
        vecs.insert(&self.vecs.height_to_weight);
        vecs.insert(&self.vecs.p2pk33index_to_p2pk33addressbytes);
        vecs.insert(&self.vecs.p2pk65index_to_p2pk65addressbytes);
        vecs.insert(&self.vecs.p2pkhindex_to_p2pkhaddressbytes);
        vecs.insert(&self.vecs.p2shindex_to_p2shaddressbytes);
        vecs.insert(&self.vecs.p2trindex_to_p2traddressbytes);
        vecs.insert(&self.vecs.p2wpkhindex_to_p2wpkhaddressbytes);
        vecs.insert(&self.vecs.p2wshindex_to_p2wshaddressbytes);
        vecs.insert(&self.vecs.txindex_to_first_txinindex);
        vecs.insert(&self.vecs.txindex_to_first_txoutindex);
        vecs.insert(&self.vecs.txindex_to_height);
        vecs.insert(&self.vecs.txindex_to_locktime);
        vecs.insert(&self.vecs.txindex_to_txid);
        vecs.insert(&self.vecs.txindex_to_txversion);
        vecs.insert(&self.vecs.txinindex_to_txoutindex);
        vecs.insert(&self.vecs.txoutindex_to_addressindex);
        vecs.insert(&self.vecs.txoutindex_to_amount);
    }
}

pub async fn main(indexer: Indexer<STATELESS>, computer: Computer<STATELESS>) -> color_eyre::Result<()> {
    // pub async fn main(routes: Routes, config: Config) -> color_eyre::Result<()> {
    // routes.generate_dts_file();

    let indexer = Box::leak(Box::new(indexer));
    let computer = Box::leak(Box::new(computer));
    let vecs = Box::leak(Box::new(VecIdToIndexToVec::default()));
    indexer.parse(vecs);

    let state = AppState {
        vecs,
        indexer,
        computer,
    };

    let compression_layer = CompressionLayer::new().br(true).deflate(true).gzip(true).zstd(true);

    let router = Router::new()
        .add_api_routes()
        .add_website_routes()
        .route("/version", get(env!("CARGO_PKG_VERSION")))
        .with_state(state)
        .layer(compression_layer);

    let mut port = 3110;

    let mut listener;
    loop {
        listener = TcpListener::bind(format!("0.0.0.0:{port}")).await;
        if listener.is_ok() {
            break;
        }
        port += 1;
    }

    info!("Starting server on port {port}...");

    let listener = listener.unwrap();

    serve(listener, router).await?;

    Ok(())
}

pub fn log_result(code: StatusCode, path: &str, instant: Instant) {
    let time = format!("{}µs", instant.elapsed().as_micros());
    let time = time.bright_black();
    match code {
        StatusCode::INTERNAL_SERVER_ERROR => error!("{} {} {}", code.as_u16().red(), path, time),
        StatusCode::NOT_MODIFIED => info!("{} {} {}", code.as_u16().bright_black(), path, time),
        StatusCode::OK => info!("{} {} {}", code.as_u16().green(), path, time),
        _ => error!("{} {} {}", code.as_u16().red(), path, time),
    }
}
