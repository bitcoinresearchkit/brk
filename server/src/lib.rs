use std::{collections::BTreeMap, time::Instant};

use api::{structs::Index, ApiRoutes};
use axum::{routing::get, serve, Json, Router};
use color_eyre::owo_colors::OwoColorize;
use computer::Computer;
use derive_deref::{Deref, DerefMut};
use indexer::Indexer;
use logger::{error, info};
use reqwest::StatusCode;
use storable_vec::{AnyJsonStorableVec, STATELESS};
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
    // Not the most performant or type safe but only built once so that's okay
    pub fn insert(&mut self, vec: &'static dyn AnyJsonStorableVec) {
        let file_name = vec.file_name();
        let split = file_name.split("_to_").collect::<Vec<_>>();
        if split.len() != 2 {
            panic!();
        }
        let str = vec.index_type_to_string().split("::").last().unwrap().to_lowercase();
        let index = Index::try_from(str.as_str())
            .inspect_err(|_| {
                dbg!(str);
            })
            .unwrap();
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
    pub index_to_vec: BTreeMap<Index, &'static dyn AnyJsonStorableVec>,
}

pub async fn main(indexer: Indexer<STATELESS>, computer: Computer<STATELESS>) -> color_eyre::Result<()> {
    // pub async fn main(routes: Routes, config: Config) -> color_eyre::Result<()> {
    // routes.generate_dts_file();

    let indexer = Box::leak(Box::new(indexer));
    let computer = Box::leak(Box::new(computer));
    let vecs = Box::leak(Box::new(VecIdToIndexToVec::default()));

    indexer
        .vecs
        .as_any_json_vec_slice()
        .into_iter()
        .for_each(|vec| vecs.insert(vec));

    let state = AppState {
        vecs,
        indexer,
        computer,
    };

    let compression_layer = CompressionLayer::new().br(true).deflate(true).gzip(true).zstd(true);

    let router = Router::new()
        .add_api_routes()
        .add_website_routes()
        .route("/version", get(Json(env!("CARGO_PKG_VERSION"))))
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
