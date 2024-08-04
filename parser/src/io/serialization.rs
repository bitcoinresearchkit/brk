use std::{fmt::Debug, fs, path::Path};

use allocative::Allocative;
use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

use crate::io::{Binary, Json};

use super::{BIN_EXTENSION, COMPRESSED_BIN_EXTENSION, HAR_EXTENSION, JSON_EXTENSION};

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Copy, Default, Allocative)]
pub enum Serialization {
    #[default]
    Binary,
    Json,
}

impl Serialization {
    pub fn is_serializable(&self, path: &Path) -> bool {
        let path = path.to_str().unwrap();
        match self {
            Self::Binary => {
                path.ends_with(BIN_EXTENSION) || path.ends_with(COMPRESSED_BIN_EXTENSION)
            }
            Self::Json => path.ends_with(JSON_EXTENSION) || path.ends_with(HAR_EXTENSION),
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let path = path.to_str().unwrap();
        if path.ends_with(BIN_EXTENSION) || path.ends_with(COMPRESSED_BIN_EXTENSION) {
            Self::Binary
        } else if path.ends_with(JSON_EXTENSION) || path.ends_with(HAR_EXTENSION) {
            Self::Json
        } else {
            panic!("Extension \"{path}\" isn't supported")
        }
    }

    pub fn import<T>(&self, path: &Path) -> color_eyre::Result<T>
    where
        T: Debug + DeserializeOwned + Decode,
    {
        match self {
            Serialization::Binary => {
                if self.is_serializable(path) {
                    Binary::import(path)
                } else {
                    let path = path.to_str().unwrap();
                    let bin_path_str = format!("{path}.{BIN_EXTENSION}");
                    let bin_path = Path::new(&bin_path_str);

                    if bin_path.exists() {
                        return Binary::import(bin_path);
                    }

                    let compressed_bin_path_str = format!("{path}.{COMPRESSED_BIN_EXTENSION}");
                    let compressed_bin_path = Path::new(&compressed_bin_path_str);

                    if compressed_bin_path.exists() {
                        return Binary::import(compressed_bin_path);
                    }

                    panic!("Wrong path")
                }
            }
            Serialization::Json => {
                if self.is_serializable(path) {
                    Json::import(path)
                } else {
                    let path = path.to_str().unwrap();
                    let json_path_str = format!("{path}.{JSON_EXTENSION}");
                    let json_path = Path::new(&json_path_str);

                    if json_path.exists() {
                        return Json::import(json_path);
                    }

                    panic!("Wrong path")
                }
            }
        }
    }

    pub fn export<T>(&self, path: &Path, value: &T) -> color_eyre::Result<()>
    where
        T: Debug + Serialize + Encode,
    {
        match self {
            Serialization::Binary => {
                if self.is_serializable(path) {
                    Binary::export(path, value)
                } else {
                    let path = path.to_str().unwrap();

                    let res = Binary::export(
                        Path::new(&format!("{}.{COMPRESSED_BIN_EXTENSION}", path,)),
                        value,
                    );

                    if res.is_ok() {
                        let _ = fs::remove_file(Path::new(&format!("{}.{BIN_EXTENSION}", path)));
                    }

                    res
                }
            }
            Serialization::Json => {
                if self.is_serializable(path) {
                    Json::export(path, value)
                } else {
                    Json::export(
                        Path::new(&format!("{}.{JSON_EXTENSION}", path.to_str().unwrap())),
                        value,
                    )
                }
            }
        }
    }
}
