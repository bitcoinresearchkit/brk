use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

use brk_core::{Error, Result};

use crate::{AnyVec, File, HEADER_OFFSET, Header, file::Reader};

use super::{StoredIndex, StoredType};

pub trait GenericStoredVec<I, T>: Send + Sync
where
    Self: AnyVec,
    I: StoredIndex,
    T: StoredType,
{
    const SIZE_OF_T: usize = size_of::<T>();

    #[inline]
    fn unwrap_read(&self, index: I, reader: &Reader<'_>) -> T {
        self.read(index, reader).unwrap().unwrap()
    }
    #[inline]
    fn read(&self, index: I, reader: &Reader<'_>) -> Result<Option<T>> {
        self.read_(index.to_usize()?, reader)
    }
    fn read_(&self, index: usize, reader: &Reader<'_>) -> Result<Option<T>>;

    #[inline]
    fn get_or_read(&self, index: I, reader: &Reader<'_>) -> Result<Option<Cow<T>>> {
        self.get_or_read_(index.to_usize()?, reader)
    }
    #[inline]
    fn get_or_read_(&self, index: usize, reader: &Reader<'_>) -> Result<Option<Cow<T>>> {
        let stored_len = self.stored_len();

        if index >= stored_len {
            let pushed = self.pushed();
            let j = index - stored_len;
            if j >= pushed.len() {
                return Ok(None);
            }
            return Ok(pushed.get(j).map(Cow::Borrowed));
        }

        let updated = self.updated();
        if !updated.is_empty()
            && let Some(updated) = updated.get(&index)
        {
            return Ok(Some(Cow::Borrowed(updated)));
        }

        let holes = self.holes();
        if !holes.is_empty() && holes.contains(&index) {
            return Ok(None);
        }

        Ok(self.read_(index, reader)?.map(Cow::Owned))
    }

    #[inline]
    fn len_(&self) -> usize {
        self.stored_len() + self.pushed_len()
    }

    fn stored_len(&self) -> usize;

    fn pushed(&self) -> &[T];
    #[inline]
    fn pushed_len(&self) -> usize {
        self.pushed().len()
    }
    fn mut_pushed(&mut self) -> &mut Vec<T>;
    #[inline]
    fn push(&mut self, value: T) {
        self.mut_pushed().push(value)
    }

    #[inline]
    fn update_or_push(&mut self, index: I, value: T) -> Result<()> {
        let len = self.len();
        match len.cmp(&index.to_usize()?) {
            Ordering::Less => {
                dbg!(index, value, len, self.header());
                Err(Error::IndexTooHigh)
            }
            Ordering::Equal => {
                self.push(value);
                Ok(())
            }
            Ordering::Greater => self.update(index, value),
        }
    }

    fn get_first_empty_index(&self) -> I {
        self.holes()
            .first()
            .cloned()
            .unwrap_or_else(|| self.len_())
            .into()
    }

    #[inline]
    fn fill_first_hole_or_push(&mut self, value: T) -> Result<I> {
        Ok(
            if let Some(hole) = self.mut_holes().pop_first().map(I::from) {
                self.update(hole, value)?;
                hole
            } else {
                self.push(value);
                I::from(self.len() - 1)
            },
        )
    }

    fn holes(&self) -> &BTreeSet<usize>;
    fn mut_holes(&mut self) -> &mut BTreeSet<usize>;
    fn take(&mut self, index: I, reader: &Reader<'_>) -> Result<Option<T>> {
        let opt = self.get_or_read(index, reader)?.map(|v| v.into_owned());
        if opt.is_some() {
            self.unchecked_delete(index);
        }
        Ok(opt)
    }
    #[inline]
    fn delete(&mut self, index: I) {
        if index.unwrap_to_usize() < self.len() {
            self.unchecked_delete(index);
        }
    }
    #[inline]
    #[doc(hidden)]
    fn unchecked_delete(&mut self, index: I) {
        let uindex = index.unwrap_to_usize();
        let updated = self.mut_updated();
        if !updated.is_empty() {
            updated.remove(&uindex);
        }
        self.mut_holes().insert(uindex);
    }

    fn updated(&self) -> &BTreeMap<usize, T>;
    fn mut_updated(&mut self) -> &mut BTreeMap<usize, T>;
    #[inline]
    fn update(&mut self, index: I, value: T) -> Result<()> {
        let uindex = index.unwrap_to_usize();
        let stored_len = self.stored_len();

        if uindex >= stored_len {
            if let Some(prev) = self.mut_pushed().get_mut(uindex - stored_len) {
                *prev = value;
                return Ok(());
            } else {
                return Err(Error::IndexTooHigh);
            }
        }

        let holes = self.mut_holes();
        if !holes.is_empty() {
            holes.remove(&index.unwrap_to_usize());
        }

        self.mut_updated().insert(index.unwrap_to_usize(), value);

        Ok(())
    }

    fn header(&self) -> &Header;
    fn mut_header(&mut self) -> &mut Header;

    fn reset(&mut self) -> Result<()>;

    #[inline]
    fn reset_(&mut self) -> Result<()> {
        self.file().remove_region(self.holes_region_name().into())?;
        self.file()
            .truncate_region(self.region_index().into(), HEADER_OFFSET as u64)
    }

    #[inline]
    fn is_pushed_empty(&self) -> bool {
        self.pushed_len() == 0
    }

    #[inline]
    fn has(&self, index: I) -> Result<bool> {
        Ok(self.has_(index.to_usize()?))
    }
    #[inline]
    fn has_(&self, index: usize) -> bool {
        index < self.len_()
    }

    fn file(&self) -> &File;

    fn region_index(&self) -> usize;

    /// Be careful with deadlocks
    ///
    /// You'll want to drop the reader before mutable ops
    fn create_reader(&self) -> Reader<'_> {
        self.create_static_reader()
    }

    /// Be careful with deadlocks
    ///
    /// You'll want to drop the reader before mutable ops
    fn create_static_reader(&self) -> Reader<'static> {
        unsafe {
            std::mem::transmute(
                self.file()
                    .create_region_reader(self.region_index().into())
                    .unwrap(),
            )
        }
    }

    fn flush(&mut self) -> Result<()>;

    fn truncate_if_needed(&mut self, index: I) -> Result<()>;

    fn index_to_name(&self) -> String {
        format!("{}_to_{}", I::to_string(), self.name())
    }

    fn vec_region_name(&self) -> String {
        Self::vec_region_name_(self.name())
    }
    fn vec_region_name_(name: &str) -> String {
        format!("{name}_{}", I::to_string())
    }

    fn holes_region_name(&self) -> String {
        Self::holes_region_name_(self.name())
    }
    fn holes_region_name_(name: &str) -> String {
        format!("{}_holes", Self::vec_region_name_(name))
    }
}
