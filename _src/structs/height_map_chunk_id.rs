use std::path::Path;

use allocative::Allocative;
use derive_deref::{Deref, DerefMut};

use super::{Height, MapChunkId, HEIGHT_MAP_CHUNK_SIZE};

#[derive(
    Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Allocative, Deref, DerefMut,
)]
pub struct HeightMapChunkId(Height);

impl HeightMapChunkId {
    pub fn new(height: &Height) -> Self {
        Self(Height::new(
            **height / HEIGHT_MAP_CHUNK_SIZE * HEIGHT_MAP_CHUNK_SIZE,
        ))
    }
}

impl MapChunkId for HeightMapChunkId {
    fn to_string(&self) -> String {
        let start = ***self;
        let end = start + HEIGHT_MAP_CHUNK_SIZE;

        format!("{start}..{end}")
    }

    fn from_path(path: &Path) -> color_eyre::Result<Self> {
        Ok(Self(Height::new(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split("..")
                .next()
                .unwrap()
                .parse::<u32>()?,
        )))
    }

    fn to_usize(self) -> usize {
        **self as usize
    }

    fn from_usize(id: usize) -> Self {
        Self(Height::new(id as u32))
    }

    fn next(&self) -> Option<Self> {
        self.checked_add(HEIGHT_MAP_CHUNK_SIZE)
            .map(|h| Self(Height::new(h)))
    }

    fn previous(&self) -> Option<Self> {
        self.checked_sub(HEIGHT_MAP_CHUNK_SIZE)
            .map(|h| Self(Height::new(h)))
    }
}
