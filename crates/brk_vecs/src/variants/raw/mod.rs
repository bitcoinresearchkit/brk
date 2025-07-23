use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fs, io,
    marker::PhantomData,
    mem,
    os::unix::fs::FileExt,
    sync::Arc,
};

use brk_core::{Error, Result, Version};
use memmap2::Mmap;
use rayon::prelude::*;
use zerocopy::IntoBytes;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedVecIterator, CollectableVec,
    File, GenericStoredVec, StoredIndex, StoredType, file::Reader,
};

use super::Format;

mod header;
mod unsafe_slice;

pub use header::*;
pub use unsafe_slice::*;

const VERSION: Version = Version::ONE;

#[derive(Debug)]
pub struct RawVec<I, T> {
    region: usize,
    file: Arc<File>,

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
    T: StoredType,
{
    /// Same as import but will reset the folder under certain errors, so be careful !
    pub fn forced_import(file: &Arc<File>, name: &str, mut version: Version) -> Result<Self> {
        version = version + VERSION;
        let res = Self::import(file, name, version);
        match res {
            Err(Error::DifferentCompressionMode)
            | Err(Error::WrongEndian)
            | Err(Error::WrongLength)
            | Err(Error::DifferentVersion { .. }) => {
                fs::remove_file(path)?;
                let holes_path = Self::holes_path_(parent, name);
                if fs::exists(&holes_path)? {
                    fs::remove_file(holes_path)?;
                }
                Self::import(file, name, version)
            }
            _ => res,
        }
    }

    pub fn import(file: &Arc<File>, name: &str, version: Version) -> Result<Self> {
        let region = file.create_region_if_needed(&format!("{name}_{}", I::to_string()))?;

        let (header, file) = match Self::open_file_(&path) {
            Ok(mut file) => {
                if file.metadata()?.len() == 0 {
                    (
                        Header::create_and_write(&mut file, version, Format::Raw)?,
                        Some(file),
                    )
                } else {
                    (
                        Header::import_and_verify(&mut file, version, Format::Raw)?,
                        Some(file),
                    )
                }
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    fs::create_dir_all(Self::folder_(parent, name))?;
                    let mut file = Self::open_file_(&path)?;
                    let header = Header::create_and_write(&mut file, version, Format::Raw)?;
                    (header, None)
                }
                _ => {
                    return Err(e.into());
                }
            },
        };

        let stored_len = if let Some(file) = file {
            (file.metadata()?.len() as usize - HEADER_OFFSET) / Self::SIZE_OF_T
        } else {
            0
        };

        let holes_path = Self::holes_path_(parent, name);
        let holes = if fs::exists(&holes_path)? {
            Some(
                fs::read(&holes_path)?
                    .chunks(size_of::<usize>())
                    .map(|b| -> Result<usize> {
                        Ok(usize::from_ne_bytes(brk_core::copy_first_8bytes(b)?))
                    })
                    .collect::<Result<BTreeSet<usize>>>()?,
            )
        } else {
            None
        };

        Ok(Self {
            file,
            region,
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
            self.file.write_all_to_region_at(
                self.region,
                self.header.inner().read().as_bytes(),
                0,
            )?;
        }
        Ok(())
    }
}

impl<I, T> GenericStoredVec<I, T> for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn read_(&self, index: usize, mmap: &Mmap) -> Result<Option<T>> {
        let index = index * Self::SIZE_OF_T + HEADER_OFFSET;
        let slice = &mmap[index..(index + Self::SIZE_OF_T)];
        T::try_read_from_bytes(slice)
            .map(|v| Some(v))
            .map_err(Error::from)
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn mut_header(&mut self) -> &mut Header {
        &mut self.header
    }

    #[inline]
    fn stored_len(&self) -> usize {
        self.file.get_region(self.region).unwrap().len() as usize / Self::SIZE_OF_T
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

    fn flush(&mut self) -> Result<()> {
        let file_opt = self.write_header_if_needed()?;

        let pushed_len = self.pushed_len();

        let has_new_data = pushed_len != 0;
        let has_updated_data = !self.updated.is_empty();
        let has_holes = !self.holes.is_empty();
        let had_holes = self.has_stored_holes && !has_holes;

        if !has_new_data && !has_updated_data && !has_holes && !had_holes {
            return Ok(());
        }

        if has_new_data || has_updated_data {
            let mut file = file_opt.unwrap_or(self.open_file()?);

            if has_new_data {
                let bytes = {
                    let mut bytes: Vec<u8> = vec![0; pushed_len * Self::SIZE_OF_T];

                    let unsafe_bytes = UnsafeSlice::new(&mut bytes);

                    mem::take(&mut self.pushed)
                        .into_par_iter()
                        .enumerate()
                        .for_each(|(i, v)| {
                            unsafe_bytes.copy_slice(i * Self::SIZE_OF_T, v.as_bytes())
                        });

                    bytes
                };

                self.file_write_all(&mut file, &bytes)?;
            }

            if has_updated_data {
                mem::take(&mut self.updated)
                    .into_iter()
                    .try_for_each(|(i, v)| -> Result<()> {
                        file.write_all_at(
                            v.as_bytes(),
                            ((i * Self::SIZE_OF_T) + HEADER_OFFSET) as u64,
                        )?;
                        Ok(())
                    })?;
            }
        }

        if has_holes || had_holes {
            let holes_path = self.holes_path();
            if has_holes {
                self.has_stored_holes = true;
                fs::write(
                    &holes_path,
                    self.holes
                        .iter()
                        .flat_map(|i| i.to_ne_bytes())
                        .collect::<Vec<_>>(),
                )?;
            } else if had_holes {
                self.has_stored_holes = false;
                let _ = fs::remove_file(&holes_path);
            }
        }

        Ok(())
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
        self.file.truncate_region(self.region, from as u64)
    }

    fn reset(&mut self) -> Result<()> {
        self.set_stored_len(0);
        self.reset_()
    }
}

impl<I, T> AnyVec for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
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

impl<I, T> Clone for RawVec<I, T> {
    fn clone(&self) -> Self {
        Self {
            file: self.file.clone(),
            region: self.region,
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

#[derive(Debug)]
pub struct RawVecIterator<'a, I, T> {
    vec: &'a RawVec<I, T>,
    reader: Reader<'a>,
    index: usize,
}

impl<I, T> BaseVecIterator for RawVecIterator<'_, I, T>
where
    I: StoredIndex,
    T: StoredType,
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
    T: StoredType,
{
    type Item = (I, Cow<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;

        let opt = self
            .vec
            .get_or_read_(index, &self.mmap)
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
    T: StoredType,
{
    type Item = (I, Cow<'a, T>);
    type IntoIter = RawVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        RawVecIterator {
            vec: self,
            reader: self.file.read_region(self.region).unwrap(),
            index: 0,
        }
    }
}

impl<I, T> AnyIterableVec<I, T> for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
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
    T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
