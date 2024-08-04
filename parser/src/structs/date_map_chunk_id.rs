use std::path::Path;

use allocative::Allocative;
use chrono::Datelike;

use crate::Date;

use super::MapChunkId;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Allocative)]
pub struct DateMapChunkId(i32);

impl DateMapChunkId {
    pub fn new(date: &Date) -> Self {
        Self(date.year())
    }
}

impl MapChunkId for DateMapChunkId {
    fn to_name(&self) -> String {
        self.0.to_string()
    }

    fn from_path(path: &Path) -> Self {
        Self(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split(".")
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        )
    }

    fn to_usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(id: usize) -> Self {
        Self(id as i32)
    }
}
