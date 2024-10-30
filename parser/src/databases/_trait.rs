use std::{fs, io};

use crate::{
    structs::{Date, Height},
    utils::log,
};

use super::{databases_folder_path, AnyDatabase};

pub trait AnyDatabaseGroup
where
    Self: Sized,
{
    fn import() -> Self;

    fn folder<'a>() -> &'a str;

    fn drain_to_vec(&mut self) -> Vec<Box<dyn AnyDatabase + Send>>;
    fn open_all(&mut self);

    fn export_metadata(&mut self, height: Height, date: Date) -> color_eyre::Result<()>;
    // fn export(&mut self, height: Height, date: Date) -> color_eyre::Result<()>;
    // fn defragment(&mut self);

    fn reset(&mut self) -> color_eyre::Result<(), io::Error> {
        log(&format!("Reset {}", Self::folder()));

        self.reset_metadata();

        fs::remove_dir_all(Self::full_path())?;

        Ok(())
    }

    fn full_path() -> String {
        databases_folder_path(Self::folder())
    }

    fn reset_metadata(&mut self);
}
