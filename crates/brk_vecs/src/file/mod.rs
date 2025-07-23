use std::{
    fs::{self, OpenOptions},
    io::Write,
    os::unix::io::AsRawFd,
    path::{Path, PathBuf},
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
pub use reader::Reader;
use region::*;
use regions::*;

pub const PAGE_SIZE: u64 = 4096;
pub const PAGE_SIZE_MINUS_1: u64 = PAGE_SIZE - 1;

#[derive(Debug)]
pub struct File {
    path: PathBuf,
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
            .open(Self::data_path_(path))?;

        let mmap = Self::create_mmap(&file)?;

        Ok(Self {
            path: path.to_owned(),
            file: RwLock::new(file),
            mmap: RwLock::new(mmap),
            regions: RwLock::new(regions),
            layout: RwLock::new(layout),
        })
    }

    pub fn file_len(&self) -> Result<u64> {
        Ok(self.file.read().metadata()?.len())
    }

    pub fn set_min_len(&self, len: u64) -> Result<()> {
        let len = Self::ceil_number_to_page_size_multiple(len);

        if self.file_len()? < len {
            let mut mmap = self.mmap.write();
            let file = self.file.write();
            file.set_len(len)?;
            *mmap = Self::create_mmap(&file)?;
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
        if let Some(index) = self.regions.read().get_region_index_from_id(id) {
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

    pub fn get_region(&self, index: usize) -> Result<RwLockReadGuard<'static, Region>> {
        let regions = self.regions.read();
        let region_opt = regions.get_region_from_index(index);
        let region_arc = region_opt.ok_or(Error::Str("Unknown region"))?;
        let region = region_arc.read();
        let region: RwLockReadGuard<'static, Region> = unsafe { std::mem::transmute(region) };
        Ok(region)
    }

    pub fn read_region<'a>(&'a self, index: usize) -> Result<Reader<'a>> {
        let mmap: RwLockReadGuard<'a, MmapMut> = self.mmap.read();
        let region = self.get_region(index)?;
        Ok(Reader::new(mmap, region))
    }

    #[inline]
    pub fn write_all_to_region(&self, region: usize, data: &[u8]) -> Result<()> {
        self.write_all_to_region_at_(region, data, None)
    }

    #[inline]
    pub fn write_all_to_region_at(&self, region: usize, data: &[u8], at: u64) -> Result<()> {
        self.write_all_to_region_at_(region, data, Some(at))
    }

    fn write_all_to_region_at_(
        &self,
        region_index: usize,
        data: &[u8],
        at: Option<u64>,
    ) -> Result<()> {
        let Some(region) = self.regions.read().get_region_from_index(region_index) else {
            return Err(Error::Str("Unknown region"));
        };
        let region_lock = region.read();
        let start = region_lock.start();
        let reserved = region_lock.reserved();
        let len = region_lock.len();
        let data_len = data.len() as u64;
        drop(region_lock);
        let new_len = at.map_or(len + data_len, |at| (at + data_len).max(len));
        // dbg!(new_len);
        let write_start = at.unwrap_or(start + len);

        if at.is_some_and(|at| at < start || at >= start + reserved) {
            return Err(Error::Str("Invalid at parameter"));
        }

        // Write to reserved space if possible
        if new_len <= reserved {
            // dbh!("Write to reserved space");
            // dbg!(write_start);

            self.write(write_start, data);

            let regions = self.regions.read();
            let mut region_lock = region.write();
            if len != new_len {
                region_lock.set_len(new_len);
            }
            regions.write_to_mmap(&region_lock, region_index);
            return Ok(());
        }

        let mut layout_lock = self.layout.write();

        debug_assert!(new_len > reserved);
        let mut new_reserved = reserved;
        while new_len > new_reserved {
            new_reserved *= 2;
        }
        debug_assert!(new_len <= new_reserved);
        let added_reserve = new_reserved - reserved;

        // If is last continue writing
        if layout_lock.is_last_region(region_index) {
            // dbg!("Append to file");
            // dbg!(start, new_reserved, start + new_reserved);

            self.set_min_len(start + new_reserved)?;

            self.write(write_start, data);

            let regions = self.regions.read();
            let mut region_lock = region.write();
            region_lock.set_reserved(new_reserved);
            region_lock.set_len(new_len);
            regions.write_to_mmap(&region_lock, region_index);
            return Ok(());
        }

        // Expand region to the right if gap is wide enough
        let hole_start = start + reserved;
        let gap = layout_lock.get_hole(hole_start);
        if gap.is_some_and(|gap| gap >= added_reserve) {
            // dbg!("Expand to hole");

            self.write(write_start, data);

            layout_lock.remove_or_compress_hole_to_right(hole_start, added_reserve);
            drop(layout_lock);

            let regions = self.regions.read();
            let mut region_lock = region.write();
            region_lock.set_reserved(new_reserved);
            region_lock.set_len(new_len);
            regions.write_to_mmap(&region_lock, region_index);
            return Ok(());
        }

        // Find hole big enough to move the region
        if let Some(hole_start) = layout_lock.find_smallest_adequate_hole(new_reserved) {
            // dbg!("Move to hole");

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
            region_lock.set_reserved(new_reserved);
            region_lock.set_len(new_len);
            regions.write_to_mmap(&region_lock, region_index);

            drop(layout_lock);

            self.punch_hole(start, reserved)?;

            return Ok(());
        }

        // Write at the end
        // dbg!("Move and write at the end");
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

        // dbg!(new_start, region_index, &region_lock, new_reserved, new_len);

        layout_lock.move_region(new_start, region_index, &region_lock)?;

        region_lock.set_start(new_start);
        region_lock.set_reserved(new_reserved);
        region_lock.set_len(new_len);
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

    pub fn truncate_region(&self, index: usize, from: u64) -> Result<()> {
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

    pub fn remove_region(&self, index: usize) -> Result<Option<Arc<RwLock<Region>>>> {
        let mut regions = self.regions.write();
        let mut layout = self.layout.write();
        let Some(region) = regions.remove_region(index)? else {
            return Ok(None);
        };
        // dbg!(&regions);
        let region_ = region.write();
        layout.remove_region(index, &region_)?;
        // dbg!(layout);
        self.punch_hole(region_.start(), region_.reserved())?;
        drop(region_);
        Ok(Some(region))
    }

    fn create_mmap(file: &fs::File) -> Result<MmapMut> {
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

    pub fn regions(&self) -> RwLockReadGuard<'_, Regions> {
        self.regions.read()
    }

    pub fn layout(&self) -> RwLockReadGuard<'_, Layout> {
        self.layout.read()
    }

    pub fn mmap(&self) -> RwLockReadGuard<'_, MmapMut> {
        self.mmap.read()
    }

    fn ceil_number_to_page_size_multiple(num: u64) -> u64 {
        (num + PAGE_SIZE_MINUS_1) & !PAGE_SIZE_MINUS_1
    }

    fn data_path(&self) -> PathBuf {
        Self::data_path_(&self.path)
    }
    fn data_path_(path: &Path) -> PathBuf {
        path.join("data")
    }

    pub fn flush(&self) -> Result<()> {
        self.file.write().flush().map_err(|e| e.into())
    }

    pub fn sync_data(&self) -> Result<()> {
        self.file.read().sync_data().map_err(|e| e.into())
    }

    pub fn sync_all(&self) -> Result<()> {
        self.file.read().sync_all().map_err(|e| e.into())
    }

    pub fn disk_usage(&self) -> String {
        let path = self.data_path();

        let output = std::process::Command::new("du")
            .arg("-h")
            .arg(&path)
            .output()
            .expect("Failed to run du");

        String::from_utf8_lossy(&output.stdout)
            .replace(path.to_str().unwrap(), " ")
            .trim()
            .to_string()
    }
}

#[repr(C)]
struct FPunchhole {
    fp_flags: u32,
    reserved: u32,
    fp_offset: off_t,
    fp_length: off_t,
}
