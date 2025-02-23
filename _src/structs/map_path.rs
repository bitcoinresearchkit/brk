use std::path::PathBuf;

use allocative::Allocative;
use derive_deref::{Deref, DerefMut};

#[derive(Debug, Clone, Deref, DerefMut, Allocative)]
pub struct MapPath(PathBuf);

impl MapPath {
    pub fn join(&self, path: &str) -> Self {
        let path = path.replace(['-', '_', ' '], "/");
        Self(self.0.join(path))
    }

    pub fn unwrap(&self) -> &PathBuf {
        &self.0
    }
}

impl From<PathBuf> for MapPath {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}
