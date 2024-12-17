use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
};

use derive_deref::{Deref, DerefMut};
use itertools::Itertools;

use crate::{
    io::Serialization,
    server::{api::API_URL_PREFIX, Grouped},
    structs::Config,
};

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

const WEBSITE_TYPES_PATH: &str = "../website/scripts/types";

impl Routes {
    pub fn build(paths_to_type: BTreeMap<PathBuf, String>, config: &Config) -> Self {
        let mut routes = Routes::default();

        paths_to_type.into_iter().for_each(|(path, value)| {
            let try_from_path = if path.is_file() {
                path.clone()
            } else {
                fs::read_dir(&path)
                    .unwrap_or_else(|_| {
                        dbg!(&path);
                        panic!();
                    })
                    .map(|e| e.unwrap().path())
                    .find(|e| e.is_file())
                    .unwrap()
            };

            let serialization =
                Serialization::try_from(&try_from_path).unwrap_or(Serialization::Binary);

            let file_path_ser = path.to_str().unwrap().to_owned();
            let split_key = file_path_ser.replace(
                &format!("{}/", config.path_datasets().to_str().unwrap()),
                "",
            );
            let split_key =
                split_key.replace(&format!("{}/", config.path_kibodir().to_str().unwrap()), "");
            let mut split_key = split_key.split('/').collect_vec();
            let last = split_key.pop().unwrap().to_owned();
            let last = last.split('.').next().unwrap();

            // Use case for: "../datasets/last": "Value",
            if split_key.is_empty() {
                split_key.push("last");
            }

            let map_key = split_key.iter().join("_");

            let url_path = split_key.iter().join("-");

            let values_type = value.to_owned();

            match last {
                "date" => {
                    routes.date.insert(
                        map_key,
                        Route {
                            url_path: format!("date-to-{url_path}"),
                            file_path: path,
                            values_type,
                            serialization,
                        },
                    );
                }
                "height" => {
                    routes.height.insert(
                        map_key,
                        Route {
                            url_path: format!("height-to-{url_path}"),
                            file_path: path,
                            values_type,
                            serialization,
                        },
                    );
                }
                "last" => {
                    routes.last.insert(
                        map_key,
                        Route {
                            url_path,
                            file_path: path,
                            values_type,
                            serialization,
                        },
                    );
                }
                _ => {
                    dbg!(&path, value, &last, &split_key);
                    panic!("")
                }
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

        // fs::write(
        //     format!("{WEBSITE_TYPES_PATH}/paths.d.ts"),
        //     format!("// This file is auto generated by the server\n// Manual changes are forbidden\n\n{date_type}\n{height_type}\n{last_type}"),
        // )
        // .unwrap();
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
                        format!("{url}{API_URL_PREFIX}/{}", route.url_path.to_owned()),
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
