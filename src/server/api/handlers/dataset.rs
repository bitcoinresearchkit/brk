use std::{fmt::Debug, path::PathBuf, time::Instant};

use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use bincode::Decode;
use chrono::{DateTime, Utc};
use color_eyre::eyre::{eyre, ContextCompat};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    server::{
        api::{
            structs::{ChunkMetadata, DatasetRange, DatasetRangeChunk, Extension, Kind, Route},
            API_URL_PREFIX,
        },
        header_map::HeaderMapUtils,
        log_result, AppState,
    },
    structs::{
        Date, GenericMap, Height, MapChunkId, MapKey, MapSerialized, MapValue, SerializedDateMap,
        SerializedVec, OHLC,
    },
};

#[derive(Deserialize)]
pub struct DatasetParams {
    pub chunk: Option<usize>,
    pub all: Option<bool>,
    pub kind: String,
}

pub async fn dataset_handler(
    headers: HeaderMap,
    path: Path<String>,
    query: Query<DatasetParams>,
    State(app_state): State<AppState>,
) -> Response {
    let instant = Instant::now();

    let ser_path = format!(
        "{API_URL_PREFIX}/{}?kind={}{}{}",
        path.0,
        query.kind,
        query
            .chunk
            .map_or("".to_string(), |chunk| format!("&chunk={chunk}")),
        query
            .all
            .map_or("".to_string(), |all| format!("&all={all}"))
    );

    match result_handler(headers, &path, &query, app_state) {
        Ok(response) => {
            log_result(response.status(), &ser_path, instant);
            response
        }
        Err(error) => {
            let mut response =
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();
            log_result(response.status(), &ser_path, instant);
            response.headers_mut().insert_cors();
            response
        }
    }
}

fn result_handler(
    headers: HeaderMap,
    Path(path): &Path<String>,
    query: &Query<DatasetParams>,
    AppState { routes, .. }: AppState,
) -> color_eyre::Result<Response> {
    let path_buf = PathBuf::from(&path);
    let id = path_buf.file_stem().unwrap().to_str().unwrap();
    let ext = Extension::from(&path_buf);

    let route = routes.get(id);
    if route.is_none() {
        return Err(eyre!("Wrong path"));
    }
    let route = route.unwrap();

    let type_name = route.type_name.as_str();
    Ok(match type_name {
        "u8" => typed_handler::<u8>(headers, id, ext, query, route)?,
        "u16" => typed_handler::<u16>(headers, id, ext, query, route)?,
        "u32" => typed_handler::<u32>(headers, id, ext, query, route)?,
        "u64" => typed_handler::<u64>(headers, id, ext, query, route)?,
        "usize" => typed_handler::<usize>(headers, id, ext, query, route)?,
        "f32" => typed_handler::<f32>(headers, id, ext, query, route)?,
        "f64" => typed_handler::<f64>(headers, id, ext, query, route)?,
        "OHLC" => typed_handler::<OHLC>(headers, id, ext, query, route)?,
        "Date" => typed_handler::<Date>(headers, id, ext, query, route)?,
        "Height" => typed_handler::<Height>(headers, id, ext, query, route)?,
        // "Value" => {
        //     value_to_response::<serde_json::Value>(Json::import(&route.file_path)?, extension)
        // }
        _ => panic!("Incompatible type: {type_name}"),
    })
}

fn typed_handler<T>(
    headers: HeaderMap,
    id: &str,
    ext: Option<Extension>,
    query: &Query<DatasetParams>,
    route: &Route,
) -> color_eyre::Result<Response>
where
    T: Serialize + Debug + DeserializeOwned + Decode + MapValue,
{
    let kind = Kind::try_from(&query.kind)?;
    if !route.list.contains(&kind) {
        return Err(eyre!("{kind:?} not supported for this dataset"));
    }

    let range = DatasetRange::try_from(query)?;

    let (mut response, date_modified) = match kind {
        Kind::Date => map_to_response::<Date, T, _, SerializedDateMap<T>>(
            id, headers, route, &ext, range, query,
        ),
        Kind::Height => map_to_response::<Height, T, _, SerializedVec<T>>(
            id, headers, route, &ext, range, query,
        ),
        Kind::Last => {
            let last_value: T = route.serialization.import(&route.path.join("last"))?;
            return Ok(axum::response::Json(last_value).into_response());
        }
    }?;

    let status_ok = response.status() == StatusCode::OK;
    let headers = response.headers_mut();

    headers.insert_cors();

    if status_ok {
        headers.insert_last_modified(date_modified);
    }

    match ext {
        Some(extension) => {
            headers.insert_content_disposition_attachment();
            match extension {
                Extension::CSV => headers.insert_content_type_text_csv(),
                Extension::JSON => headers.insert_content_type_application_json(),
            }
        }
        _ => headers.insert_content_type_application_json(),
    }

    Ok(response)
}

fn map_to_response<Key, Value, ChunkId, Serialized>(
    id: &str,
    headers: HeaderMap,
    route: &Route,
    ext: &Option<Extension>,
    range: DatasetRange,
    query: &Query<DatasetParams>,
) -> color_eyre::Result<(Response, DateTime<Utc>)>
where
    Key: MapKey<ChunkId>,
    Value: MapValue,
    ChunkId: MapChunkId,
    Serialized: MapSerialized<Key, Value, ChunkId>,
{
    let folder_path = route.path.join(Key::map_name());
    let serialization = &route.serialization;

    let date_modified;

    let datasets =
        GenericMap::<Key, Value, ChunkId, Serialized>::_read_dir(&folder_path, serialization);

    let mut chunk_meta = None;

    let dataset = if let DatasetRange::Chunk(range_chunk) = range {
        let chunk_id = match range_chunk {
            DatasetRangeChunk::Last => {
                *datasets
                    .last_key_value()
                    .context("Last tuple of dataset directory")?
                    .0
            }
            DatasetRangeChunk::Chunk(chunk) => ChunkId::from_usize(chunk),
        };

        let chunk_path = datasets.get(&chunk_id);
        if chunk_path.is_none() {
            return Err(eyre!("Couldn't find chunk"));
        }
        let chunk_path = chunk_path.unwrap();

        let (date, response) = headers.check_if_modified_since(chunk_path)?;
        date_modified = date;

        if let Some(response) = response {
            return Ok((response, date_modified));
        }

        let to_url = |chunk: Option<ChunkId>| {
            chunk.and_then(|chunk| {
                datasets.contains_key(&chunk).then(|| {
                    let scheme = headers.get_scheme();
                    let host = headers.get_host();
                    format!(
                        "{scheme}://{host}/api/{id}?kind={}&chunk={}",
                        query.kind,
                        chunk.to_usize()
                    )
                })
            })
        };

        chunk_meta.replace(ChunkMetadata {
            id: chunk_id.to_usize(),
            next: to_url(chunk_id.next()),
            previous: to_url(chunk_id.previous()),
        });

        serialization.import::<Serialized>(chunk_path)?
    } else {
        let newest_file = datasets
            .values()
            .max_by(|a, b| {
                a.metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .cmp(&b.metadata().unwrap().modified().unwrap())
            })
            .context("Expect to find newest file")?;

        let (date, response) = headers.check_if_modified_since(newest_file)?;
        date_modified = date;

        if let Some(response) = response {
            return Ok((response, date_modified));
        }

        Serialized::import_all(&folder_path, serialization)
    };

    let response = if *ext == Some(Extension::CSV) {
        dataset.to_csv(id).into_response()
    } else if let Some(chunk) = chunk_meta {
        axum::response::Json(SerializedMapChunk {
            chunk,
            map: dataset.map(),
            version: dataset.version(),
        })
        .into_response()
    } else {
        axum::response::Json(dataset).into_response()
    };

    Ok((response, date_modified))
}

#[derive(Serialize)]
struct SerializedMapChunk<T>
where
    T: Serialize,
{
    version: u32,
    chunk: ChunkMetadata,
    map: T,
}
