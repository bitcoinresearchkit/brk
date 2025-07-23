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
pub const PAGE_SIZE_MINUS_1: u64 = PAGE_SIZE - 1;

pub struct File {
    // TODO: Remove pub
    pub regions: RwLock<Regions>,
    pub layout: RwLock<Layout>,
    pub file: RwLock<fs::File>,
    pub mmap: RwLock<MmapMut>,
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

    pub fn set_min_len(&self, len: u64) -> Result<()> {
        let len = Self::ceil_number_to_page_size_multiple(len);

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

    pub fn set_min_regions(&self, regions: usize) -> Result<()> {
        self.regions
            .write()
            .set_min_len((regions * SIZE_OF_REGION) as u64)?;
        self.set_min_len(regions as u64 * PAGE_SIZE)
    }

    pub fn create_region_if_needed(&self, id: &str) -> Result<usize> {
        if let Some(index) = self.regions.read().get_region_index_from_id(id.to_owned()) {
            return Ok(index);
        }
        let mut regions = self.regions.write();
        let mut layout = self.layout.write();

        let start = if let Some(start) = layout.find_smallest_adequate_hole(PAGE_SIZE) {
            layout.remove_or_compress_hole_to_right(start, PAGE_SIZE);
            start
        } else {
            let start = layout
                .get_last_region_index()
                .map(|index| {
                    let region_opt = regions.get_region_from_index(index);
                    let region = region_opt.as_ref().unwrap().read();
                    region.start() + region.reserved()
                })
                .unwrap_or_default();
            self.set_min_len(start + PAGE_SIZE)?;
            start
        };

        let index = regions.create_region(id.to_owned(), start)?;

        layout.insert_region(start, index);

        Ok(index)
    }

    pub fn get_region(&self, index: usize) -> Option<Arc<RwLock<Region>>> {
        self.regions.read().get_region_from_index(index)
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
    pub fn write_all(&self, region: usize, data: &[u8]) -> Result<()> {
        self.write_all_at_(region, data, None)
    }

    #[inline]
    pub fn write_all_at(&self, region: usize, data: &[u8], at: u64) -> Result<()> {
        self.write_all_at_(region, data, Some(at))
    }

    fn write_all_at_(&self, region_index: usize, data: &[u8], at: Option<u64>) -> Result<()> {
        let Some(region) = self.regions.read().get_region_from_index(region_index) else {
            return Err(Error::Str("Unknown region"));
        };
        let region_lock = region.read();
        let start = region_lock.start();
        let reserved = region_lock.reserved();
        let left = region_lock.left();
        let len = region_lock.len();
        let data_len = data.len() as u64;
        drop(region_lock);
        let write_start = at.unwrap_or(start + len);

        if at.is_some_and(|at| at < start || at >= start + reserved) {
            return Err(Error::Str("Invalid at parameter"));
        }

        // Write to reserved space if possible
        let at_left = at.map_or_else(|| left, |at| reserved - at);
        if at_left >= data_len {
            let len = reserved - at_left.min(left) + data_len;

            dbg!(write_start);

            self.write(write_start, data);

            let regions = self.regions.read();
            let mut region_lock = region.write();
            dbg!(len);
            region_lock.set_len(len);
            regions.write_to_mmap(&region_lock, region_index);
            return Ok(());
        }

        let mut layout_lock = self.layout.write();

        let new_len = len + data_len;
        debug_assert!(new_len > reserved);
        let mut new_reserved = reserved;
        while new_len < new_reserved {
            new_reserved *= 2;
        }
        let added_reserve = new_reserved - reserved;

        // If is last continue writing
        if layout_lock.is_last_region(region_index) {
            self.set_min_len(start + new_reserved)?;

            self.write(write_start, data);

            let regions = self.regions.read();
            let mut region_lock = region.write();
            region_lock.set_len(new_len);
            region_lock.set_reserved(new_reserved);
            regions.write_to_mmap(&region_lock, region_index);
            return Ok(());
        }

        // Expand region to the right if gap is wide enough
        let hole_start = start + reserved;
        let gap = layout_lock.get_hole(hole_start);
        if gap.is_some_and(|gap| gap >= added_reserve) {
            self.write(write_start, data);

            layout_lock.remove_or_compress_hole_to_right(hole_start, added_reserve);
            drop(layout_lock);

            let regions = self.regions.read();
            let mut region_lock = region.write();
            region_lock.set_len(new_len);
            region_lock.set_reserved(new_reserved);
            regions.write_to_mmap(&region_lock, region_index);
            return Ok(());
        }

        // Find hole big enough to move the region
        if let Some(hole_start) = layout_lock.find_smallest_adequate_hole(new_reserved) {
            self.write(
                hole_start,
                &self.mmap.read()[start as usize..(start + len) as usize],
            );
            self.write(hole_start + len, data);

            let regions = self.regions.read();
            let mut region_lock = region.write();

            layout_lock.remove_or_compress_hole_to_right(hole_start, new_reserved);
            layout_lock.move_region(hole_start, region_index, &region_lock)?;

            region_lock.set_start(hole_start);
            region_lock.set_len(new_len);
            region_lock.set_reserved(new_reserved);
            regions.write_to_mmap(&region_lock, region_index);

            drop(layout_lock);

            self.punch_hole(start, reserved)?;

            return Ok(());
        }

        // Write at the end
        let regions = self.regions.read();
        let mut region_lock = region.write();
        let (last_region_start, last_region_index) = layout_lock.get_last_region().unwrap();
        let new_start = last_region_start
            + regions
                .get_region_from_index(last_region_index)
                .unwrap()
                .read()
                .reserved();
        self.set_min_len(new_start + new_reserved)?;

        self.write(
            new_start,
            &self.mmap.read()[start as usize..(start + len) as usize],
        );
        self.write(new_start + len, data);

        region_lock.set_start(new_start);
        region_lock.set_len(new_len);
        region_lock.set_reserved(new_reserved);
        regions.write_to_mmap(&region_lock, region_index);

        self.punch_hole(start, reserved)?;

        Ok(())
    }

    fn write(&self, start: u64, data: &[u8]) {
        let mmap = self.mmap.read();

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
        let reserved = region_.reserved();

        if from < start {
            return Err(Error::Str("Truncating too much"));
        } else if from >= len {
            return Err(Error::Str("Not truncating enough"));
        }

        region_.set_len(from);

        let end = start + reserved;
        let start = Self::ceil_number_to_page_size_multiple(from);
        if start > end {
            unreachable!("Should not be possible");
        } else if start < end {
            self.punch_hole(start, end - start)?;
        }
        Ok(())
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

    fn ceil_number_to_page_size_multiple(num: u64) -> u64 {
        (num + PAGE_SIZE_MINUS_1) & !PAGE_SIZE_MINUS_1
    }
}

#[repr(C)]
struct FPunchhole {
    fp_flags: u32,
    reserved: u32,
    fp_offset: off_t,
    fp_length: off_t,
}
