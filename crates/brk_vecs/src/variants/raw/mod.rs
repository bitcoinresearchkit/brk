use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    marker::PhantomData,
    mem,
    sync::Arc,
};

use parking_lot::RwLock;
use zerocopy::{FromBytes, IntoBytes};

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyStoredVec, AnyVec, BaseVecIterator, BoxedVecIterator,
    CollectableVec, Error, File, GenericStoredVec, Reader, Result, StoredIndex, StoredRaw, Version,
    file::{Region, RegionReader},
};

use super::Format;

mod header;

pub use header::*;

const VERSION: Version = Version::ONE;

#[derive(Debug)]
pub struct RawVec<I, T> {
    file: Arc<File>,
    region: Arc<RwLock<Region>>,
    region_index: usize,

    header: Header,
    name: &'static str,
    pushed: Vec<T>,
    has_stored_holes: bool,
    holes: BTreeSet<usize>,
    updated: BTreeMap<usize, T>,
    phantom: PhantomData<I>,
}

impl<I, T> RawVec<I, T>
where
    I: StoredIndex,
    T: StoredRaw,
{
    /// Same as import but will reset the vec under certain errors, so be careful !
    pub fn forced_import(file: &Arc<File>, name: &str, mut version: Version) -> Result<Self> {
        version = version + VERSION;
        let res = Self::import(file, name, version);
        match res {
            Err(Error::DifferentCompressionMode)
            | Err(Error::WrongEndian)
            | Err(Error::WrongLength)
            | Err(Error::DifferentVersion { .. }) => {
                let _ = file.remove_region(Self::vec_region_name_(name).into());
                let _ = file.remove_region(Self::holes_region_name_(name).into());
                Self::import(file, name, version)
            }
            _ => res,
        }
    }

    pub fn import(file: &Arc<File>, name: &str, version: Version) -> Result<Self> {
        Self::any_import(file, name, version, Format::Raw)
    }

    pub fn any_import(
        file: &Arc<File>,
        name: &str,
        version: Version,
        format: Format,
    ) -> Result<Self> {
        let (region_index, region) = file.create_region_if_needed(&Self::vec_region_name_(name))?;

        let region_len = region.read().len() as usize;
        if region_len > 0
            && (region_len < HEADER_OFFSET
                || (format.is_raw() && (region_len - HEADER_OFFSET) % Self::SIZE_OF_T != 0))
        {
            return Err(Error::Str("Region was saved incorrectly"));
        }

        let header = if region_len == 0 {
            Header::create_and_write(file, region_index, version, format)?
        } else {
            Header::import_and_verify(file, region_index, region.read().len(), version, format)?
        };

        let holes = if let Ok(holes) = file.get_region(Self::holes_region_name_(name).into()) {
            Some(
                holes
                    .create_reader(file)
                    .read_all()
                    .chunks(size_of::<usize>())
                    .map(|b| -> Result<usize> { usize::read_from_bytes(b).map_err(|e| e.into()) })
                    .collect::<Result<BTreeSet<usize>>>()?,
            )
        } else {
            None
        };

        Ok(Self {
            file: file.clone(),
            region: region.clone(),
            region_index,
            header,
            name: Box::leak(Box::new(name.to_string())),
            pushed: vec![],
            has_stored_holes: holes.is_some(),
            holes: holes.unwrap_or_default(),
            updated: BTreeMap::new(),
            phantom: PhantomData,
        })
    }

    #[inline]
    pub fn iter(&self) -> RawVecIterator<'_, I, T> {
        self.into_iter()
    }

    #[inline]
    pub fn iter_at(&self, i: I) -> RawVecIterator<'_, I, T> {
        self.iter_at_(i.unwrap_to_usize())
    }

    #[inline]
    pub fn iter_at_(&self, i: usize) -> RawVecIterator<'_, I, T> {
        let mut iter = self.into_iter();
        iter.set_(i);
        iter
    }

    pub fn write_header_if_needed(&mut self) -> Result<()> {
        if self.header.modified() {
            self.header.write(&self.file, self.region_index)?;
        }
        Ok(())
    }
}

impl<I, T> Clone for RawVec<I, T> {
    fn clone(&self) -> Self {
        Self {
            file: self.file.clone(),
            region: self.region.clone(),
            region_index: self.region_index,
            header: self.header.clone(),
            name: self.name,
            pushed: vec![],
            updated: BTreeMap::new(),
            has_stored_holes: false,
            holes: BTreeSet::new(),
            phantom: PhantomData,
        }
    }
}

impl<I, T> AnyVec for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredRaw,
{
    #[inline]
    fn version(&self) -> Version {
        self.header.vec_version()
    }

    #[inline]
    fn name(&self) -> &str {
        self.name
    }

    #[inline]
    fn len(&self) -> usize {
        self.len_()
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

impl<I, T> AnyStoredVec for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredRaw,
{
    fn header(&self) -> &Header {
        &self.header
    }

    fn mut_header(&mut self) -> &mut Header {
        &mut self.header
    }

    #[inline]
    fn stored_len(&self) -> usize {
        (self.region.read().len() as usize - HEADER_OFFSET) / Self::SIZE_OF_T
    }

    fn flush(&mut self) -> Result<()> {
        self.write_header_if_needed()?;

        let has_new_data = !self.is_pushed_empty();
        let has_updated_data = !self.updated.is_empty();
        let has_holes = !self.holes.is_empty();
        let had_holes = self.has_stored_holes && !has_holes;

        if !has_new_data && !has_updated_data && !has_holes && !had_holes {
            return Ok(());
        }

        if has_new_data || has_updated_data {
            let file = &self.file;

            if has_new_data {
                file.write_all_to_region(
                    self.region_index.into(),
                    mem::take(&mut self.pushed).as_bytes(),
                )?;
            }

            if has_updated_data {
                mem::take(&mut self.updated)
                    .into_iter()
                    .try_for_each(|(i, v)| -> Result<()> {
                        let bytes = v.as_bytes();
                        let at = ((i * Self::SIZE_OF_T) + HEADER_OFFSET) as u64;
                        file.write_all_to_region_at(self.region_index.into(), bytes, at)?;
                        Ok(())
                    })?;
            }
        }

        if has_holes || had_holes {
            if has_holes {
                self.has_stored_holes = true;
                let (holes_index, _) = self
                    .file
                    .create_region_if_needed(&self.holes_region_name())?;
                let bytes = self
                    .holes
                    .iter()
                    .flat_map(|i| i.to_ne_bytes())
                    .collect::<Vec<_>>();
                self.file
                    .truncate_write_all_to_region(holes_index.into(), 0, &bytes)?;
            } else if had_holes {
                self.has_stored_holes = false;
                let _ = self.file.remove_region(self.holes_region_name().into());
            }
        }

        Ok(())
    }

    fn file(&self) -> &File {
        &self.file
    }

    fn region_index(&self) -> usize {
        self.region_index
    }

    fn region(&self) -> &RwLock<Region> {
        &self.region
    }
}

impl<I, T> GenericStoredVec<I, T> for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredRaw,
{
    #[inline]
    fn read_(&self, index: usize, reader: &Reader) -> Result<T> {
        T::read_from_prefix(reader.prefixed((index * Self::SIZE_OF_T + HEADER_OFFSET) as u64))
            .map(|(v, _)| v)
            .map_err(Error::from)
    }

    #[inline]
    fn pushed(&self) -> &[T] {
        self.pushed.as_slice()
    }
    #[inline]
    fn mut_pushed(&mut self) -> &mut Vec<T> {
        &mut self.pushed
    }

    #[inline]
    fn holes(&self) -> &BTreeSet<usize> {
        &self.holes
    }
    #[inline]
    fn mut_holes(&mut self) -> &mut BTreeSet<usize> {
        &mut self.holes
    }

    #[inline]
    fn updated(&self) -> &BTreeMap<usize, T> {
        &self.updated
    }
    #[inline]
    fn mut_updated(&mut self) -> &mut BTreeMap<usize, T> {
        &mut self.updated
    }

    fn truncate_if_needed(&mut self, index: I) -> Result<()> {
        let index = index.to_usize()?;

        if index >= self.stored_len() {
            return Ok(());
        }

        if index == 0 {
            self.reset()?;
            return Ok(());
        }

        let from = index * Self::SIZE_OF_T + HEADER_OFFSET;

        self.file
            .truncate_region(self.region_index.into(), from as u64)
    }

    fn reset(&mut self) -> Result<()> {
        self.reset_()
    }
}

#[derive(Debug)]
pub struct RawVecIterator<'a, I, T> {
    vec: &'a RawVec<I, T>,
    reader: Reader<'a>,
    index: usize,
}

impl<I, T> BaseVecIterator for RawVecIterator<'_, I, T>
where
    I: StoredIndex,
    T: StoredRaw,
{
    #[inline]
    fn mut_index(&mut self) -> &mut usize {
        &mut self.index
    }

    #[inline]
    fn len(&self) -> usize {
        self.vec.len()
    }

    #[inline]
    fn name(&self) -> &str {
        self.vec.name()
    }
}

impl<'a, I, T> Iterator for RawVecIterator<'a, I, T>
where
    I: StoredIndex,
    T: StoredRaw,
{
    type Item = (I, Cow<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;

        let opt = self
            .vec
            .get_or_read_(index, &self.reader)
            .unwrap()
            .map(|v| (I::from(index), v));

        if opt.is_some() {
            self.index += 1;
        }

        opt
    }
}

impl<'a, I, T> IntoIterator for &'a RawVec<I, T>
where
    I: StoredIndex,
    T: StoredRaw,
{
    type Item = (I, Cow<'a, T>);
    type IntoIter = RawVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        RawVecIterator {
            vec: self,
            reader: self.create_static_reader(),
            index: 0,
        }
    }
}

impl<I, T> AnyIterableVec<I, T> for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredRaw,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        T: 'a,
    {
        Box::new(self.into_iter())
    }
}

impl<I, T> AnyCollectableVec for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredRaw,
{
    fn collect_range_serde_json(
        &self,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
