use std::{
    fmt::Debug,
    fs, io,
    path::{Path, PathBuf},
};

use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

use crate::{io::Serialization, structs::Config};

// https://github.com/djkoloski/rust_serialization_benchmark
pub trait AnyState
where
    Self: Debug + Encode + Decode + Serialize + DeserializeOwned,
{
    fn name<'a>() -> &'a str;

    fn path(config: &Config) -> PathBuf {
        config.path_states().join(Self::name())
    }

    fn reset(&mut self, config: &Config) -> color_eyre::Result<(), io::Error> {
        self.clear();
        fs::remove_file(Self::path(config))
    }

    fn import(config: &Config) -> color_eyre::Result<Self> {
        let path = Self::path(config);
        fs::create_dir_all(&path)?;
        Serialization::Binary.import(&path)
    }

    fn export(&self, config: &Config) -> color_eyre::Result<()> {
        Serialization::Binary.export(Path::new(&Self::path(config)), self)
    }

    fn clear(&mut self);
}
