use std::sync::Arc;

use parking_lot::RwLock;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{Error, File, Result, Stamp, Version};

use super::Format;

const HEADER_VERSION: Version = Version::ONE;
pub const HEADER_OFFSET: usize = size_of::<HeaderInner>();

#[derive(Debug, Clone)]
pub struct Header {
    inner: Arc<RwLock<HeaderInner>>,
    modified: bool,
}

impl Header {
    pub fn create_and_write(
        file: &File,
        region_index: usize,
        vec_version: Version,
        format: Format,
    ) -> Result<Self> {
        let inner = HeaderInner::create_and_write(file, region_index, vec_version, format)?;
        Ok(Self {
            inner: Arc::new(RwLock::new(inner)),
            modified: false,
        })
    }

    pub fn import_and_verify(
        file: &File,
        region_index: usize,
        region_len: u64,
        vec_version: Version,
        format: Format,
    ) -> Result<Self> {
        let inner =
            HeaderInner::import_and_verify(file, region_index, region_len, vec_version, format)?;
        Ok(Self {
            inner: Arc::new(RwLock::new(inner)),
            modified: false,
        })
    }

    pub fn update_stamp(&mut self, stamp: Stamp) {
        self.modified = true;
        self.inner.write().stamp = stamp;
    }

    pub fn update_format(&mut self, format: Format) {
        self.modified = true;
        self.inner.write().compressed = ZeroCopyBool::from(format);
    }

    pub fn update_computed_version(&mut self, computed_version: Version) {
        self.modified = true;
        self.inner.write().computed_version = computed_version;
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    pub fn vec_version(&self) -> Version {
        self.inner.read().vec_version
    }

    pub fn computed_version(&self) -> Version {
        self.inner.read().computed_version
    }

    pub fn stamp(&self) -> Stamp {
        self.inner.read().stamp
    }

    pub fn write(&mut self, file: &File, region_index: usize) -> Result<()> {
        self.inner.read().write(file, region_index)?;
        self.modified = false;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, KnownLayout)]
struct HeaderInner {
    pub header_version: Version,
    pub vec_version: Version,
    pub computed_version: Version,
    pub stamp: Stamp,
    pub compressed: ZeroCopyBool,
}

impl HeaderInner {
    pub fn create_and_write(
        file: &File,
        region_index: usize,
        vec_version: Version,
        format: Format,
    ) -> Result<Self> {
        let header = Self {
            header_version: HEADER_VERSION,
            vec_version,
            computed_version: Version::default(),
            stamp: Stamp::default(),
            compressed: ZeroCopyBool::from(format),
        };
        header.write(file, region_index)?;
        Ok(header)
    }

    pub fn write(&self, file: &File, region_index: usize) -> Result<()> {
        file.write_all_to_region_at(region_index.into(), self.as_bytes(), 0)
    }

    pub fn import_and_verify(
        file: &File,
        region_index: usize,
        region_len: u64,
        vec_version: Version,
        format: Format,
    ) -> Result<Self> {
        let len = region_len;

        if len < HEADER_OFFSET as u64 {
            return Err(Error::WrongLength);
        }

        let reader = file.create_region_reader(region_index.into())?;
        let slice = reader.read(0, HEADER_OFFSET as u64);
        let header = HeaderInner::read_from_bytes(slice)?;

        if header.header_version != HEADER_VERSION {
            return Err(Error::DifferentVersion {
                found: header.header_version,
                expected: HEADER_VERSION,
            });
        }
        if header.vec_version != vec_version {
            return Err(Error::DifferentVersion {
                found: header.vec_version,
                expected: vec_version,
            });
        }
        if header.compressed.is_broken() {
            return Err(Error::WrongEndian);
        }
        if (header.compressed.is_true() && format.is_raw())
            || (header.compressed.is_false() && format.is_compressed())
        {
            return Err(Error::DifferentCompressionMode);
        }

        Ok(header)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    IntoBytes,
    Immutable,
    KnownLayout,
)]
pub struct ZeroCopyBool(u64);

impl ZeroCopyBool {
    pub const TRUE: Self = Self(1);
    pub const FALSE: Self = Self(0);

    pub fn is_true(&self) -> bool {
        *self == Self::TRUE
    }

    pub fn is_false(&self) -> bool {
        *self == Self::FALSE
    }

    pub fn is_broken(&self) -> bool {
        *self > Self::TRUE
    }
}

impl From<Format> for ZeroCopyBool {
    fn from(value: Format) -> Self {
        if value.is_raw() {
            Self::FALSE
        } else {
            Self::TRUE
        }
    }
}
