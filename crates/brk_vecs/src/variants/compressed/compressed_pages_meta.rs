use std::{
    fs::{self, OpenOptions},
    io::{self, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use brk_core::Result;
use rayon::prelude::*;
use zerocopy::{IntoBytes, TryFromBytes};

use super::{CompressedPageMetadata, UnsafeSlice};

#[derive(Debug, Clone)]
pub struct CompressedPagesMetadata {
    vec: Vec<CompressedPageMetadata>,
    change_at: Option<usize>,
    path: PathBuf,
}

impl CompressedPagesMetadata {
    const PAGE_SIZE: usize = size_of::<CompressedPageMetadata>();

    pub fn read(path: &Path) -> Result<CompressedPagesMetadata> {
        let this = Self {
            vec: fs::read(path)
                .unwrap_or_default()
                .chunks(Self::PAGE_SIZE)
                .map(|bytes| {
                    if bytes.len() != Self::PAGE_SIZE {
                        panic!()
                    }
                    CompressedPageMetadata::try_read_from_bytes(bytes).unwrap()
                })
                .collect::<Vec<_>>(),
            path: path.to_owned(),
            change_at: None,
        };

        Ok(this)
    }

    pub fn write(&mut self) -> io::Result<()> {
        if self.change_at.is_none() {
            return Ok(());
        }

        let change_at = self.change_at.take().unwrap();

        let len = (self.vec.len() - change_at) * Self::PAGE_SIZE;

        let mut bytes: Vec<u8> = vec![0; len];

        let unsafe_bytes = UnsafeSlice::new(&mut bytes);

        self.vec[change_at..]
            .par_iter()
            .enumerate()
            .for_each(|(i, v)| unsafe_bytes.copy_slice(i * Self::PAGE_SIZE, v.as_bytes()));

        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(&self.path)?;

        file.set_len((change_at * Self::PAGE_SIZE) as u64)?;
        file.seek(SeekFrom::End(0))?;

        file.write_all(&bytes)?;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, page_index: usize) -> Option<&CompressedPageMetadata> {
        self.vec.get(page_index)
    }

    pub fn last(&self) -> Option<&CompressedPageMetadata> {
        self.vec.last()
    }

    pub fn pop(&mut self) -> Option<CompressedPageMetadata> {
        self.vec.pop()
    }

    pub fn push(&mut self, page_index: usize, page: CompressedPageMetadata) {
        if page_index != self.vec.len() {
            panic!();
        }

        self.set_changed_at(page_index);

        self.vec.push(page);
    }

    fn set_changed_at(&mut self, page_index: usize) {
        if self.change_at.is_none_or(|pi| pi > page_index) {
            self.change_at.replace(page_index);
        }
    }

    pub fn truncate(&mut self, page_index: usize) -> Option<CompressedPageMetadata> {
        let page = self.get(page_index).cloned();
        self.vec.truncate(page_index);
        self.set_changed_at(page_index);
        page
    }
}
