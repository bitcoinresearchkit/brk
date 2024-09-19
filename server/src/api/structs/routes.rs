use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Path, PathBuf},
};

use derive_deref::{Deref, DerefMut};
use itertools::Itertools;
use parser::{Json, Serialization};

use crate::Grouped;

use super::Paths;

#[derive(Clone, Debug)]
pub struct Route {
    pub url_path: String,
    pub file_path: PathBuf,
    pub values_type: String,
    pub serialization: Serialization,
}

#[derive(Clone, Default, Deref, DerefMut)]
pub struct Routes(pub Grouped<HashMap<String, Route>>);

const INPUTS_PATH: &str = "./in";
const WEBSITE_TYPES_PATH: &str = "../website/types";

impl Routes {
    pub fn build() -> Self {
        let path_to_type: BTreeMap<String, String> =
            Json::import(Path::new(&format!("{INPUTS_PATH}/disk_path_to_type.json"))).unwrap();

        let mut routes = Routes::default();

        path_to_type.into_iter().for_each(|(key, value)| {
            let mut split_key = key.split('/').collect_vec();
            let last = split_key.pop().unwrap().to_owned();

            let mut skip = 2;

            let mut serialization = Serialization::Binary;

            if *split_key.get(1).unwrap() == "price" {
                skip = 1;
                serialization = Serialization::Json;
            }

            let split_key = split_key.iter().skip(skip).collect_vec();

            let map_key = split_key.iter().join("_");

            let url_path = split_key.iter().join("-");

            let file_path = PathBuf::from(key.to_owned());
            let values_type = value.to_owned();

            if last == "date" {
                routes.date.insert(
                    map_key,
                    Route {
                        url_path: format!("date-to-{url_path}"),
                        file_path,
                        values_type,
                        serialization,
                    },
                );
            } else if last == "height" {
                routes.height.insert(
                    map_key,
                    Route {
                        url_path: format!("height-to-{url_path}"),
                        file_path,
                        values_type,
                        serialization,
                    },
                );
            } else if last == "last" {
                routes.last.insert(
                    map_key,
                    Route {
                        url_path,
                        file_path,
                        values_type,
                        serialization,
                    },
                );
            } else {
                dbg!(&key, value, &last);
                panic!("")
            }
        });

        routes
    }

    pub fn generate_dts_file(&self) {
        let map_to_type = |name: &str, map: &HashMap<String, Route>| -> String {
            let paths = map
                .values()
                .map(|route| format!("\"{}\"", route.url_path))
                .join(" | ");

            format!("export type {}Path = {};\n", name, paths)
        };

        let date_type = map_to_type("Date", &self.date);

        let height_type = map_to_type("Height", &self.height);

        let last_type = map_to_type("Last", &self.last);

        fs::write(
            format!("{WEBSITE_TYPES_PATH}/paths.d.ts"),
            format!("// This file is auto generated by the server\n// Manual changes are forbidden\n\n{date_type}\n{height_type}\n{last_type}"),
        )
        .unwrap();
    }

    pub fn to_full_paths(&self, host: String) -> Paths {
        let url = {
            let scheme = if host.contains("0.0.0.0") || host.contains("localhost") {
                "http"
            } else {
                "https"
            };

            format!("{scheme}://{host}")
        };

        let transform = |map: &HashMap<String, Route>| -> BTreeMap<String, String> {
            map.iter()
                .map(|(key, route)| {
                    (
                        key.to_owned(),
                        format!("{url}/api/{}", route.url_path.to_owned()),
                    )
                })
                .collect()
        };

        let date_paths = transform(&self.date);
        let height_paths = transform(&self.height);
        let last_paths = transform(&self.last);

        Paths(Grouped {
            date: date_paths,
            height: height_paths,
            last: last_paths,
        })
    }
}
