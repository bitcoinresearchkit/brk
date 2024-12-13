use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

pub struct Json;

pub const JSON_EXTENSION: &str = "json";
pub const HAR_EXTENSION: &str = "har";

impl Json {
    pub fn import<T>(path: &Path) -> color_eyre::Result<T>
    where
        T: DeserializeOwned,
    {
        if !Self::has_correct_extension(path) {
            panic!("Wrong extension");
        }

        let file = File::open(path)?;

        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn export<T>(path: &Path, value: &T) -> color_eyre::Result<()>
    where
        T: Serialize,
    {
        if !Self::has_correct_extension(path) {
            dbg!(path);
            panic!("Wrong extension");
        }

        let file = File::create(path).unwrap_or_else(|_| {
            dbg!(&path);
            panic!("No such file or directory")
        });

        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, value)?;

        Ok(())
    }

    #[inline(always)]
    pub fn has_correct_extension(path: &Path) -> bool {
        let path = path.to_str().unwrap();
        path.ends_with(JSON_EXTENSION) || path.ends_with(HAR_EXTENSION)
    }
}
