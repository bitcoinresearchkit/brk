use std::{collections::BTreeMap, path::PathBuf};

use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use color_eyre::{eyre::eyre, owo_colors::OwoColorize};
use reqwest::StatusCode;
use serde::Deserialize;

use parser::{
    log, Date, DateMap, Height, HeightMap, Json, MapChunkId, COMPRESSED_BIN_EXTENSION,
    HEIGHT_MAP_CHUNK_SIZE, JSON_EXTENSION, OHLC,
};

use crate::{
    api::structs::{Chunk, Kind, Route},
    header_map::HeaderMapUtils,
    AppState,
};

use super::response::{typed_value_to_response, value_to_response};

#[derive(Deserialize)]
pub struct Params {
    chunk: Option<usize>,
}

pub async fn dataset_handler(
    headers: HeaderMap,
    path: Path<String>,
    query: Query<Params>,
    State(app_state): State<AppState>,
) -> Response {
    match _dataset_handler(headers, path, query, app_state) {
        Ok(response) => response,
        Err(error) => {
            let mut response =
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();

            response.headers_mut().insert_cors();

            response
        }
    }
}

fn _dataset_handler(
    headers: HeaderMap,
    Path(path): Path<String>,
    query: Query<Params>,
    AppState { routes }: AppState,
) -> color_eyre::Result<Response> {
    log(&format!(
        "{}{}",
        path,
        query.chunk.map_or("".to_string(), |chunk| format!(
            "{}{chunk}",
            "?chunk=".bright_black()
        ))
    ));

    let date_prefix = "date-to-";
    let height_prefix = "height-to-";

    let (kind, route) = if path.starts_with(date_prefix) {
        (
            Kind::Date,
            routes.date.get(&replace_dash_by_underscore(
                path.strip_prefix(date_prefix).unwrap(),
            )),
        )
    } else if path.starts_with(height_prefix) {
        (
            Kind::Height,
            routes.height.get(&replace_dash_by_underscore(
                path.strip_prefix(height_prefix).unwrap(),
            )),
        )
    } else {
        (
            Kind::Last,
            routes.last.get(&replace_dash_by_underscore(&path)),
        )
    };

    if route.is_none() {
        return Err(eyre!("Path error"));
    }

    let mut route = route.unwrap().to_owned();

    let mut chunk = None;

    match kind {
        Kind::Date => {
            let datasets = DateMap::<usize>::_read_dir(&route.file_path, &route.serialization);

            process_datasets(&headers, kind, &mut chunk, &mut route, query, datasets)?;
        }
        Kind::Height => {
            let datasets = HeightMap::<usize>::_read_dir(&route.file_path, &route.serialization);

            process_datasets(&headers, kind, &mut chunk, &mut route, query, datasets)?;
        }
        Kind::Last => {
            if !route.values_type.ends_with("Value") {
                route.file_path.set_extension(COMPRESSED_BIN_EXTENSION);
            } else {
                route.file_path.set_extension(JSON_EXTENSION);
            }
        }
    };

    let (date, response) = headers.check_if_modified_since(&route.file_path).unwrap();

    if let Some(response) = response {
        return Ok(response);
    }

    let type_name = route.values_type.split("::").last().unwrap();

    let mut response = match type_name {
        "u8" => typed_value_to_response::<u8>(kind, &route, chunk)?,
        "u16" => typed_value_to_response::<u16>(kind, &route, chunk)?,
        "u32" => typed_value_to_response::<u32>(kind, &route, chunk)?,
        "u64" => typed_value_to_response::<u64>(kind, &route, chunk)?,
        "usize" => typed_value_to_response::<usize>(kind, &route, chunk)?,
        "f32" => typed_value_to_response::<f32>(kind, &route, chunk)?,
        "f64" => typed_value_to_response::<f64>(kind, &route, chunk)?,
        "OHLC" => typed_value_to_response::<OHLC>(kind, &route, chunk)?,
        "Date" => typed_value_to_response::<Date>(kind, &route, chunk)?,
        "Height" => typed_value_to_response::<Height>(kind, &route, chunk)?,
        "Value" => value_to_response::<serde_json::Value>(Json::import(&route.file_path)?),
        _ => panic!("Incompatible type: {type_name}"),
    };

    let headers = response.headers_mut();
    headers.insert_last_modified(date);

    Ok(response)
}

fn replace_dash_by_underscore(s: &str) -> String {
    s.replace('-', "_")
}

fn process_datasets<ChunkId>(
    headers: &HeaderMap,
    kind: Kind,
    chunk: &mut Option<Chunk>,
    route: &mut Route,
    query: Query<Params>,
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
