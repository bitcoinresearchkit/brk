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
mod regions;

use layout::*;
use reader::*;
use region::*;
use regions::*;

pub const PAGE_SIZE: u64 = 4096;

pub struct File {
    regions: RwLock<Regions>,
    layout: RwLock<Layout>,
    file: RwLock<fs::File>,
    mmap: RwLock<MmapMut>,
}

impl File {
    pub fn open(path: &Path) -> Result<Self> {
        fs::create_dir_all(path)?;

        let regions = Regions::open(path)?;
        let layout = Layout::from(&regions);

        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(false)
            .open(path.join("data"))?;

        let mmap = Self::mmap(&file)?;

        Ok(Self {
            file: RwLock::new(file),
            mmap: RwLock::new(mmap),
            regions: RwLock::new(regions),
            layout: RwLock::new(layout),
        })
    }

    /// len % PAGE_SIZE == 0
    pub fn set_min_len(&self, len: u64) -> Result<()> {
        assert!(len % PAGE_SIZE == 0);
        if self.file.read().metadata()?.len() < len {
            let mut mmap = self.mmap.write();
            let file = self.file.write();
            file.set_len(len)?;
            *mmap = Self::mmap(&file)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn get_or_create(&self, id: String) -> Result<usize> {
        if let Some(index) = self.regions.read().get_region_index_from_id(id.clone()) {
            return Ok(index);
        }
        let mut regions = self.regions.write();
        let mut layout = self.layout.write();

        let start = if let Some(start) = layout.find_smallest_adequate_hole(PAGE_SIZE) {
            layout.remove_or_compress_hole_to_right(start, PAGE_SIZE);
            start
        } else {
            let start = layout
                .get_last_region()
                .map(|index| {
                    let region_opt = regions.get_region_from_index(index);
                    let region = region_opt.as_ref().unwrap().read();
                    region.start() + region.reserved()
                })
                .unwrap_or_default();
            self.set_min_len(start + PAGE_SIZE)?;
            start
        };

        let index = regions.create_region(id, start)?;

        layout.insert_region(start, index);

        Ok(index)
    }

    pub fn read<'a>(&'a self, index: usize) -> Result<Reader<'a>> {
        let mmap: RwLockReadGuard<'a, MmapMut> = self.mmap.read();
        let region: RwLockReadGuard<'static, Region> = unsafe {
            std::mem::transmute(
                self.regions
                    .read()
                    .get_region_from_index(index)
                    .ok_or(Error::Str("Unknown region"))?
                    .read(),
            )
        };
        Ok(Reader::new(mmap, region))
    }

    #[inline]
    pub fn write_all(&mut self, region: usize, data: &[u8]) -> Result<()> {
        self.write_all_at_(region, data, None)
    }

    #[inline]
    pub fn write_all_at(&mut self, region: usize, data: &[u8], at: u64) -> Result<()> {
        self.write_all_at_(region, data, Some(at))
    }

    fn write_all_at_(&mut self, region: usize, data: &[u8], at: Option<u64>) -> Result<()> {
        let Some(region) = self.regions.read().get_region_from_index(region) else {
            return Err(Error::Str("Unknown region"));
        };
        let region_lock = region.read();
        let start = region_lock.start();
        let reserved = region_lock.reserved();
        let left = region_lock.left();
        let data_len = data.len() as u64;
        drop(region_lock);

        let new_left = at.map_or_else(|| left, |at| reserved - (at - start));
        let new_len = reserved - new_left;

        // Write to reserved space if possible
        if new_left >= data_len {
            Self::write_to_mmap(&self.mmap.read(), at.unwrap_or(start), data);

            let mut region_lock = region.write();
            region_lock.set_len(new_len);

            // TODO: Flush layout
            return Ok(());
        }

        let mut layout_lock = self.layout.write();

        let hole_start = start + reserved;
        let hole = layout_lock.get_hole(hole_start);

        // Expand region to the right if possible
        if hole.is_some_and(|gap| gap >= reserved) {
            Self::write_to_mmap(&self.mmap.read(), at.unwrap_or(start), data);

            layout_lock.remove_or_compress_hole_to_right(hole_start, reserved);
            drop(layout_lock);

            let mut region_lock = region.write();
            region_lock.set_len(new_len);
            region_lock.set_reserved(reserved * 2);

            // TODO: Flush layout
            return Ok(());
        }

        let reserved = reserved * 2;

        // Find hole big enough to move the current region or the next region depending on which is smaller to if possible
        if let Some(hole_start) = layout_lock.find_smallest_adequate_hole(reserved) {
            layout_lock.remove_or_compress_hole_to_right(hole_start, reserved);
            // TODO: Before every drop of layout.write flush to disk
            drop(layout_lock);

            // write
            Self::write_to_mmap(&self.mmap.read(), at.unwrap_or(start), data);

            let mut region_lock = region.write();
            region_lock.set_start(hole_start);
            region_lock.set_len(new_len);
            region_lock.set_reserved(reserved * 2);

            // TODO: create hole in prev position

            Self::write_to_mmap(&self.mmap.read(), at.unwrap_or(start), data);

            // TODO: Flush layout
            return Ok(());
        }

        // copy region to new position then lock and update region meta then remove

        // let old_length = region_lock.len();
        // let new_length = old_length + data_len as u64;

        // self.layout.ho

        todo!();

        Ok(())
    }

    fn write_to_mmap(mmap: &MmapMut, start: u64, data: &[u8]) {
        let data_len = data.len();
        let start = start as usize;
        let end = start + data_len;

        if end > mmap.len() {
            unreachable!("Trying to write beyond mmap")
        }

        let slice = unsafe { std::slice::from_raw_parts_mut(mmap.as_ptr() as *mut u8, mmap.len()) };

        slice[start..end].copy_from_slice(data);
    }

    pub fn truncate(&self, index: usize, from: u64) -> Result<()> {
        let Some(region) = self.regions.read().get_region_from_index(index) else {
            return Err(Error::Str("Unknown region"));
        };
        let mut region_ = region.write();
        let start = region_.start();
        let len = region_.len();

        if from <= start {
            return Err(Error::Str("Truncating too much"));
        } else if from >= len {
            return Err(Error::Str("Not truncating enough"));
        }

        region_.set_len(from);

        // TODO: Widen hole if present and needed (if truncating a big portion)
        // Not needed in BRK and with hole punching it's not a big deal but good to have nonetheless

        self.punch_hole(from, region_.left())
    }

    pub fn remove(&self, index: usize) -> Result<Option<Arc<RwLock<Region>>>> {
        let mut regions = self.regions.write();
        let mut layout = self.layout.write();
        let Some(region) = regions.remove_region(index)? else {
            return Ok(None);
        };
        let region_ = region.write();
        layout.remove_region(index, &region_)?;
        self.punch_hole(region_.start(), region_.len())?;
        drop(region_);
        Ok(Some(region))
    }

    fn mmap(file: &fs::File) -> Result<MmapMut> {
        Ok(unsafe { MmapOptions::new().map_mut(file)? })
    }

    fn punch_hole(&self, start: u64, length: u64) -> Result<()> {
        let file = self.file.write();
        Self::punch_hole_macos(&file, start, length)
    }

    #[cfg(target_os = "macos")]
    fn punch_hole_macos(file: &fs::File, start: u64, length: u64) -> Result<()> {
        let fpunchhole = FPunchhole {
            fp_flags: 0,
            reserved: 0,
            fp_offset: start as libc::off_t,
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
