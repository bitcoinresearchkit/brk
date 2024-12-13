use std::{fs, io, path::Path};

use log::info;

use crate::structs::{Config, Date, Height};

use super::{AnyDatabase, Metadata};

pub trait AnyDatabaseGroup
where
    Self: Sized,
{
    fn init(config: &Config) -> Self {
        let s = Self::import(config);
        s.create_dir_all().unwrap();
        s
    }

    fn import(config: &Config) -> Self;

    fn drain_to_vec(&mut self) -> Vec<Box<dyn AnyDatabase + Send>>;
    fn open_all(&mut self);

    fn metadata(&mut self) -> &mut Metadata;
    fn export_metadata(&mut self, height: Height, date: Date) -> color_eyre::Result<()> {
        self.metadata().export(height, date)
    }

    fn create_dir_all(&self) -> color_eyre::Result<(), std::io::Error> {
        fs::create_dir_all(self.path())
    }

    fn remove_dir_all(&self) -> color_eyre::Result<(), io::Error> {
        fs::remove_dir_all(self.path())
    }

    fn reset(&mut self) -> color_eyre::Result<(), io::Error> {
        info!(
            "Reset {}",
            self.path()
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
        );

        self.reset_metadata();
        self.remove_dir_all()?;
        self.create_dir_all()?;

        Ok(())
    }

    fn reset_metadata(&mut self);

    fn path(&self) -> &Path;
}
