use std::{collections::BTreeMap, path::PathBuf};

use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use color_eyre::{eyre::eyre, owo_colors::OwoColorize};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    io::{Json, COMPRESSED_BIN_EXTENSION, JSON_EXTENSION},
    server::{
        api::{
            structs::{Chunk, Kind, Route},
            API_URL_PREFIX,
        },
        header_map::HeaderMapUtils,
        log_result, AppState,
    },
    structs::{Date, DateMap, Height, HeightMap, MapChunkId, HEIGHT_MAP_CHUNK_SIZE, OHLC},
};

use super::{
    extension::Extension,
    response::{typed_value_to_response, value_to_response},
};

#[derive(Deserialize)]
pub struct Params {
    chunk: Option<usize>,
    all: Option<bool>,
}

pub async fn dataset_handler(
    headers: HeaderMap,
    path: Path<String>,
    query: Query<Params>,
    State(app_state): State<AppState>,
) -> Response {
    match _dataset_handler(headers, &path, &query, app_state) {
        Ok(response) => {
            log_result(
                response.status(),
                &format!(
                    "{API_URL_PREFIX}/{}{}{}",
                    path.0,
                    query.chunk.map_or("".to_string(), |chunk| format!(
                        "{}{chunk}",
                        "?chunk=".bright_black()
                    )),
                    query.all.map_or("".to_string(), |all| format!(
                        "{}{all}",
                        "?all=".bright_black()
                    ))
                ),
            );

            response
        }
        Err(error) => {
            let code = StatusCode::INTERNAL_SERVER_ERROR;

            log_result(code, &format!("{API_URL_PREFIX}/{}", path.0));

            let mut response = (code, error.to_string()).into_response();

            response.headers_mut().insert_cors();

            response
        }
    }
}

const DATE_PREFIX: &str = "date-to-";
const HEIGHT_PREFIX: &str = "height-to-";

fn _dataset_handler(
    headers: HeaderMap,
    Path(path): &Path<String>,
    query: &Query<Params>,
    AppState { routes }: AppState,
) -> color_eyre::Result<Response> {
    if query.chunk.is_some() && query.all.is_some() {
        return Err(eyre!("chunk and all are exclusive"));
    }

    let (kind, id, route) = if path.starts_with(DATE_PREFIX) {
        let id = convert_path_to_id(path.strip_prefix(DATE_PREFIX).unwrap());
        let route = routes.date.get(&id);
        (Kind::Date, id, route)
    } else if path.starts_with(HEIGHT_PREFIX) {
        let id = convert_path_to_id(path.strip_prefix(HEIGHT_PREFIX).unwrap());
        let route = routes.height.get(&id);
        (Kind::Height, id, route)
    } else {
        let id = convert_path_to_id(path);
        let route = routes.last.get(&id);
        (Kind::Last, id, route)
    };

    if route.is_none() {
        return Err(eyre!("Path error"));
    }

    let mut route = route.unwrap().to_owned();

    let mut chunk = None;

    if query.all.map_or(true, |b| !b) {
        match kind {
            Kind::Date => {
                let datasets = DateMap::<usize>::_read_dir(&route.file_path, &route.serialization);

                process_datasets(&headers, kind, &mut chunk, &mut route, query, datasets)?;
            }
            Kind::Height => {
                let datasets =
                    HeightMap::<usize>::_read_dir(&route.file_path, &route.serialization);

                process_datasets(&headers, kind, &mut chunk, &mut route, query, datasets)?;
            }
            Kind::Last => {
                if !route.values_type.ends_with("Value") {
                    route.file_path.set_extension(COMPRESSED_BIN_EXTENSION);
                } else {
                    route.file_path.set_extension(JSON_EXTENSION);
                }
            }
        }
    }

    let (date, response) = headers.check_if_modified_since(&route.file_path)?;

    if let Some(response) = response {
        return Ok(response);
    }

    let type_name = route.values_type.split("::").last().unwrap();

    let extension = Extension::from(&std::path::PathBuf::from(&path));

    let mut response = match type_name {
        "u8" => typed_value_to_response::<u8>(kind, &route, chunk, id, extension)?,
        "u16" => typed_value_to_response::<u16>(kind, &route, chunk, id, extension)?,
        "u32" => typed_value_to_response::<u32>(kind, &route, chunk, id, extension)?,
        "u64" => typed_value_to_response::<u64>(kind, &route, chunk, id, extension)?,
        "usize" => typed_value_to_response::<usize>(kind, &route, chunk, id, extension)?,
        "f32" => typed_value_to_response::<f32>(kind, &route, chunk, id, extension)?,
        "f64" => typed_value_to_response::<f64>(kind, &route, chunk, id, extension)?,
        "OHLC" => typed_value_to_response::<OHLC>(kind, &route, chunk, id, extension)?,
        "Date" => typed_value_to_response::<Date>(kind, &route, chunk, id, extension)?,
        "Height" => typed_value_to_response::<Height>(kind, &route, chunk, id, extension)?,
        "Value" => {
            value_to_response::<serde_json::Value>(Json::import(&route.file_path)?, extension)
        }
        _ => panic!("Incompatible type: {type_name}"),
    };

    let headers = response.headers_mut();
    headers.insert_last_modified(date);

    Ok(response)
}

fn convert_path_to_id(s: &str) -> String {
    Extension::remove_extension(s).replace('-', "_")
}

fn process_datasets<ChunkId>(
    headers: &HeaderMap,
    kind: Kind,
    chunk: &mut Option<Chunk>,
    route: &mut Route,
    query: &Query<Params>,
    datasets: BTreeMap<ChunkId, PathBuf>,
) -> color_eyre::Result<()>
where
    ChunkId: MapChunkId,
{
    let (last_chunk_id, _) = datasets.last_key_value().unwrap_or_else(|| {
        dbg!(&datasets, &route);
        panic!()
    });

    let chunk_id = query
        .chunk
        .map(|id| ChunkId::from_usize(id))
        .unwrap_or(*last_chunk_id);

    let path = datasets.get(&chunk_id);

    if path.is_none() {
        return Err(eyre!("Couldn't find chunk"));
    }

    let path = path.unwrap();
    route.file_path = path.clone();

    let offset = match kind {
        Kind::Date => 1,
        Kind::Height => HEIGHT_MAP_CHUNK_SIZE as usize,
        _ => panic!(),
    };

    let offsetted_to_url = |offseted| {
        datasets.get(&ChunkId::from_usize(offseted)).map(|_| {
            let scheme = headers.get_scheme();
            let host = headers.get_host();
            format!("{scheme}://{host}/api/{}?chunk={offseted}", route.url_path)
        })
    };

    let chunk_id = chunk_id.to_usize();

    chunk.replace(Chunk {
        id: chunk_id,
        next: chunk_id.checked_add(offset).and_then(offsetted_to_url),
        previous: chunk_id.checked_sub(offset).and_then(offsetted_to_url),
    });

    Ok(())
}
