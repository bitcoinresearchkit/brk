use std::sync::Arc;

use parking_lot::RwLock;
use zerocopy::{FromBytes, IntoBytes};

use crate::{
    File, Result,
    file::{Region, RegionReader},
};

use super::Page;

#[derive(Debug, Clone)]
pub struct Pages {
    _region: Arc<RwLock<Region>>,
    region_index: usize,

    vec: Vec<Page>,
    change_at: Option<usize>,
}

impl Pages {
    const SIZE_OF_PAGE: usize = size_of::<Page>();

    pub fn import(file: &File, name: &str) -> Result<Self> {
        let (region_index, _region) = file.create_region_if_needed(name)?;

        let vec = _region
            .read()
            .create_reader(file)
            .read_all()
            .chunks(Self::SIZE_OF_PAGE)
            .map(|b| Page::read_from_bytes(b).map_err(|e| e.into()))
            .collect::<Result<_>>()?;

        Ok(Self {
            _region,
            region_index,
            vec,
            change_at: None,
        })
    }

    pub fn flush(&mut self, file: &File) -> Result<()> {
        if self.change_at.is_none() {
            return Ok(());
        }

        let change_at = self.change_at.take().unwrap();
        let at = (change_at * Self::SIZE_OF_PAGE) as u64;

        file.truncate_write_all_to_region(
            self.region_index.into(),
            at,
            self.vec[change_at..].as_bytes(),
        )?;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, page_index: usize) -> Option<&Page> {
        self.vec.get(page_index)
    }

    pub fn last(&self) -> Option<&Page> {
        self.vec.last()
    }

    pub fn pop(&mut self) -> Option<Page> {
        let popped = self.vec.pop();
        if popped.is_some() {
            self.set_changed_at(self.vec.len());
        }
        popped
    }

    pub fn checked_push(&mut self, page_index: usize, page: Page) {
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

    pub fn reset(&mut self, file: &File) -> Result<()> {
        self.truncate(0);
        self.flush(file)
    }

    pub fn truncate(&mut self, page_index: usize) -> Option<Page> {
        let page = self.get(page_index).cloned();
        self.vec.truncate(page_index);
        self.set_changed_at(page_index);
        page
    }

    pub fn stored_len(&self, per_page: usize) -> usize {
        if let Some(last) = self.last() {
            (self.len() - 1) * per_page + last.values as usize
        } else {
            0
        }
    }
}
