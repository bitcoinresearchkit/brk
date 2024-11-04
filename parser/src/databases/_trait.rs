use std::{fs, io, path::PathBuf};

use crate::{
    io::OUTPUTS_FOLDER_PATH,
    structs::{Date, Height},
    utils::log,
};

use super::AnyDatabase;

pub trait AnyDatabaseGroup
where
    Self: Sized,
{
    fn init() -> Self {
        let s = Self::import();
        s.create_dir_all().unwrap();
        s
    }

    fn import() -> Self;

    fn folder<'a>() -> &'a str;

    fn drain_to_vec(&mut self) -> Vec<Box<dyn AnyDatabase + Send>>;
    fn open_all(&mut self);

    fn export_metadata(&mut self, height: Height, date: Date) -> color_eyre::Result<()>;

    fn create_dir_all(&self) -> color_eyre::Result<(), io::Error>;

    fn remove_dir_all(&self) -> color_eyre::Result<(), io::Error> {
        fs::remove_dir_all(Self::root())
    }

    fn reset(&mut self) -> color_eyre::Result<(), io::Error> {
        log(&format!("Reset {}", Self::folder()));

        self.reset_metadata();
        self.remove_dir_all()?;
        self.create_dir_all()?;

        Ok(())
    }

    fn reset_metadata(&mut self);

    fn root() -> PathBuf {
        let folder = Self::folder();
        PathBuf::from(format!("{OUTPUTS_FOLDER_PATH}/databases/{folder}"))
    }
}
