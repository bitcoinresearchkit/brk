use std::{
    fs::{self, OpenOptions},
    os::unix::io::AsRawFd,
    path::Path,
    sync::Arc,
};

use brk_core::{Error, Result};
use libc::off_t;
use memmap2::{MmapMut, MmapOptions};
use parking_lot::{RwLock, RwLockReadGuard};

mod layout;
mod reader;
mod region;

use layout::*;
use region::*;

use crate::file::reader::Reader;

pub const PAGE_SIZE: usize = 4096;

pub struct File {
    layout: Arc<RwLock<Layout>>,
    file: Arc<RwLock<fs::File>>,
    mmap: Arc<RwLock<MmapMut>>,
}

impl File {
    pub fn open(path: &Path) -> Result<Self> {
        fs::create_dir_all(path)?;

        let layout = Layout::open(&path.join("layout.dat"))?;

        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(false)
            .open(path.join("data.dat"))?;

        let mmap = Self::mmap(&file)?;

        Ok(Self {
            file: Arc::new(RwLock::new(file)),
            mmap: Arc::new(RwLock::new(mmap)),
            layout: Arc::new(RwLock::new(layout)),
        })
    }

    /// len % PAGE_SIZE == 0
    pub fn grow_if_needed(&self, len: usize) -> Result<()> {
        assert!(len % PAGE_SIZE == 0);
        let file = self.file.write();
        let len = len as u64;
        if file.metadata()?.len() < len {
            file.set_len(len)?;
            self.remap_(&file)
        } else {
            Ok(())
        }
    }

    pub fn get_or_create_region_from_id(&mut self, id: String) -> Result<usize> {
        self.layout.write().get_or_create_region_from_id(id)
    }

    pub fn create_reader<'a, 'b>(&'a self, region_id: usize) -> Result<Reader<'a, 'b>>
    where
        'a: 'b,
    {
        let layout: RwLockReadGuard<'a, Layout> = self.layout.read();
        let mmap: RwLockReadGuard<'a, MmapMut> = self.mmap.read();

        let region: RwLockReadGuard<'b, Region> =
            layout.get(region_id).ok_or(Error::Str("Unknown region"))?;

        Ok(Reader::new(mmap, layout, region))
    }

    fn remap(&self) -> Result<()> {
        *self.mmap.write() = Self::mmap(&self.file.read())?;
        Ok(())
    }
    fn remap_(&self, file: &fs::File) -> Result<()> {
        *self.mmap.write() = Self::mmap(file)?;
        Ok(())
    }
    fn mmap(file: &fs::File) -> Result<MmapMut> {
        Ok(unsafe { MmapOptions::new().map_mut(file)? })
    }

    pub fn delete() {}

    #[cfg(target_os = "macos")]
    fn punch_hole(file: &fs::File, offset: u64, length: u64) -> Result<()> {
        let fpunchhole = FPunchhole {
            fp_flags: 0,
            reserved: 0,
            fp_offset: offset as libc::off_t,
            fp_length: length as libc::off_t,
        };

        let result = unsafe {
            libc::fcntl(
                file.as_raw_fd(),
                libc::F_PUNCHHOLE,
                &fpunchhole as *const FPunchhole,
            )
        };

        if result == -1 {
            let err = std::io::Error::last_os_error();
            return Err(Error::String(format!("Failed to punch hole: {err}")));
        }

        Ok(())
    }
}

#[repr(C)]
struct FPunchhole {
    fp_flags: u32,
    reserved: u32,
    fp_offset: off_t,
    fp_length: off_t,
}
