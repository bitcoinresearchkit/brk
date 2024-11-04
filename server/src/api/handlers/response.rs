use std::fmt::Debug;

use axum::response::{IntoResponse, Json, Response};
use bincode::Decode;
use parser::{Date, MapValue, SerializedBTreeMap, SerializedVec};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{
    api::structs::{Chunk, Kind, Route},
    header_map::HeaderMapUtils,
};

use super::extension::Extension;

#[derive(Serialize)]
struct WrappedDataset<'a, T>
where
    T: Serialize,
{
    source: &'a str,
    chunk: Chunk,
    dataset: T,
}

#[derive(Serialize)]
struct WrappedValue<T>
where
    T: Serialize,
{
    value: T,
}

pub fn typed_value_to_response<T>(
    kind: Kind,
    route: &Route,
    chunk: Option<Chunk>,
    id: String,
    extension: Option<Extension>,
) -> color_eyre::Result<Response>
where
    T: Serialize + Debug + DeserializeOwned + Decode + MapValue,
{
    Ok(match kind {
        Kind::Date => {
            let dataset = if chunk.is_some() {
                route
                    .serialization
                    .import::<SerializedBTreeMap<Date, T>>(&route.file_path)?
            } else {
                SerializedBTreeMap::<Date, T>::import_all(&route.file_path, &route.serialization)
            };

            if extension == Some(Extension::CSV) {
                let mut csv = format!("date,{}\n", id);

                dataset.map.iter().for_each(|(k, v)| {
                    csv += &format!("{},{:?}\n", k, v);
                });

                string_to_response(csv, extension)
            } else {
                dataset_to_response(dataset, chunk, extension)
            }
        }
        Kind::Height => {
            let dataset = if chunk.is_some() {
                route
                    .serialization
                    .import::<SerializedVec<T>>(&route.file_path)?
            } else {
                SerializedVec::<T>::import_all(&route.file_path, &route.serialization)
            };

            if extension == Some(Extension::CSV) {
                let mut csv = format!("height,{}\n", id);

                let starting_height = chunk.map_or(0, |chunk| chunk.id);

                dataset.map.iter().enumerate().for_each(|(k, v)| {
                    csv += &format!("{},{:?}\n", starting_height + k, v);
                });

                string_to_response(csv, extension)
            } else {
                dataset_to_response(dataset, chunk, extension)
            }
        }
        Kind::Last => {
            let value = route.serialization.import::<T>(&route.file_path)?;

            if extension == Some(Extension::JSON) {
                value_to_response(WrappedValue { value }, extension)
            } else {
                value_to_response(value, extension)
            }
        }
    })
}

pub fn string_to_response(s: String, extension: Option<Extension>) -> Response {
    update_reponse_headers(s.into_response(), 5, extension)
}

pub fn value_to_response<T>(value: T, extension: Option<Extension>) -> Response
where
    T: Serialize,
{
    update_reponse_headers(generic_to_reponse(value, None), 1, extension)
}

fn dataset_to_response<T>(
    dataset: T,
    chunk: Option<Chunk>,
    extension: Option<Extension>,
) -> Response
where
    T: Serialize,
{
    update_reponse_headers(generic_to_reponse(dataset, chunk), 5, extension)
}

pub fn generic_to_reponse<T>(generic: T, chunk: Option<Chunk>) -> Response
where
    T: Serialize,
{
    if let Some(chunk) = chunk {
        Json(WrappedDataset {
            source: "https://kibo.money",
            chunk,
            dataset: generic,
        })
        .into_response()
    } else {
        Json(generic).into_response()
    }
}

pub fn update_reponse_headers(
    mut response: Response,
    cache_time: u64,
    extension: Option<Extension>,
) -> Response {
    let headers = response.headers_mut();

    let max_age = cache_time;
    let stale_while_revalidate = max_age;

    headers.insert_cors();
    headers.insert_cache_control_revalidate(max_age, stale_while_revalidate);

    match extension {
        Some(extension) => {
            headers.insert_content_disposition_attachment();

            match extension {
                Extension::CSV => headers.insert_content_type_text_csv(),
                Extension::JSON => headers.insert_content_type_application_json(),
            }
        }
        _ => headers.insert_content_type_application_json(),
    }

    response
}
