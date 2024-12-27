use std::{fmt::Debug, path::PathBuf, time::Instant};

use axum::{
    body::Body,
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
            structs::{
                ChunkMetadata, DatasetRange, DatasetRangeChunk, Extension, Kind, Route, Routes,
            },
            API_URL_PREFIX,
        },
        header_map::{HeaderMapExtended, Modified},
        log_result,
        response::ResponseExtended,
        AppState,
    },
    structs::{
        Date, GenericMap, Height, HeightMapChunkId, MapChunkId, MapKey, MapSerialized, MapValue,
        SerializedBTreeMap, SerializedDateMap, SerializedTimeMap, SerializedVec, Timestamp, OHLC,
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
        "u8" => typed_handler::<u8>(headers, id, ext, query, route, &routes)?,
        "u16" => typed_handler::<u16>(headers, id, ext, query, route, &routes)?,
        "u32" => typed_handler::<u32>(headers, id, ext, query, route, &routes)?,
        "u64" => typed_handler::<u64>(headers, id, ext, query, route, &routes)?,
        "usize" => typed_handler::<usize>(headers, id, ext, query, route, &routes)?,
        "f32" => typed_handler::<f32>(headers, id, ext, query, route, &routes)?,
        "f64" => typed_handler::<f64>(headers, id, ext, query, route, &routes)?,
        "OHLC" => typed_handler::<OHLC>(headers, id, ext, query, route, &routes)?,
        "Date" => typed_handler::<Date>(headers, id, ext, query, route, &routes)?,
        "Height" => typed_handler::<Height>(headers, id, ext, query, route, &routes)?,
        "Timestamp" => typed_handler::<Timestamp>(headers, id, ext, query, route, &routes)?,
        _ => panic!("Incompatible type: {type_name}"),
    })
}

fn typed_handler<T>(
    headers: HeaderMap,
    id: &str,
    ext: Option<Extension>,
    query: &Query<DatasetParams>,
    route: &Route,
    routes: &Routes,
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
        Kind::Last => {
            let last_value: T = route.serialization.import(&route.path.join("last"))?;
            return Ok(axum::response::Json(last_value).into_response());
        }
        Kind::Date => match read_serialized::<Date, T, _, SerializedDateMap<T>>(
            id, &headers, route, &range, query,
        )? {
            ReadSerialized::DatasetAndDate((dataset, date, chunk_meta)) => {
                (serialized_to_response(dataset, id, chunk_meta, ext), date)
            }
            ReadSerialized::NotModified => return Ok(Response::new_not_modified()),
            ReadSerialized::_Phantom(_) => unreachable!(),
        },
        Kind::Height => match read_serialized::<Height, T, _, SerializedVec<T>>(
            id, &headers, route, &range, query,
        )? {
            ReadSerialized::DatasetAndDate((dataset, date, chunk_meta)) => (
                serialized_to_response::<Height, T, _, SerializedVec<T>>(
                    dataset, id, chunk_meta, ext,
                ),
                date,
            ),
            ReadSerialized::NotModified => return Ok(Response::new_not_modified()),
            ReadSerialized::_Phantom(_) => unreachable!(),
        },
        Kind::Timestamp => {
            let (dataset, date, chunk_meta) = match read_serialized::<Height, T, _, SerializedVec<T>>(
                id, &headers, route, &range, query,
            )? {
                ReadSerialized::DatasetAndDate(tuple) => tuple,
                ReadSerialized::NotModified => return Ok(Response::new_not_modified()),
                ReadSerialized::_Phantom(_) => unreachable!(),
            };

            let (timestamp_dataset, _, _) =
                match read_serialized::<Height, Timestamp, _, SerializedVec<Timestamp>>(
                    "timestamp",
                    &headers,
                    routes.get("timestamp").unwrap(),
                    &range,
                    query,
                )? {
                    ReadSerialized::DatasetAndDate(tuple) => tuple,
                    ReadSerialized::NotModified => return Ok(Response::new_not_modified()),
                    ReadSerialized::_Phantom(_) => unreachable!(),
                };

            let mut serialized_timemap: SerializedTimeMap<T> = SerializedBTreeMap::default();

            dataset
                .map
                .into_iter()
                .enumerate()
                .for_each(|(index, value)| {
                    serialized_timemap.map.insert(
                        timestamp_dataset
                            .get_index(index)
                            .cloned()
                            .unwrap_or(Timestamp::now()),
                        value,
                    );
                });

            (
                serialized_to_response::<Timestamp, T, HeightMapChunkId, SerializedTimeMap<T>>(
                    serialized_timemap,
                    id,
                    chunk_meta,
                    ext,
                ),
                date,
            )

            // let m = read_serialized::<Height, T, _, SerializedVec<T>>(
            //     id, &headers, route, &range, query,
            // )?;
            // let t = read_serialized::<Height, Timestamp, _, SerializedVec<Timestamp>>(
            //     "timestamp",
            //     &headers,
            //     routes.get("timestamp").unwrap(),
            //     &range,
            //     query,
            // );
            // t
        }
    };

    let headers = response.headers_mut();

    headers.insert_cors();
    headers.insert_last_modified(date_modified);

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

fn serialized_to_response<Key, Value, ChunkId, Serialized>(
    dataset: Serialized,
    id: &str,
    chunk_meta: Option<ChunkMetadata>,
    ext: Option<Extension>,
) -> Response<Body>
where
    Key: MapKey<ChunkId>,
    Value: MapValue,
    ChunkId: MapChunkId,
    Serialized: MapSerialized<Key, Value, ChunkId>,
{
    if ext == Some(Extension::CSV) {
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
    }
}

enum ReadSerialized<Key, Value, ChunkId, Serialized>
where
    Key: MapKey<ChunkId>,
    Value: MapValue,
    ChunkId: MapChunkId,
    Serialized: MapSerialized<Key, Value, ChunkId>,
{
    DatasetAndDate((Serialized, DateTime<Utc>, Option<ChunkMetadata>)),
    NotModified,
    _Phantom((Key, Value, ChunkId)),
}

fn read_serialized<Key, Value, ChunkId, Serialized>(
    id: &str,
    headers: &HeaderMap,
    route: &Route,
    range: &DatasetRange,
    query: &Query<DatasetParams>,
) -> color_eyre::Result<ReadSerialized<Key, Value, ChunkId, Serialized>>
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
            DatasetRangeChunk::Chunk(chunk) => ChunkId::from_usize(*chunk),
        };

        let chunk_path = datasets.get(&chunk_id);
        if chunk_path.is_none() {
            return Err(eyre!("Couldn't find chunk"));
        }
        let chunk_path = chunk_path.unwrap();

        let (modified, date) = headers.check_if_modified_since(chunk_path)?;
        if modified == Modified::NotModifiedSince {
            return Ok(ReadSerialized::NotModified);
        }
        date_modified = date;

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

        let (modified, date) = headers.check_if_modified_since(newest_file)?;
        if modified == Modified::NotModifiedSince {
            return Ok(ReadSerialized::NotModified);
        }

        date_modified = date;

        Serialized::import_all(&folder_path, serialization)
    };

    Ok(ReadSerialized::DatasetAndDate((
        dataset,
        date_modified,
        chunk_meta,
    )))
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
