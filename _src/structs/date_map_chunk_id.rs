use std::path::Path;

use allocative::Allocative;
use chrono::Datelike;

use super::{Date, MapChunkId};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Allocative)]
pub struct DateMapChunkId(i32);

impl DateMapChunkId {
    pub fn new(date: &Date) -> Self {
        Self(date.year())
    }
}

impl MapChunkId for DateMapChunkId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn from_path(path: &Path) -> color_eyre::Result<Self> {
        Ok(Self(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split(".")
                .next()
                .unwrap()
                .parse::<i32>()?,
        ))
    }

    fn to_usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(id: usize) -> Self {
        Self(id as i32)
    }

    fn next(&self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }

    fn previous(&self) -> Option<Self> {
        self.0.checked_sub(1).map(Self)
    }
}
