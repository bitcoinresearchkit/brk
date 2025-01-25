use std::{
    cmp::Ordering,
    fmt::{self, Debug},
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    mem,
    ops::{Deref, Range},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use memmap2::{Mmap, MmapOptions};

///
/// A very small, fast, efficient and simple storable Vec
///
/// Reads (imports of Mmap) are lazy
///
/// Stores only raw data without any overhead, and doesn't even have a header (TODO: which it should, at least to Err if wrong endian)
///
/// The file isn't portable for speed reasons (TODO: but could be ?)
///
/// If you don't call `.flush()` it just acts as a normal Vec
///
#[derive(Debug)]
pub struct StorableVec<I, T> {
    pathbuf: PathBuf,
    file: File,
    cache: Vec<OnceLock<Box<Mmap>>>, // Boxed to reduce the size of the lock (24 > 16)
    disk_len: usize,
    pushed: Vec<T>,
    // updated: BTreeMap<usize, T>,
    // inserted: BTreeMap<usize, T>,
    // removed: BTreeSet<usize>,
    phantom: PhantomData<I>,
}

/// In bytes
const MAX_PAGE_SIZE: usize = 4 * 4096;
const ONE_MB: usize = 1000 * 1024;
const MAX_CACHE_SIZE: usize = 100 * ONE_MB;

impl<I, T> StorableVec<I, T>
where
    I: Into<usize>,
    T: Sized + Debug + Clone,
{
    pub const SIZE_OF_T: usize = size_of::<T>();
    pub const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE_OF_T;
    /// In bytes
    pub const PAGE_SIZE: usize = Self::PER_PAGE * Self::SIZE_OF_T;
    pub const CACHE_LENGTH: usize = usize::MAX;
    // pub const CACHE_LENGTH: usize = MAX_CACHE_SIZE / Self::PAGE_SIZE;

    pub fn import(path: &Path) -> Result<Self, io::Error> {
        let file = Self::open_file(path)?;

        let mut this = Self {
            pathbuf: path.to_owned(),
            disk_len: Self::byte_index_to_index(file.metadata()?.len() as usize),
            file,
            cache: vec![],
            pushed: vec![],
            // updated: BTreeMap::new(),
            // inserted: BTreeMap::new(),
            // removed: BTreeSet::new(),
            phantom: PhantomData,
        };

        this.reset_cache();

        Ok(this)
    }

    fn reset_cache(&mut self) {
        // let len = (self.disk_len as f64 / Self::PER_PAGE as f64).ceil() as usize;
        // self.cache.clear();
        // self.cache.resize_with(len, Default::default);

        self.cache.iter_mut().for_each(|lock| {
            lock.take();
        });
        let len = (self.disk_len as f64 / Self::PER_PAGE as f64).ceil() as usize;
        self.cache
            .resize_with(Self::CACHE_LENGTH.min(len), Default::default);
        self.cache.shrink_to_fit();
    }

    fn open_file(path: &Path) -> Result<File, io::Error> {
        OpenOptions::new()
            .read(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(path)
    }

    #[inline]
    fn index_to_byte_range(index: usize) -> Range<usize> {
        let index = Self::index_to_byte_index(index) % Self::PAGE_SIZE;
        index..(index + Self::SIZE_OF_T)
    }

    #[inline]
    fn index_to_byte_index(index: usize) -> usize {
        index * Self::SIZE_OF_T
    }

    #[inline]
    fn byte_index_to_index(byte_index: usize) -> usize {
        byte_index / Self::SIZE_OF_T
    }

    fn index_to_pushed_index(&self, index: usize) -> Result<Option<usize>> {
        if index >= self.disk_len {
            let index = index - self.disk_len;
            if index >= self.pushed.len() {
                Err(Error::IndexTooHigh)
            } else {
                Ok(Some(index))
            }
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn get(&self, index: I) -> Result<Option<Value<'_, T>>> {
        self.get_(index.into())
    }
    pub fn get_(&self, index: usize) -> Result<Option<Value<'_, T>>> {
        match self.index_to_pushed_index(index) {
            Ok(index) => {
                if let Some(index) = index {
                    return Ok(self.pushed.get(index).map(|v| Value::Ref(v)));
                }
            }
            Err(Error::IndexTooHigh) => return Ok(None),
            Err(error) => return Err(error),
        }

        // if !self.updated.is_empty() {
        //     if let Some(v) = self.updated.get(&index) {
        //         return Ok(Some(v));
        //     }
        // }

        let byte_index = Self::index_to_byte_index(index);
        let page_index = index / Self::PER_PAGE;
        let last_index = self.disk_len - 1;
        let max_page_index = last_index / Self::PER_PAGE;
        let min_page_index = (max_page_index + 1)
            .checked_sub(self.cache.len())
            .unwrap_or_default();

        if page_index >= min_page_index {
            let mmap = &**self
                .cache
                .get(page_index - min_page_index)
                .ok_or(Error::MmapsVecIsTooSmall)?
                .get_or_init(|| {
                    Box::new(unsafe {
                        MmapOptions::new()
                            .len(Self::PAGE_SIZE)
                            .offset((page_index * Self::PAGE_SIZE) as u64)
                            .map(&self.file)
                            .unwrap()
                    })
                });

            let range = Self::index_to_byte_range(index);

            let slice = &mmap[range];

            Ok(Some(Value::Ref(T::unsafe_try_from_slice(slice)?)))
        } else {
            let mut file = Self::open_file(&self.pathbuf).unwrap();

            file.seek(SeekFrom::Start(byte_index as u64)).unwrap();

            let mut buf = vec![0; Self::SIZE_OF_T];
            file.read_exact(&mut buf).unwrap();

            let value = T::unsafe_try_from_slice(&buf[..])?;

            Ok(Some(Value::Owned(value.to_owned())))
        }
    }
    pub fn get_or_default(&self, index: I) -> Result<T>
    where
        T: Default + Clone,
    {
        Ok(self
            .get(index)?
            .map(|v| (*v).clone())
            .unwrap_or(Default::default()))
    }

    #[allow(unused)]
    pub fn first(&self) -> Result<Option<Value<'_, T>>> {
        self.get_(0)
    }

    #[allow(unused)]
    pub fn last(&self) -> Result<Option<Value<'_, T>>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        self.get_(len - 1)
    }

    pub fn push(&mut self, value: T) {
        self.pushed.push(value)
    }

    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        self.push_if_needed_(index.into(), value)
    }
    pub fn push_if_needed_(&mut self, index: usize, value: T) -> Result<()> {
        let len = self.len();
        match len.cmp(&index) {
            Ordering::Greater => {
                // dbg!(len, index);
                // panic!();
                Ok(())
            }
            Ordering::Equal => {
                self.push(value);
                Ok(())
            }
            Ordering::Less => Err(Error::IndexTooHigh),
        }
    }

    // pub fn update(&mut self, index: I, value: T) -> Result<()> {
    //     self._update(index.into(), value)
    // }
    // pub fn update_(&mut self, index: usize, value: T) -> Result<()> {
    //     if let Some(index) = self.index_to_pushed_index(index) {
    //         self.pushed[index] = value;
    //     } else {
    //         self.updated.insert(index, value);
    //     }
    //     Ok(())
    // }

    // pub fn fetch_update(&mut self, index: I, value: T) -> Result<T>
    // where
    //     T: Clone,
    // {
    //     self._fetch_update(index.into(), value)
    // }
    // pub fn fetch_update_(&mut self, index: usize, value: T) -> Result<T>
    // where
    //     T: Clone,
    // {
    //     let prev_opt = self.updated.insert(index, value);
    //     if let Some(prev) = prev_opt {
    //         Ok(prev)
    //     } else {
    //         Ok(self
    //             ._get(index)?
    //             .ok_or(Error::ExpectFileToHaveIndex)?
    //             .clone())
    //     }
    // }

    // pub fn remove(&mut self, index: I) {
    //     self._remove(index.into())
    // }
    // pub fn remove_(&mut self, index: usize) {
    //     self.removed.insert(index);
    // }

    pub fn len(&self) -> usize {
        self.disk_len + self.pushed.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn has(&self, index: I) -> bool {
        self.has_(index.into())
    }
    pub fn has_(&self, index: usize) -> bool {
        index < self.len()
    }

    pub fn hasnt(&self, index: I) -> bool {
        self.hasnt_(index.into())
    }
    pub fn hasnt_(&self, index: usize) -> bool {
        !self.has_(index)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.reset_cache();

        if self.pushed.is_empty() {
            return Ok(());
        }

        self.disk_len += self.pushed.len();

        let mut bytes: Vec<u8> = vec![];

        mem::take(&mut self.pushed)
            .into_iter()
            .for_each(|v| bytes.extend_from_slice(v.unsafe_as_slice()));

        self.file.write_all(&bytes)?;

        Ok(())
    }
}

pub trait AnyStorableVec {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn flush(&mut self) -> io::Result<()>;
}

impl<I, T> AnyStorableVec for StorableVec<I, T>
where
    I: Into<usize>,
    T: Sized + Debug + Clone,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush()
    }
}

#[derive(Debug, Clone)]
pub enum Value<'a, T> {
    Ref(&'a T),
    Owned(T),
}

impl<T> Value<'_, T>
where
    T: Sized + Debug + Clone,
{
    pub fn into_inner(self) -> T {
        match self {
            Self::Ref(t) => t.to_owned(),
            Self::Owned(t) => t,
        }
    }
}
impl<T> Deref for Value<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(t) => t,
            Self::Owned(t) => t,
        }
    }
}
impl<T> AsRef<T> for Value<'_, T>
where
    T: Sized + Debug + Clone,
{
    fn as_ref(&self) -> &T {
        match self {
            Self::Ref(t) => t,
            Self::Owned(t) => t,
        }
    }
}

pub trait UnsafeSizedSerDe
where
    Self: Sized,
{
    const SIZE: usize = size_of::<Self>();

    fn unsafe_try_from_slice(slice: &[u8]) -> Result<&Self> {
        let (prefix, shorts, suffix) = unsafe { slice.align_to::<Self>() };

        if !prefix.is_empty() || shorts.len() != 1 || !suffix.is_empty() {
            // dbg!(&slice, &prefix, &shorts, &suffix);
            return Err(Error::FailedToAlignToSelf);
        }

        Ok(&shorts[0])
    }

    fn unsafe_as_slice(&self) -> &[u8] {
        let data: *const Self = self;
        let data: *const u8 = data as *const u8;
        unsafe { std::slice::from_raw_parts(data, Self::SIZE) }
    }
}
impl<T> UnsafeSizedSerDe for T {}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    MmapsVecIsTooSmall,
    FailedToAlignToSelf,
    IndexTooHigh,
    ExpectFileToHaveIndex,
    ExpectVecToHaveIndex,
}
impl fmt::Display for Error {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MmapsVecIsTooSmall => write!(f, "Mmaps vec is too small"),
            Error::FailedToAlignToSelf => write!(f, "Failed to align_to for T"),
            Error::IndexTooHigh => write!(f, "Index too high"),
            Error::ExpectFileToHaveIndex => write!(f, "Expect file to have index"),
            Error::ExpectVecToHaveIndex => write!(f, "Expect vec to have index"),
        }
    }
}
impl std::error::Error for Error {}
