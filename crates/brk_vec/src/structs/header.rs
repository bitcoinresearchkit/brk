use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    os::unix::fs::FileExt,
    sync::Arc,
};

use arc_swap::ArcSwap;
use brk_core::{Error, Height, Result, Version};
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::Format;

const HEADER_VERSION: Version = Version::ONE;
pub const HEADER_OFFSET: usize = size_of::<HeaderInner>();

#[derive(Debug, Clone)]
pub struct Header {
    inner: Arc<ArcSwap<HeaderInner>>,
    modified: bool,
}

impl Header {
    pub fn create_and_write(file: &mut File, vec_version: Version, format: Format) -> Result<Self> {
        let inner = HeaderInner::create_and_write(file, vec_version, format)?;
        Ok(Self {
            inner: Arc::new(ArcSwap::from_pointee(inner)),
            modified: false,
        })
    }

    pub fn import_and_verify(
        file: &mut File,
        vec_version: Version,
        format: Format,
    ) -> Result<Self> {
        let inner = HeaderInner::import_and_verify(file, vec_version, format)?;
        Ok(Self {
            inner: Arc::new(ArcSwap::from_pointee(inner)),
            modified: false,
        })
    }

    pub fn update_height(&mut self, height: Height) {
        self.modified = true;
        self.inner.rcu(|header| {
            let mut header = (**header).clone();
            header.height = height;
            header
        });
    }

    pub fn update_computed_version(&mut self, computed_version: Version) {
        self.modified = true;
        self.inner.rcu(|header| {
            let mut header = (**header).clone();
            header.computed_version = computed_version;
            header
        });
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    pub fn vec_version(&self) -> Version {
        self.inner.load().vec_version
    }

    pub fn computed_version(&self) -> Version {
        self.inner.load().computed_version
    }

    pub fn height(&self) -> Height {
        self.inner.load().height
    }

    pub fn write(&mut self, file: &mut File) -> io::Result<()> {
        self.inner.load().write(file)?;
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
    pub height: Height,
    pub compressed: ZeroCopyBool,
}

impl HeaderInner {
    pub fn create_and_write(file: &mut File, vec_version: Version, format: Format) -> Result<Self> {
        let header = Self {
            header_version: HEADER_VERSION,
            vec_version,
            computed_version: Version::default(),
            height: Height::default(),
            compressed: ZeroCopyBool::from(format),
        };
        header.write(file)?;
        // dbg!(file.bytes().map(|b| b.unwrap()).collect::<Vec<_>>());
        file.seek(SeekFrom::End(0))?;
        Ok(header)
    }

    pub fn write(&self, file: &mut File) -> io::Result<()> {
        file.write_all_at(self.as_bytes(), 0)
    }

    pub fn import_and_verify(
        file: &mut File,
        vec_version: Version,
        format: Format,
    ) -> Result<Self> {
        if file.metadata()?.len() < HEADER_OFFSET as u64 {
            return Err(Error::WrongLength);
        }

        let mut buf = [0; HEADER_OFFSET];
        file.read_exact(&mut buf)?;

        let header = HeaderInner::read_from_bytes(&buf)?;

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
#[repr(C)]
pub struct ZeroCopyBool(u32);

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
