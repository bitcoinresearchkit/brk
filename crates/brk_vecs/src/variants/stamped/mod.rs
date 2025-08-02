use std::{borrow::Cow, cmp::Ordering, fmt::Debug, sync::Arc};

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BoxedVecIterator, CollectableVec, Error, File,
    Format, GenericStoredVec, Header, Result, StoredCompressed, StoredIndex, StoredVec, Version,
    file::Reader,
};

use super::StoredVecIterator;

mod stamp;

pub use stamp::*;

#[derive(Debug, Clone)]
pub struct StampedVec<I, T>(StoredVec<I, T>);

impl<I, T> StampedVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    pub fn forced_import(
        file: &Arc<File>,
        name: &str,
        version: Version,
        format: Format,
    ) -> Result<Self> {
        Ok(Self(StoredVec::forced_import(file, name, version, format)?))
    }

    #[inline]
    pub fn unwrap_read(&self, index: I, reader: &Reader) -> T {
        self.0.unwrap_read(index, reader)
    }

    #[inline]
    pub fn get_or_read<'a, 'b>(&'a self, index: I, reader: &'b Reader) -> Result<Option<Cow<'b, T>>>
    where
        'a: 'b,
    {
        self.0.get_or_read(index, reader)
    }

    #[inline]
    pub fn update_or_push(&mut self, index: I, value: T) -> Result<()> {
        self.0.update_or_push(index, value)
    }

    #[inline]
    pub fn checked_push(&mut self, index: I, value: T) -> Result<()> {
        let len = self.0.len();
        match len.cmp(&index.to_usize()?) {
            Ordering::Greater => {
                dbg!(index, value, len, self.0.header());
                Err(Error::IndexTooLow)
            }
            Ordering::Equal => {
                self.0.push(value);
                Ok(())
            }
            Ordering::Less => {
                dbg!(index, value, len, self.0.header());
                Err(Error::IndexTooHigh)
            }
        }
    }

    #[inline]
    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        let len = self.0.len();
        match len.cmp(&index.to_usize()?) {
            Ordering::Greater => {
                // dbg!(len, index, &self.pathbuf);
                // panic!();
                Ok(())
            }
            Ordering::Equal => {
                self.0.push(value);
                Ok(())
            }
            Ordering::Less => {
                dbg!(index, value, len, self.0.header());
                Err(Error::IndexTooHigh)
            }
        }
    }

    #[inline]
    pub fn fill_first_hole_or_push(&mut self, value: T) -> Result<I> {
        self.0.fill_first_hole_or_push(value)
    }

    pub fn update(&mut self, index: I, value: T) -> Result<()> {
        self.0.update(index, value)
    }

    pub fn take(&mut self, index: I, reader: &Reader) -> Result<Option<T>> {
        self.0.take(index, reader)
    }

    pub fn delete(&mut self, index: I) {
        self.0.delete(index)
    }

    fn update_stamp(&mut self, stamp: Stamp) {
        self.0.mut_header().update_stamp(stamp);
    }

    pub fn reset(&mut self) -> Result<()> {
        self.update_stamp(Stamp::default());
        self.0.reset()
    }

    pub fn truncate_if_needed(&mut self, index: I, stamp: Stamp) -> Result<()> {
        self.update_stamp(stamp);
        self.0.truncate_if_needed(index)?;
        Ok(())
    }

    pub fn flush(&mut self, stamp: Stamp) -> Result<()> {
        self.update_stamp(stamp);
        self.0.flush()
    }

    pub fn header(&self) -> &Header {
        self.0.header()
    }

    #[inline]
    pub fn hasnt(&self, index: I) -> Result<bool> {
        self.0.has(index).map(|b| !b)
    }

    pub fn create_reader(&self) -> Reader {
        self.0.create_reader()
    }

    pub fn create_static_reader(&self) -> Reader<'static> {
        self.0.create_static_reader()
    }
}

impl<I, T> AnyVec for StampedVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    #[inline]
    fn version(&self) -> Version {
        self.0.version()
    }

    #[inline]
    fn name(&self) -> &str {
        self.0.name()
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }
}

pub trait AnyStampedVec: AnyVec {
    fn stamp(&self) -> Stamp;
    fn flush(&mut self, stamp: Stamp) -> Result<()>;
}

impl<I, T> AnyStampedVec for StampedVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    fn stamp(&self) -> Stamp {
        self.0.header().stamp()
    }

    fn flush(&mut self, stamp: Stamp) -> Result<()> {
        self.flush(stamp)
    }
}

impl<'a, I, T> IntoIterator for &'a StampedVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    type Item = (I, Cow<'a, T>);
    type IntoIter = StoredVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<I, T> AnyIterableVec<I, T> for StampedVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        T: 'a,
    {
        Box::new(self.into_iter())
    }
}

impl<I, T> AnyCollectableVec for StampedVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    fn collect_range_serde_json(
        &self,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
